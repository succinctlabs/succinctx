// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {FunctionGateway} from "src/FunctionGateway.sol";
import {IFunctionVerifier} from "src/interfaces/IFunctionVerifier.sol";

library TestFuncLib {
    function request(
        address _gateway,
        bytes32 _functionId,
        bytes memory _input,
        bytes4 _callbackSelector,
        bytes memory _context
    ) internal returns (bytes32) {
        return FunctionGateway(_gateway).request{value: msg.value}(_functionId, _input, _callbackSelector, _context);
    }

    function decode(bytes memory _output) internal pure returns (bool) {
        return abi.decode(_output, (bool));
    }
}

contract TestConsumer {
    uint256 public nonce;

    error NotVerified();
    error InvalidRequestNonce(uint256 expectedNonce, uint256 actualNonce);

    function sendRequest(
        address _gateway,
        bytes32 _functionId,
        bytes memory _input,
        bytes4 _callbackSelector,
        bytes memory _context
    ) external payable returns (bytes32) {
        return TestFuncLib.request(_gateway, _functionId, _input, _callbackSelector, _context);
    }

    function handleRequest(bytes memory _output, bytes memory _context) external {
        bool verified = TestFuncLib.decode(_output);
        if (!verified) {
            revert NotVerified();
        }

        uint256 requestNonce = abi.decode(_context, (uint256));
        if (requestNonce != nonce) {
            revert InvalidRequestNonce(nonce, requestNonce);
        }

        ++nonce;
    }
}

// Attempts re-entry into the gateway contract.
contract AttackConsumer {
    uint256 public nonce;

    bytes32 public requestId;
    bytes public output;
    bytes public context;

    error NotVerified();
    error InvalidRequestNonce(uint256 expectedNonce, uint256 actualNonce);

    function sendRequest(
        address _gateway,
        bytes32 _functionId,
        bytes memory _input,
        bytes4 _callbackSelector,
        bytes memory _context
    ) external payable returns (bytes32) {
        return TestFuncLib.request(_gateway, _functionId, _input, _callbackSelector, _context);
    }

    function setCallbackParams(bytes32 _requestId, bytes memory _output, bytes memory _proof) external {
        requestId = _requestId;
        output = _output;
        context = _proof;
    }

    function handleRequest(bytes memory, bytes memory) external {
        FunctionGateway(msg.sender).callback(requestId, output, context);
    }
}

contract TestFunctionVerifier is IFunctionVerifier {
    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256("verificationKeyHash");
    }

    function verify(bytes32, bytes32, bytes memory) external pure returns (bool) {
        return true;
    }
}
