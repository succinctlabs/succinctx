// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {FunctionGateway} from "src/FunctionGateway.sol";
import {
    FunctionRequest,
    IFunctionGateway,
    IFunctionGatewayEvents,
    IFunctionGatewayErrors
} from "src/interfaces/IFunctionGateway.sol";
import {IFunctionRegistry} from "src/interfaces/IFunctionRegistry.sol";
import {TestConsumer, AttackConsumer, TestFunctionVerifier} from "test/TestUtils.sol";
import {SuccinctFeeVault} from "@telepathy-v2/payment/SuccinctFeeVault.sol";

contract FunctionGatewayTest is Test, IFunctionGatewayEvents, IFunctionGatewayErrors {
    // Example Function Request and expected values
    bytes32 internal constant FUNCTION_ID = keccak256("functionId");
    bytes internal constant REQUEST = bytes("functionInput");
    bytes4 internal constant CALLBACK_SELECTOR = TestConsumer.handleRequest.selector;
    bytes internal constant CALLBACK_CONTEXT = abi.encode(0);
    bytes internal constant REQUEST_OUTPUT = abi.encode(true);
    bytes32 internal constant REQUEST_OUTPUT_HASH = keccak256(REQUEST_OUTPUT);
    bytes internal constant REQUEST_PROOF = hex"";
    bytes32 internal EXPECTED_REQUEST_ID;
    uint256 internal constant DEFAULT_FEE = 0.1 ether;
    uint256 internal constant DEFAULT_GAS_LIMIT = 1000000;
    uint256 internal constant DEFAULT_SCALAR = 1;
    string internal constant VERIFIER_NAME = "test-verifier";

    address internal owner;
    address internal sender;
    address internal feeVault;
    address internal gateway;
    address internal verifier;
    address internal consumer;
    FunctionRequest internal expectedRequest;

    function setUp() public {
        owner = makeAddr("owner");
        sender = makeAddr("sender");
        feeVault = address(new SuccinctFeeVault(owner));
        gateway = address(new FunctionGateway(DEFAULT_SCALAR, feeVault, owner));
        consumer = address(new TestConsumer());
        expectedRequest = FunctionRequest({
            functionId: FUNCTION_ID,
            inputHash: keccak256(REQUEST),
            outputHash: bytes32(0),
            contextHash: keccak256(CALLBACK_CONTEXT),
            callbackAddress: consumer,
            callbackSelector: CALLBACK_SELECTOR,
            proofFulfilled: false,
            callbackFulfilled: false
        });
        EXPECTED_REQUEST_ID = keccak256(abi.encode(FunctionGateway(gateway).nonce(), expectedRequest));

        vm.prank(owner);
        verifier = IFunctionRegistry(gateway).registerFunction(type(TestFunctionVerifier).creationCode, VERIFIER_NAME);
        console.log("Verifier address: %s", verifier);

        vm.deal(sender, DEFAULT_FEE);
        vm.deal(consumer, DEFAULT_FEE);
    }

    function test_Request() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit ProofRequested(
            prevNonce, FUNCTION_ID, EXPECTED_REQUEST_ID, REQUEST, CALLBACK_CONTEXT, DEFAULT_GAS_LIMIT, 0
        );
        vm.prank(consumer);
        bytes32 requestId = FunctionGateway(gateway).request{value: DEFAULT_FEE}(
            FUNCTION_ID, REQUEST, CALLBACK_SELECTOR, CALLBACK_CONTEXT
        );
        assertEq(EXPECTED_REQUEST_ID, requestId);

        (
            bytes32 functionId,
            bytes32 inputHash,
            bytes32 outputHash,
            bytes32 contextHash,
            address callbackAddress,
            bytes4 callbackSelector,
            bool proofFulfilled,
            bool callbackFulfilled
        ) = FunctionGateway(gateway).requests(requestId);
        assertEq(prevNonce + 1, FunctionGateway(gateway).nonce());
        assertEq(FUNCTION_ID, functionId);
        assertEq(keccak256(REQUEST), inputHash);
        assertEq(bytes32(0), outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(false, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Request_WhenNoFee() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit ProofRequested(
            prevNonce, FUNCTION_ID, EXPECTED_REQUEST_ID, REQUEST, CALLBACK_CONTEXT, DEFAULT_GAS_LIMIT, 0
        );
        vm.prank(consumer);
        bytes32 requestId = FunctionGateway(gateway).request(FUNCTION_ID, REQUEST, CALLBACK_SELECTOR, CALLBACK_CONTEXT);
        assertEq(EXPECTED_REQUEST_ID, requestId);

        (
            bytes32 functionId,
            bytes32 inputHash,
            bytes32 outputHash,
            bytes32 contextHash,
            address callbackAddress,
            bytes4 callbackSelector,
            bool proofFulfilled,
            bool callbackFulfilled
        ) = FunctionGateway(gateway).requests(requestId);
        assertEq(prevNonce + 1, FunctionGateway(gateway).nonce());
        assertEq(FUNCTION_ID, functionId);
        assertEq(keccak256(REQUEST), inputHash);
        assertEq(bytes32(0), outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(false, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Request_WhenFromConsumer() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(address(consumer)).sendRequest{value: DEFAULT_FEE}(
            address(gateway), FUNCTION_ID, REQUEST, CALLBACK_SELECTOR, CALLBACK_CONTEXT
        );
        assertEq(EXPECTED_REQUEST_ID, requestId);

        (
            bytes32 functionId,
            bytes32 inputHash,
            bytes32 outputHash,
            bytes32 contextHash,
            address callbackAddress,
            bytes4 callbackSelector,
            bool proofFulfilled,
            bool callbackFulfilled
        ) = FunctionGateway(gateway).requests(requestId);
        assertEq(prevNonce + 1, FunctionGateway(gateway).nonce());
        assertEq(FUNCTION_ID, functionId);
        assertEq(keccak256(REQUEST), inputHash);
        assertEq(bytes32(0), outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(false, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Fulfill_WhenFromConsumer() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(address(consumer)).sendRequest{value: DEFAULT_FEE}(
            address(gateway), FUNCTION_ID, REQUEST, CALLBACK_SELECTOR, CALLBACK_CONTEXT
        );

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit ProofFulfilled(requestId, REQUEST_OUTPUT_HASH, REQUEST_PROOF);
        FunctionGateway(gateway).fulfill(requestId, REQUEST_OUTPUT_HASH, REQUEST_PROOF);

        (
            bytes32 functionId,
            bytes32 inputHash,
            bytes32 outputHash,
            bytes32 contextHash,
            address callbackAddress,
            bytes4 callbackSelector,
            bool proofFulfilled,
            bool callbackFulfilled
        ) = FunctionGateway(gateway).requests(requestId);
        assertEq(prevNonce + 1, FunctionGateway(gateway).nonce());
        assertEq(FUNCTION_ID, functionId);
        assertEq(keccak256(REQUEST), inputHash);
        assertEq(REQUEST_OUTPUT_HASH, outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(true, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Callback_WhenFromConsumer() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(address(consumer)).sendRequest{value: DEFAULT_FEE}(
            address(gateway), FUNCTION_ID, REQUEST, CALLBACK_SELECTOR, CALLBACK_CONTEXT
        );

        // Fulfill
        FunctionGateway(gateway).fulfill(requestId, REQUEST_OUTPUT_HASH, REQUEST_PROOF);

        // Callback
        vm.expectEmit(true, true, true, true, gateway);
        emit CallbackFulfilled(requestId, REQUEST_OUTPUT, CALLBACK_CONTEXT);
        FunctionGateway(gateway).callback(requestId, REQUEST_OUTPUT, CALLBACK_CONTEXT);
        (
            bytes32 functionId,
            bytes32 inputHash,
            bytes32 outputHash,
            bytes32 contextHash,
            address callbackAddress,
            bytes4 callbackSelector,
            bool proofFulfilled,
            bool callbackFulfilled
        ) = FunctionGateway(gateway).requests(requestId);
        assertEq(prevNonce + 1, FunctionGateway(gateway).nonce());
        assertEq(FUNCTION_ID, functionId);
        assertEq(keccak256(REQUEST), inputHash);
        assertEq(REQUEST_OUTPUT_HASH, outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(true, proofFulfilled);
        assertEq(true, callbackFulfilled);
    }

    function test_RevertCallback_WhenReentered() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Setup attack consumer
        consumer = address(new AttackConsumer());

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(address(consumer)).sendRequest(
            address(gateway), FUNCTION_ID, REQUEST, CALLBACK_SELECTOR, CALLBACK_CONTEXT
        );

        AttackConsumer(consumer).setCallbackParams(requestId, REQUEST_OUTPUT, CALLBACK_CONTEXT);

        // Fulfill
        FunctionGateway(gateway).fulfill(requestId, REQUEST_OUTPUT_HASH, REQUEST_PROOF);

        // Callback
        // inner error: vm.expectRevert(abi.encodeWithSelector(CallbackAlreadyFulfilled.selector, requestId));
        vm.expectRevert(
            abi.encodeWithSelector(CallbackFailed.selector, consumer, AttackConsumer.handleRequest.selector)
        );
        FunctionGateway(gateway).callback(requestId, REQUEST_OUTPUT, CALLBACK_CONTEXT);
    }
}
