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
import {Proxy} from "src/upgrades/Proxy.sol";
import {SuccinctFeeVault} from "src/payments/SuccinctFeeVault.sol";

contract FunctionGatewayTest is Test, IFunctionGatewayEvents, IFunctionGatewayErrors {
    // Example Function Request and expected values
    string internal constant FUNCTION_NAME = "test-verifier";
    bytes32 internal FUNCTION_ID;
    bytes internal constant REQUEST = bytes("functionInput");
    bytes4 internal constant CALLBACK_SELECTOR = TestConsumer.handleRequest.selector;
    bytes internal constant CALLBACK_CONTEXT = abi.encode(0);
    bytes internal constant REQUEST_OUTPUT = abi.encode(true);
    bytes32 internal constant REQUEST_OUTPUT_HASH = sha256(REQUEST_OUTPUT);
    bytes internal constant REQUEST_PROOF = hex"";
    bytes32 internal EXPECTED_REQUEST_ID;
    uint256 internal constant DEFAULT_FEE = 0.1 ether;
    uint256 internal constant DEFAULT_SCALAR = 1;

    address internal timelock;
    address internal guardian;
    address payable internal sender;
    address internal feeVault;
    address internal gateway;
    address internal verifier;
    address payable internal consumer;
    FunctionRequest internal expectedRequest;

    function setUp() public {
        timelock = makeAddr("timelock");
        guardian = makeAddr("guardian");
        sender = payable(makeAddr("sender"));
        feeVault = address(new SuccinctFeeVault(guardian));
        consumer = payable(address(new TestConsumer()));

        // Deploy FunctionGateway
        address gatewayImpl = address(new FunctionGateway());
        gateway = address(new Proxy(gatewayImpl, ""));
        FunctionGateway(gateway).initialize(DEFAULT_SCALAR, feeVault, timelock, guardian);

        vm.prank(sender);
        (FUNCTION_ID, verifier) =
            IFunctionRegistry(gateway).deployAndRegisterFunction(type(TestFunctionVerifier).creationCode, FUNCTION_NAME);

        expectedRequest = FunctionRequest({
            functionId: FUNCTION_ID,
            inputHash: sha256(REQUEST),
            outputHash: bytes32(0),
            contextHash: keccak256(CALLBACK_CONTEXT),
            callbackAddress: consumer,
            callbackSelector: CALLBACK_SELECTOR,
            proofFulfilled: false,
            callbackFulfilled: false
        });
        EXPECTED_REQUEST_ID = keccak256(abi.encode(FunctionGateway(gateway).nonce(), expectedRequest));

        vm.deal(sender, DEFAULT_FEE);
        vm.deal(consumer, DEFAULT_FEE);
    }

    function test_Request_WhenFromCall() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit ProofRequested(
            prevNonce,
            FUNCTION_ID,
            EXPECTED_REQUEST_ID,
            REQUEST,
            CALLBACK_CONTEXT,
            FunctionGateway(gateway).DEFAULT_GAS_LIMIT(),
            0
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
        assertEq(sha256(REQUEST), inputHash);
        assertEq(bytes32(0), outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(false, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Request_WhenFromContract() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(payable(address(consumer))).sendRequest{value: DEFAULT_FEE}(
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
        assertEq(sha256(REQUEST), inputHash);
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
            prevNonce,
            FUNCTION_ID,
            EXPECTED_REQUEST_ID,
            REQUEST,
            CALLBACK_CONTEXT,
            FunctionGateway(gateway).DEFAULT_GAS_LIMIT(),
            0
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
        assertEq(sha256(REQUEST), inputHash);
        assertEq(bytes32(0), outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(false, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Fulfill_WhenFromContract() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(payable(address(consumer))).sendRequest{value: DEFAULT_FEE}(
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
        assertEq(sha256(REQUEST), inputHash);
        assertEq(REQUEST_OUTPUT_HASH, outputHash);
        assertEq(keccak256(CALLBACK_CONTEXT), contextHash);
        assertEq(address(consumer), callbackAddress);
        assertEq(CALLBACK_SELECTOR, callbackSelector);
        assertEq(true, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_Callback_WhenFromContract() public {
        uint256 prevNonce = FunctionGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(payable(address(consumer))).sendRequest{value: DEFAULT_FEE}(
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
        assertEq(sha256(REQUEST), inputHash);
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
        consumer = payable(address(new AttackConsumer()));

        // Request
        vm.prank(sender);
        bytes32 requestId = TestConsumer(payable(address(consumer))).sendRequest(
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
