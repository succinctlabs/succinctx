// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionGateway, FunctionRequest} from "../interfaces/IFunctionGateway.sol";

contract MockFunctionGateway is IFunctionGateway {
    uint256 public DEFAULT_GAS_LIMIT = 1000000;
    uint256 public nonce;
    mapping(bytes32 => FunctionRequest) public requests;
    uint256 scalar = 1;

    function setRequest(bytes32 _requestId, FunctionRequest memory _request) public {
        requests[_requestId] = _request;
    }

    function request(
        bytes32 _functionId,
        bytes memory _input,
        bytes4 _callbackSelector,
        bytes memory _context
    ) external payable returns (bytes32) {
        return
            request(_functionId, _input, _callbackSelector, _context, DEFAULT_GAS_LIMIT, tx.origin);
    }

    function request(
        bytes32 _functionId,
        bytes memory _input,
        bytes4 _callbackSelector,
        bytes memory _context,
        uint256 _gasLimit,
        address
    ) public payable returns (bytes32) {
        bytes32 inputHash = sha256(_input);
        bytes32 contextHash = keccak256(_context);
        FunctionRequest memory r = FunctionRequest({
            functionId: _functionId,
            inputHash: inputHash,
            outputHash: bytes32(0),
            contextHash: contextHash,
            callbackAddress: msg.sender,
            callbackSelector: _callbackSelector,
            proofFulfilled: false,
            callbackFulfilled: false
        });

        bytes32 requestId = keccak256(abi.encode(nonce, r));
        requests[requestId] = r;

        emit ProofRequested(
            nonce, requestId, _input, _context, _gasLimit, calculateFeeAmount(_gasLimit)
        );
        nonce++;
        return requestId;
    }

    function fulfill(bytes32 _requestId, bytes32 _outputHash, bytes memory _proof) external {
        FunctionRequest storage r = requests[_requestId];
        r.proofFulfilled = true;
        r.outputHash = _outputHash;

        emit ProofFulfilled(_requestId, _outputHash, _proof);
    }

    function fulfillBatch(
        bytes32[] memory _requestIds,
        bytes memory _aggregateProof,
        bytes32 _inputsRoot,
        bytes32[] memory _outputHashes,
        bytes32 _outputsRoot,
        bytes32 _verificationKeyRoot
    ) external {
        for (uint256 i = 0; i < _requestIds.length; i++) {
            bytes32 requestId = _requestIds[i];
            FunctionRequest storage r = requests[requestId];
            r.proofFulfilled = true;
            r.outputHash = _outputHashes[i];
        }

        emit ProofBatchFulfilled(
            _requestIds,
            _aggregateProof,
            _inputsRoot,
            _outputHashes,
            _outputsRoot,
            _verificationKeyRoot
        );
    }

    function callback(bytes32 _requestId, bytes memory _output, bytes memory _context) external {
        FunctionRequest storage r = requests[_requestId];
        r.callbackFulfilled = true;

        (bool status,) =
            r.callbackAddress.call(abi.encodeWithSelector(r.callbackSelector, _output, _context));
        if (!status) {
            revert CallbackFailed(r.callbackAddress, r.callbackSelector);
        }

        emit CallbackFulfilled(_requestId, _output, _context);
    }

    function calculateFeeAmount() external view returns (uint256 feeAmount) {
        return calculateFeeAmount(DEFAULT_GAS_LIMIT);
    }

    function calculateFeeAmount(uint256 _gasLimit) public view returns (uint256 feeAmount) {
        if (scalar == 0) {
            feeAmount = tx.gasprice * _gasLimit;
        } else {
            feeAmount = tx.gasprice * _gasLimit * scalar;
        }
    }

    function setScalar(uint256 _scalar) external {
        scalar = _scalar;

        emit ScalarUpdated(scalar);
    }
}
