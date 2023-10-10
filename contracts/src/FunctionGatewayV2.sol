// SPDX-License-Identifier: MIT

pragma solidity ^0.8.16;

import {IFunctionGateway, FunctionRequest} from "./interfaces/IFunctionGateway.sol";
import {IFunctionVerifier} from "./interfaces/IFunctionVerifier.sol";
import {FunctionRegistry} from "./FunctionRegistry.sol";
import {TimelockedUpgradeable} from "./upgrades/TimelockedUpgradeable.sol";

contract FunctionGateway is
    IFunctionGateway,
    FunctionRegistry,
    TimelockedUpgradeable
{
    /// @dev The default gas limit for requests.
    uint256 public constant DEFAULT_GAS_LIMIT = 1000000;

    /// @dev A nonce for keeping track of requests.
    uint32 public nonce;

    /// @dev A mapping from request nonces to request hashes.
    mapping(uint32 => bytes32) public requests;

    /// @dev The current function identifier.
    bytes32 public currentVerifiedFunctionId;

    /// @dev The current function input hash.
    bytes32 public currentVerifiedInputHash;

    /// @dev The current function output.
    bytes public currentVerifiedOutput;

    /// @dev Initializes the contract.
    /// @param _timelock The address of the timelock contract.
    /// @param _guardian The address of the guardian.
    function initialize(
        address _timelock,
        address _guardian
    ) external initializer {
        __TimelockedUpgradeable_init(_timelock, _guardian);
    }

    /// @dev Creates a onchain request for a proof. The output and proof is fulfilled asynchronously
    ///      by the provided callback.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    /// @param _context The function context.
    /// @param _callbackSelector The selector of the callback function.
    function request(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector
    ) external payable returns (bytes32) {
        return
            request(
                _functionId,
                _input,
                _context,
                _callbackSelector,
                DEFAULT_GAS_LIMIT
            );
    }

    /// @dev Creates a onchain request for a proof. The output and proof is fulfilled asynchronously
    ///      by the provided callback.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    /// @param _context The function context.
    /// @param _callbackSelector The selector of the callback function.
    /// @param _callbackGasLimit The gas limit for the callback function.
    function zkRequest(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) external payable returns (bytes32) {
        // Compute the callback hash uniquely associated with this request.
        bytes32 inputHash = sha256(_input);
        bytes32 contextHash = sha256(_context);
        address callbackAddress = msg.sender;
        bytes32 requestHash = _requestHash(
            nonce,
            _functionId,
            inputHash,
            contextHash,
            callbackAddress,
            _callbackSelector,
            _callbackGasLimit
        );

        // Store the callback hash.
        requests[nonce] = requestHash;
        emit Request(
            nonce,
            _functionId,
            _input,
            _context,
            _callbackSelector,
            _callbackGasLimit
        );

        // Increment the nonce.
        nonce++;
        return callbackHash;
    }

    /// @dev Fulfills a request by providing the output and proof.
    /// @param _nonce The nonce of the request.
    /// @param _functionId The function identifier.
    /// @param _inputHash The hash of the function input.
    /// @param _callbackAddress The address of the callback contract.
    /// @param _callbackSelector The selector of the callback function.
    /// @param _callbackGasLimit The gas limit for the callback function.
    /// @param _context The function context.
    /// @param _output The function output.
    /// @param _proof The function proof.
    function zkRequestFulfill(
        uint32 _nonce,
        bytes32 _functionId,
        bytes32 _inputHash,
        address _callbackAddress,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit,
        bytes memory _context,
        bytes memory _output,
        bytes memory _proof
    ) external {
        // Reconstruct the callback hash.
        bytes32 contextHash = sha256(_context);
        bytes32 requestHash = _requestHash(
            _nonce,
            _functionId,
            _inputHash,
            contextHash,
            _callbackAddress,
            _callbackSelector,
            _callbackGasLimit
        );

        // Assert that the callback hash is unfilfilled.
        if (requests[_nonce] != requestHash) {
            revert InvalidFulfill(_nonce, requests[_nonce], requestHash);
        }

        // Compute the output hash.
        bytes32 outputHash = sha256(_output);

        // Verify the proof.
        _verify(_functionId, _inputHash, outputHash, _proof);

        // Execute the callback.
        (bool status, ) = _callbackAddress.call(
            abi.encodeWithSelector(_callbackSelector, _output, _context)
        );

        // If the callback failed, revert.
        require(status, "FunctionGateway: callback failed");

        // Delete the callback hash for a gas refund.
        delete requests[_nonce];
    }

    /// @dev If the call matches the currently verified function, returns the output. Otherwise,
    ///      this function reverts.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    function zkCall(
        bytes32 _functionId,
        bytes memory _input
    ) external returns (bytes memory) {
        if (
            currentVerifiedFunctionId == _functionId &&
            currentVerifiedInputHash == sha256(_input)
        ) {
            return currentVerifiedCall.output;
        }
        revert("FunctionGateway: not verified");
    }

    /// @dev The entrypoint for fulfilling a call.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    /// @param _output The function output.
    /// @param _proof The function proof.
    /// @param _callbackAddress The address of the callback contract.
    /// @param _callbackData The data for the callback function.
    function zkCallFulfill(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _output,
        bytes memory _proof,
        address _callbackAddress,
        bytes memory _callbackData
    ) external {
        // Compute the input and output hashes.
        bytes32 inputHash = sha256(_input);
        bytes32 outputHash = sha256(_output);

        // Verify the proof.
        _verify(_functionId, inputHash, outputHash, _proof);

        // Set the current verified call.
        currentVerifiedFunctionId = _functionId;
        currentVerifiedInputHash = inputHash;
        currentVerifiedOutput = _output;

        // Execute the callback.
        (bool status, ) = _callbackAddress.call(_callbackData);
        if (!status) {
            revert("FunctionGateway: callback failed");
        }

        // Delete the current verified call.
        delete currentVerifiedFunctionId;
        delete currentVerifiedInputHash;
        delete currentVerifiedOutput;
    }

    /// @dev Computes a unique identifier for a request.
    /// @param _functionId The function identifier.
    /// @param _inputHash The hash of the function input.
    /// @param _contextHash The hash of the function context.
    /// @param _callbackAddress The address of the callback contract.
    /// @param _callbackSelector The selector of the callback function.
    /// @param _callbackGasLimit The gas limit for the callback function.
    function _requestHash(
        uint32 _nonce,
        bytes32 _functionId,
        bytes32 _inputHash,
        bytes32 _contextHash,
        address _callbackAddress,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) internal returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(
                    _nonce,
                    _functionId,
                    _inputHash,
                    _contextHash,
                    _callbackAddress,
                    _callbackSelector,
                    _callbackGasLimit
                )
            );
    }

    /// @dev Verifies a proof with respect to a function identifier, input hash, and output hash.
    /// @param _functionId The function identifier.
    /// @param _inputHash The hash of the function input.
    /// @param _outputHash The hash of the function output.
    /// @param _proof The function proof.
    function _verify(
        bytes32 _functionId,
        bytes32 _inputHash,
        bytes32 _outputHash,
        bytes memory _proof
    ) internal {
        address verifier = verifiers[_functionId];
        if (
            !IFunctionVerifier(verifier).verify(_inputHash, _outputHash, _proof)
        ) {
            revert InvalidProof(
                address(verifier),
                r.inputHash,
                _outputHash,
                _proof
            );
        }
    }

    /// @dev This empty reserved space to add new variables without shifting down storage.
    ///      See: https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
    uint256[50] private __gap;

    // TODO: fix
    event Request(
        uint32 nonce,
        bytes32 functionId,
        bytes input,
        bytes context,
        bytes4 callbackSelector,
        uint256 callbackGasLimit
    );

    error InvalidCallback(
        uint256 nonce,
        bytes32 expectedCallbackHash,
        bytes32 callbackHash
    );
}
