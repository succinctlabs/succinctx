// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {Test} from "forge-std/Test.sol";
import {IFunctionVerifier} from "src/interfaces/IFunctionVerifier.sol";
import {ISuccinctGateway} from "src/interfaces/ISuccinctGateway.sol";

interface ISuccinctGatewayWithFulfill is ISuccinctGateway {
    function fulfillCallback(
        uint32 nonce,
        bytes32 functionId,
        bytes32 inputHash,
        address callbackAddress,
        bytes4 callbackSelector,
        uint32 callbackGasLimit,
        bytes memory context,
        bytes memory output,
        bytes memory proof
    ) external;

    function fulfillCall(
        bytes32 functionId,
        bytes memory input,
        bytes memory output,
        bytes memory proof,
        address callbackAddress,
        bytes memory callbackData
    ) external;
}

contract TestConsumer {
    address public immutable SUCCINCT_GATEWAY;
    bytes32 public immutable FUNCTION_ID;
    bytes public INPUT;

    uint32 public nonce;
    mapping(uint32 => bool) public handledRequests;

    error NotValid();
    error InvalidRequestNonce(uint32 expectedNonce, uint32 actualNonce);
    error ResultNotTrue();

    constructor(address _gateway, bytes32 _functionId, bytes memory _input) payable {
        SUCCINCT_GATEWAY = _gateway;
        FUNCTION_ID = _functionId;
        INPUT = _input;
    }

    function requestCallback(uint32 _callbackGasLimit) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, INPUT, abi.encode(nonce), this.handleCallback.selector, _callbackGasLimit
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

    function requestCall(uint32 _callGasLimit) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID,
            INPUT,
            address(this),
            abi.encodeWithSelector(this.handleCall.selector),
            _callGasLimit
        );
    }

    function handleCall() external {
        if (msg.sender != SUCCINCT_GATEWAY) {
            revert NotValid();
        }

        bytes memory output = ISuccinctGateway(SUCCINCT_GATEWAY).verifiedCall(FUNCTION_ID, INPUT);

        bool result = abi.decode(output, (bool));
        if (!result) {
            revert ResultNotTrue();
        }

        handledRequests[nonce++] = result;
    }

    function verifiedCall() public view {
        ISuccinctGateway(SUCCINCT_GATEWAY).verifiedCall(FUNCTION_ID, INPUT);
    }
}

// Attempts re-enter into the gateway contract. Errors are directly checked because expectRevert only
// checks the last call, which would be CallbackFailed() if in test.
contract AttackConsumer is Test {
    address public immutable SUCCINCT_GATEWAY;
    bytes32 public immutable FUNCTION_ID;
    bytes public INPUT;
    uint32 public constant CALLBACK_GAS_LIMIT = 2000000;

    constructor(address _gateway, bytes32 _functionId, bytes memory _input) payable {
        SUCCINCT_GATEWAY = _gateway;
        FUNCTION_ID = _functionId;
        INPUT = _input;
    }

    function requestCallbackReenterCallback() external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, INPUT, "", this.handleCallbackReenterCallback.selector, CALLBACK_GAS_LIMIT
        );
    }

    function requestCallbackReenterCall() external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, INPUT, "", this.handleCallbackReenterCall.selector, CALLBACK_GAS_LIMIT
        );
    }

    function requestCallReenterCallback() external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID,
            INPUT,
            address(this),
            abi.encodeWithSelector(this.handleCallReenterCallback.selector),
            CALLBACK_GAS_LIMIT
        );
    }

    function requestCallReenterCall() external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID,
            INPUT,
            address(this),
            abi.encodeWithSelector(this.handleCallReenterCall.selector),
            CALLBACK_GAS_LIMIT
        );
    }

    function handleCallbackReenterCallback(bytes memory _output, bytes memory) external {
        vm.expectRevert(abi.encodeWithSignature("ReentrantFulfill()"));
        ISuccinctGatewayWithFulfill(SUCCINCT_GATEWAY).fulfillCallback(
            0,
            FUNCTION_ID,
            "",
            address(this),
            this.handleCallbackReenterCallback.selector,
            0,
            "",
            _output,
            ""
        );
    }

    function handleCallbackReenterCall(bytes memory _output, bytes memory) external {
        vm.expectRevert(abi.encodeWithSignature("ReentrantFulfill()"));
        ISuccinctGatewayWithFulfill(SUCCINCT_GATEWAY).fulfillCall(
            FUNCTION_ID, "", _output, "", address(this), ""
        );
    }

    function handleCallReenterCallback() external {
        vm.expectRevert(abi.encodeWithSignature("ReentrantFulfill()"));
        ISuccinctGatewayWithFulfill(SUCCINCT_GATEWAY).fulfillCallback(
            0,
            FUNCTION_ID,
            "",
            address(this),
            this.handleCallbackReenterCallback.selector,
            0,
            "",
            "",
            ""
        );
    }

    function handleCallReenterCall() external {
        vm.expectRevert(abi.encodeWithSignature("ReentrantFulfill()"));
        ISuccinctGatewayWithFulfill(SUCCINCT_GATEWAY).fulfillCall(
            FUNCTION_ID, "", "", "", address(this), ""
        );
    }
}

contract TestFunctionVerifier1 is IFunctionVerifier {
    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256("verificationKeyHash1");
    }

    function verify(bytes32, bytes32, bytes memory) external pure returns (bool) {
        return true;
    }
}

contract TestFunctionVerifier2 is IFunctionVerifier {
    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256("verificationKeyHash2");
    }

    function verify(bytes32, bytes32, bytes memory) external pure returns (bool) {
        return true;
    }
}
