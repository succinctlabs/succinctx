// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {ISuccinctGateway, WhitelistStatus} from "./interfaces/ISuccinctGateway.sol";
import {IFunctionVerifier} from "./interfaces/IFunctionVerifier.sol";
import {FunctionRegistry} from "./FunctionRegistry.sol";
import {IFeeVault} from "./payments/interfaces/IFeeVault.sol";
import {Initializable} from "@openzeppelin-upgradeable/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgradeable/contracts/access/OwnableUpgradeable.sol";

contract SuccinctGateway is
    ISuccinctGateway,
    FunctionRegistry,
    Initializable,
    OwnableUpgradeable
{
    /// @notice The address of the fee vault.
    address public feeVault;

    /// @notice A nonce for keeping track of requests.
    uint32 public nonce;

    /// @notice A mapping from request nonces to request hashes.
    mapping(uint32 => bytes32) public requests;

    /// @notice The currently verified function identifier.
    bytes32 public verifiedFunctionId;

    /// @notice The currently verified function input hash.
    bytes32 public verifiedInputHash;

    /// @notice The currently verified function output.
    bytes public verifiedOutput;

    /// @notice A flag that indicates whether the contract is currently making a callback.
    bool public override isCallback;

    mapping(bytes32 => WhitelistStatus) public whitelistStatus;

    /// @notice The allowed provers that can fulfill requests.
    mapping(bytes32 => mapping(address => bool)) public allowedProvers;

    /// @dev Protects functions from being re-entered during a fullfil call.
    modifier nonReentrant() {
        if (
            isCallback || verifiedFunctionId != bytes32(0) || verifiedInputHash != bytes32(0)
                || verifiedOutput.length != 0
        ) {
            revert ReentrantFulfill();
        }
        _;
    }

    /// @dev Protects functions from being called by anyone other than the prover.
    modifier onlyProver(bytes32 _functionId) {
        if (
            whitelistStatus[_functionId] == WhitelistStatus.Default
                && !allowedProvers[bytes32(0)][msg.sender]
        ) {
            revert OnlyProver(_functionId, msg.sender);
        } else if (
            whitelistStatus[_functionId] == WhitelistStatus.Custom
                && !allowedProvers[_functionId][msg.sender]
        ) {
            revert OnlyProver(_functionId, msg.sender);
        }
        _;
    }

    /// @notice Initializes the contract. Only callable once, and only callable by deployer.
    /// @param _owner The address of the owner of the contract.
    /// @param _feeVault The address of the fee vault.
    /// @param _defaultProver The address of the default prover.
    function initialize(address _owner, address _feeVault, address _defaultProver)
        external
        initializer
    {
        _transferOwnership(_owner);
        feeVault = _feeVault;
        allowedProvers[bytes32(0)][_defaultProver] = true;
    }

    /// @notice Creates a onchain request for a proof. The output and proof is fulfilled asynchronously
    ///         by the provided callback.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    /// @param _context The function context.
    /// @param _callbackSelector The selector of the callback function.
    /// @param _callbackGasLimit The gas limit for the callback function.
    function requestCallback(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) external payable override returns (bytes32) {
        // Compute the callback hash uniquely associated with this request.
        bytes32 inputHash = sha256(_input);
        bytes32 contextHash = keccak256(_context);
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
        emit RequestCallback(
            nonce,
            _functionId,
            _input,
            _context,
            callbackAddress,
            _callbackSelector,
            _callbackGasLimit,
            msg.value
        );

        // Increment the nonce.
        nonce++;

        // Send the fee to the vault.
        if (feeVault != address(0)) {
            IFeeVault(feeVault).depositNative{value: msg.value}(callbackAddress);
        }

        return requestHash;
    }

    /// @notice Creates a proof request for a call. This function is equivalent to an off-chain request
    ///         through an API.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    /// @param _entryAddress The address of the callback contract.
    /// @param _entryCalldata The entry calldata for the call.
    /// @param _entryGasLimit The gas limit for the call.
    function requestCall(
        bytes32 _functionId,
        bytes memory _input,
        address _entryAddress,
        bytes memory _entryCalldata,
        uint32 _entryGasLimit
    ) external payable override {
        // Emit event.
        emit RequestCall(
            _functionId,
            _input,
            _entryAddress,
            _entryCalldata,
            _entryGasLimit,
            msg.sender,
            msg.value
        );

        // Send the fee to the vault.
        if (feeVault != address(0)) {
            IFeeVault(feeVault).depositNative{value: msg.value}(msg.sender);
        }
    }

    /// @notice If the call matches the currently verified function, returns the output. Otherwise,
    ///         this function reverts.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    function verifiedCall(bytes32 _functionId, bytes memory _input)
        external
        view
        override
        returns (bytes memory)
    {
        bytes32 inputHash = sha256(_input);
        if (verifiedFunctionId == _functionId && verifiedInputHash == inputHash) {
            return verifiedOutput;
        } else {
            revert InvalidCall(_functionId, _input);
        }
    }

    /// @notice Fulfills a request by providing the output and proof.
    /// @param _nonce The nonce of the request.
    /// @param _functionId The function identifier.
    /// @param _inputHash The hash of the function input.
    /// @param _callbackAddress The address of the callback contract.
    /// @param _callbackSelector The selector of the callback function.
    /// @param _callbackGasLimit The gas limit for the callback function.
    /// @param _context The function context.
    /// @param _output The function output.
    /// @param _proof The function proof.
    function fulfillCallback(
        uint32 _nonce,
        bytes32 _functionId,
        bytes32 _inputHash,
        address _callbackAddress,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit,
        bytes memory _context,
        bytes memory _output,
        bytes memory _proof
    ) external nonReentrant onlyProver(_functionId) {
        // Reconstruct the callback hash.
        bytes32 contextHash = keccak256(_context);
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
            revert InvalidRequest(_nonce, requests[_nonce], requestHash);
        }

        // Delete the callback hash for a gas refund.
        delete requests[_nonce];

        // Compute the output hash.
        bytes32 outputHash = sha256(_output);

        // Verify the proof.
        _verify(_functionId, _inputHash, outputHash, _proof);

        // Execute the callback.
        isCallback = true;
        (bool status,) = _callbackAddress.call{gas: _callbackGasLimit}(
            abi.encodeWithSelector(_callbackSelector, _output, _context)
        );
        isCallback = false;

        // If the callback failed, revert.
        if (!status) {
            revert CallbackFailed(_callbackSelector, _output, _context);
        }

        // Emit event.
        emit RequestFulfilled(_nonce, _functionId, _inputHash, outputHash);
    }

    /// @notice The entrypoint for fulfilling a call.
    /// @param _functionId The function identifier.
    /// @param _input The function input.
    /// @param _output The function output.
    /// @param _proof The function proof.
    /// @param _callbackAddress The address of the callback contract.
    /// @param _callbackData The data for the callback function.
    function fulfillCall(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _output,
        bytes memory _proof,
        address _callbackAddress,
        bytes memory _callbackData
    ) external nonReentrant onlyProver(_functionId) {
        // Compute the input and output hashes.
        bytes32 inputHash = sha256(_input);
        bytes32 outputHash = sha256(_output);

        // Verify the proof.
        _verify(_functionId, inputHash, outputHash, _proof);

        // Set the current verified call.
        verifiedFunctionId = _functionId;
        verifiedInputHash = inputHash;
        verifiedOutput = _output;

        // Execute the callback.
        (bool status,) = _callbackAddress.call(_callbackData);
        if (!status) {
            revert CallFailed(_callbackAddress, _callbackData);
        }

        // Delete the current verified call.
        delete verifiedFunctionId;
        delete verifiedInputHash;
        delete verifiedOutput;

        // Emit event.
        emit Call(_functionId, inputHash, outputHash);
    }

    /// @notice Sets the whitelist status for a function.
    /// @param _functionId The function identifier.
    /// @param _status The whitelist status to set.
    function setWhitelistStatus(bytes32 _functionId, WhitelistStatus _status) external {
        if (msg.sender != verifierOwners[_functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[_functionId]);
        }
        whitelistStatus[_functionId] = _status;
        emit WhitelistStatusUpdated(_functionId, _status);
    }

    /// @notice Add a custom prover.
    /// @param _functionId The function identifier.
    /// @param _prover The address of the prover to add.
    function addCustomProver(bytes32 _functionId, address _prover) external {
        if (msg.sender != verifierOwners[_functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[_functionId]);
        }
        allowedProvers[_functionId][_prover] = true;
        emit ProverUpdated(_functionId, _prover, true);
    }

    /// @notice Remove a custom prover.
    /// @param _functionId The function identifier.
    /// @param _prover The address of the prover to remove.
    function removeCustomProver(bytes32 _functionId, address _prover) external {
        if (msg.sender != verifierOwners[_functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[_functionId]);
        }
        delete allowedProvers[_functionId][_prover];
        emit ProverUpdated(_functionId, _prover, false);
    }

    /// @notice Add a default prover.
    /// @param _prover The address of the prover to add.
    function addDefaultProver(address _prover) external onlyOwner {
        allowedProvers[bytes32(0)][_prover] = true;
        emit ProverUpdated(bytes32(0), _prover, true);
    }

    /// @notice Remove a default prover.
    /// @param _prover The address of the prover to remove.
    function removeDefaultProver(address _prover) external onlyOwner {
        delete allowedProvers[bytes32(0)][_prover];
        emit ProverUpdated(bytes32(0), _prover, false);
    }

    /// @notice Sets the fee vault to a new address. Can be set to address(0) to disable fees.
    /// @param _feeVault The address of the fee vault.
    function setFeeVault(address _feeVault) external onlyOwner {
        emit SetFeeVault(feeVault, _feeVault);
        feeVault = _feeVault;
    }

    /// @notice Recovers stuck ETH from the contract.
    /// @param _to The address to send the ETH to.
    /// @param _amount The wei amount of ETH to send.
    function recover(address _to, uint256 _amount) external onlyOwner {
        (bool success,) = _to.call{value: _amount}("");
        if (!success) {
            revert RecoverFailed();
        }
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
    ) internal pure returns (bytes32) {
        return keccak256(
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
        if (!IFunctionVerifier(verifier).verify(_inputHash, _outputHash, _proof)) {
            revert InvalidProof(address(verifier), _inputHash, _outputHash, _proof);
        }
    }
}
