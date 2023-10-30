// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/Test.sol";
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
    uint32 public constant CALLBACK_GAS_LIMIT = 2000000;

    uint32 public nonce;
    bytes callInput;
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

    function requestCall(bytes memory _input) external payable {
        callInput = _input;

        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID,
            _input,
            address(this),
            abi.encodeWithSelector(this.handleCall.selector),
            CALLBACK_GAS_LIMIT
        );

        nonce++;
    }

    function handleCall() external {
        if (msg.sender != SUCCINCT_GATEWAY) {
            revert NotValid();
        }

        bytes memory output =
            ISuccinctGateway(SUCCINCT_GATEWAY).verifiedCall(FUNCTION_ID, callInput);
        callInput = "";

        bool result = abi.decode(output, (bool));
        if (!result) {
            revert ResultNotTrue();
        }

        handledRequests[nonce - 1] = result;
    }

    function verifiedCall(bytes memory _input) public view {
        ISuccinctGateway(SUCCINCT_GATEWAY).verifiedCall(FUNCTION_ID, _input);
    }
}

// Attempts re-enter into the gateway contract. Errors are directly checked because expectRevert only
// checks the last call, which would be CallbackFailed() if in test.
contract AttackConsumer is Test {
    address public immutable SUCCINCT_GATEWAY;
    bytes32 public immutable FUNCTION_ID;
    uint32 public constant CALLBACK_GAS_LIMIT = 2000000;

    constructor(address _gateway, bytes32 _functionId) payable {
        SUCCINCT_GATEWAY = _gateway;
        FUNCTION_ID = _functionId;
    }

    function requestCallbackReenterCallback(bytes memory _input) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, _input, "", this.handleCallbackReenterCallback.selector, CALLBACK_GAS_LIMIT
        );
    }

    function requestCallbackReenterCall(bytes memory _input) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, _input, "", this.handleCallbackReenterCall.selector, CALLBACK_GAS_LIMIT
        );
    }

    function requestCallReenterCallback(bytes memory _input) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID,
            _input,
            address(this),
            abi.encodeWithSelector(this.handleCallReenterCallback.selector),
            CALLBACK_GAS_LIMIT
        );
    }

    function requestCallReenterCall(bytes memory _input) external payable {
        ISuccinctGateway(SUCCINCT_GATEWAY).requestCall{value: msg.value}(
            FUNCTION_ID,
            _input,
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

contract TestFunctionVerifier is IFunctionVerifier {
    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256("verificationKeyHash");
    }

    function verify(bytes32, bytes32, bytes memory) external pure returns (bool) {
        return true;
    }
}
