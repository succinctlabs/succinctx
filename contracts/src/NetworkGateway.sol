// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {IFeeVault} from "./payments/interfaces/IFeeVault.sol";
import {INetworkVerifier} from "./interfaces/INetworkVerifier.sol";
import {NetworkStorage} from "./NetworkStorage.sol";
import {NetworkAdmin} from "./NetworkAdmin.sol";
import {NetworkRegistry} from "./NetworkRegistry.sol";

interface INetworkGatewayEvents {
    event RequestCallback(
        uint32 indexed nonce,
        address indexed verifier,
        bytes32 programHash,
        bytes input,
        bytes context,
        address callback,
        bytes4 callbackSelector,
        uint32 callbackGasLimit,
        uint256 value
    );
    event RequestCall(
        address indexed verifier,
        bytes32 programHash,
        bytes input,
        address callback,
        bytes callbackCalldata,
        uint32 callbackGasLimit,
        address caller,
        uint256 value
    );
    event FulfilledCallback(
        uint32 indexed nonce,
        address indexed verifier,
        bytes32 programHash,
        address callback,
        address prover,
        bytes32 inputHash,
        bytes32 outputHash
    );
    event FullfilledCall(
        address indexed verifier,
        bytes32 programHash,
        address callback,
        address prover,
        bytes32 inputHash,
        bytes32 outputHash
    );
}

interface INetworkGatewayErrors {
    error ReentrantFulfill();
    error InvalidRequest(uint32 nonce, bytes32 expected, bytes32 actual);
    error InvalidProof();
    error CallbackFailed(bytes4 callbackSelector, bytes output, bytes context);
    error CallFailed(address callback, bytes callbackData);
    error InvalidCall(address verifier, bytes input);
}

interface INetworkGateway is INetworkGatewayEvents, INetworkGatewayErrors {
    function requestCallback(
        address verifier,
        bytes32 programHash,
        bytes calldata input,
        bytes calldata context,
        bytes4 callbackSelector,
        uint32 callbackGasLimit
    ) external payable returns (bytes32);

    function requestCall(
        address verifier,
        bytes32 programHash,
        bytes calldata input,
        address callback,
        bytes calldata callbackCalldata,
        uint32 callbackGasLimit
    ) external payable;

    function isCallback() external view returns (bool);

    function verifiedCall(address verifier, bytes calldata input)
        external
        view
        returns (bytes memory);

    function fulfillCallback(
        uint32 nonce,
        address verifier,
        bytes32 programHash,
        bytes32 inputHash,
        address callback,
        bytes4 callbackSelector,
        uint32 callbackGasLimit,
        bytes calldata context,
        bytes calldata output,
        bytes calldata proof
    ) external;

    function fulfillCall(
        address verifier,
        bytes calldata input,
        bytes calldata output,
        bytes calldata proof,
        address callback,
        bytes calldata callbackData
    ) external;
}

contract NetworkGateway is INetworkGateway, NetworkStorage, NetworkAdmin, NetworkRegistry {
    modifier nonReentrant() {
        if (
            executingCallback || verifiedVerifier != address(0) || verifiedInputHash != bytes32(0)
                || verifiedOutput.length != 0
        ) {
            revert ReentrantFulfill();
        }
        _;
    }

    function initialize(address _owner, address _feeVault, address _defaultProver)
        external
        initializer
    {
        _transferOwnership(_owner);
        feeVault = _feeVault;
        allowedProvers[address(0)][_defaultProver] = true;
    }

    function requestCallback(
        address _verifier,
        bytes32 _programHash,
        bytes calldata _input,
        bytes calldata _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) external payable override returns (bytes32) {
        bytes32 inputHash = sha256(_input);
        bytes32 contextHash = keccak256(_context);
        address callback = msg.sender;
        bytes32 requestHash = _requestHash(
            nonce,
            _verifier,
            _programHash,
            inputHash,
            contextHash,
            callback,
            _callbackSelector,
            _callbackGasLimit
        );

        requests[nonce] = requestHash;
        emit RequestCallback(
            nonce,
            _verifier,
            _programHash,
            _input,
            _context,
            callback,
            _callbackSelector,
            _callbackGasLimit,
            msg.value
        );

        nonce++;

        if (msg.value > 0 && feeVault != address(0)) {
            IFeeVault(feeVault).depositNative{value: msg.value}(callback);
        }

        return requestHash;
    }

    function requestCall(
        address _verifier,
        bytes32 _programHash,
        bytes calldata _input,
        address _callback,
        bytes calldata _callbackCalldata,
        uint32 _callbackGasLimit
    ) external payable override {
        emit RequestCall(
            _verifier,
            _programHash,
            _input,
            _callback,
            _callbackCalldata,
            _callbackGasLimit,
            msg.sender,
            msg.value
        );

        if (msg.value > 0 && feeVault != address(0)) {
            IFeeVault(feeVault).depositNative{value: msg.value}(msg.sender);
        }
    }

    function isCallback() external view override returns (bool) {
        return executingCallback;
    }

    function verifiedCall(address _verifier, bytes calldata _input)
        external
        view
        override
        returns (bytes memory)
    {
        bytes32 inputHash = sha256(_input);
        if (verifiedVerifier == _verifier && verifiedInputHash == inputHash) {
            return verifiedOutput;
        } else {
            revert InvalidCall(_verifier, _input);
        }
    }

    function fulfillCallback(
        uint32 _nonce,
        address _verifier,
        bytes32 _programHash,
        bytes32 _inputHash,
        address _callback,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit,
        bytes calldata _context,
        bytes calldata _output,
        bytes calldata _proof
    ) external nonReentrant onlyProver(_verifier) {
        bytes32 contextHash = keccak256(_context);
        bytes32 requestHash = _requestHash(
            _nonce,
            _verifier,
            _programHash,
            _inputHash,
            contextHash,
            _callback,
            _callbackSelector,
            _callbackGasLimit
        );

        if (requests[_nonce] != requestHash) {
            revert InvalidRequest(_nonce, requests[_nonce], requestHash);
        }
        delete requests[_nonce];

        bytes32 outputHash = sha256(_output);

        _verify(_verifier, _programHash, _inputHash, outputHash, _proof);

        executingCallback = true;
        (bool status,) = _callback.call{gas: _callbackGasLimit}(
            abi.encodeWithSelector(_callbackSelector, _output, _context)
        );
        delete executingCallback;

        if (!status) {
            revert CallbackFailed(_callbackSelector, _output, _context);
        }

        emit FulfilledCallback(
            _nonce, _verifier, _programHash, _callback, msg.sender, _inputHash, outputHash
        );
    }

    function fulfillCall(
        address _verifier,
        bytes32 _programHash,
        bytes calldata _input,
        bytes calldata _output,
        bytes calldata _proof,
        address _callback,
        bytes calldata _callbackData
    ) external nonReentrant onlyProver(_verifier) {
        bytes32 inputHash = sha256(_input);
        bytes32 outputHash = sha256(_output);

        _verify(_verifier, _programHash, inputHash, outputHash, _proof);

        verifiedVerifier = _verifier;
        verifiedInputHash = inputHash;
        verifiedOutput = _output;
        (bool status,) = _callback.call(_callbackData);
        if (!status) {
            revert CallFailed(_callback, _callbackData);
        }
        delete verifiedVerifier;
        delete verifiedInputHash;
        delete verifiedOutput;

        emit FullfilledCall(_verifier, _programHash, _callback, msg.sender, inputHash, outputHash);
    }

    function _requestHash(
        uint32 _nonce,
        address _verifier,
        bytes32 _programHash,
        bytes32 _inputHash,
        bytes32 _contextHash,
        address _callback,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encodePacked(
                _nonce,
                _verifier,
                _programHash,
                _inputHash,
                _contextHash,
                _callback,
                _callbackSelector,
                _callbackGasLimit
            )
        );
    }

    function _verify(
        address _verifier,
        bytes32 _programHash,
        bytes32 _inputHash,
        bytes32 _outputHash,
        bytes calldata _proof
    ) internal {
        if (!INetworkVerifier(_verifier).verify(_programHash, _inputHash, _outputHash, _proof)) {
            revert InvalidProof();
        }
    }
}
