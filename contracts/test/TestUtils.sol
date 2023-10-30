// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionVerifier} from "src/interfaces/IFunctionVerifier.sol";
import {ISuccinctGateway} from "src/interfaces/ISuccinctGateway.sol";

contract TestConsumer {
    address public immutable SUCCINCT_GATEWAY;
    bytes32 public immutable FUNCTION_ID;
    uint32 public constant CALLBACK_GAS_LIMIT = 2000000;

    uint32 public nonce;
    mapping(uint32 => bool) public handledRequests;

    error NotValid();
    error InvalidRequestNonce(uint32 expectedNonce, uint32 actualNonce);
    error ResultNotTrue();

    constructor(address _gateway, bytes32 _functionId) payable {
        SUCCINCT_GATEWAY = _gateway;
        FUNCTION_ID = _functionId;
    }

    function requestCallback(bytes memory _input) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, _input, abi.encode(nonce), this.handleCallback.selector, CALLBACK_GAS_LIMIT
        );

        nonce++;
    }

    function handleCallback(bytes memory _output, bytes memory _context) external {
        if (msg.sender != SUCCINCT_GATEWAY || !ISuccinctGateway(SUCCINCT_GATEWAY).isCallback()) {
            revert NotValid();
        }
        if (abi.decode(_context, (uint32)) != nonce - 1) {
            revert InvalidRequestNonce(nonce, abi.decode(_context, (uint32)));
        }

        bool result = abi.decode(_output, (bool));
        if (!result) {
            revert ResultNotTrue();
        }

        handledRequests[nonce - 1] = result;
    }

    function requestCall(bytes memory _input, bytes memory callData) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID, _input, address(this), callData, CALLBACK_GAS_LIMIT
        );

        nonce++;
    }

    function handleCall(bytes memory _output) external {
        if (msg.sender != SUCCINCT_GATEWAY) {
            revert NotValid();
        }

        bool result = abi.decode(_output, (bool));
        if (!result) {
            revert ResultNotTrue();
        }

        handledRequests[nonce - 1] = result;
    }

    function verifiedCall(bytes memory _input) public view {
        ISuccinctGateway(SUCCINCT_GATEWAY).verifiedCall(FUNCTION_ID, _input);
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
