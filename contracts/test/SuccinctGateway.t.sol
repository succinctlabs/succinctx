// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {SuccinctGateway} from "src/SuccinctGateway.sol";
import {
    ISuccinctGateway,
    ISuccinctGatewayEvents,
    ISuccinctGatewayErrors
} from "src/interfaces/ISuccinctGateway.sol";
import {IFunctionRegistry} from "src/interfaces/IFunctionRegistry.sol";
import {TestConsumer, TestFunctionVerifier} from "test/TestUtils.sol";
import {Proxy} from "src/upgrades/Proxy.sol";
import {SuccinctFeeVault} from "src/payments/SuccinctFeeVault.sol";

contract SuccinctGatewayTest is Test, ISuccinctGatewayEvents, ISuccinctGatewayErrors {
    // Example Function Request and expected values.
    bytes internal constant INPUT = bytes("function-input");
    bytes32 internal constant INPUT_HASH = sha256(INPUT);
    bytes internal constant OUTPUT = abi.encode(true);
    bytes32 internal constant OUTPUT_HASH = sha256(OUTPUT);
    bytes internal constant PROOF = hex"";

    uint256 internal constant DEFAULT_FEE = 0.1 ether;

    address internal timelock;
    address internal guardian;
    address internal feeVault;
    address internal gateway;
    address internal verifier;
    address payable internal consumer;
    address payable internal sender;
    address internal owner;

    function setUp() public {
        // Init variables
        timelock = makeAddr("timelock");
        guardian = makeAddr("guardian");
        sender = payable(makeAddr("sender"));
        owner = makeAddr("owner");

        // Deploy FeeVault
        address feeVaultImpl = address(new SuccinctFeeVault());
        feeVault = address(new Proxy(feeVaultImpl, ""));
        SuccinctFeeVault(feeVault).initialize(timelock, guardian);

        // Deploy SuccinctGateway
        address gatewayImpl = address(new SuccinctGateway());
        gateway = address(new Proxy(gatewayImpl, ""));
        SuccinctGateway(gateway).initialize(feeVault, timelock, guardian);

        bytes32 functionId;
        vm.prank(sender);
        (functionId, verifier) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier).creationCode, "test-verifier"
        );

        // Deploy TestConsumer
        consumer = payable(address(new TestConsumer(gateway, functionId)));

        vm.deal(sender, DEFAULT_FEE);
        vm.deal(consumer, DEFAULT_FEE);
    }

    function test_Callback() public {
        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;
        bytes memory context = abi.encode(nonce);
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCallback(
            nonce,
            functionId,
            INPUT,
            context,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            fee
        );
        vm.prank(sender);
        TestConsumer(consumer).requestCallback{value: fee}(INPUT);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        SuccinctGateway(gateway).fulfillCallback(
            nonce,
            functionId,
            inputHash,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            context,
            output,
            proof
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_Callback_WhenNoFee() public {
        uint32 nonce = SuccinctGateway(gateway).nonce();
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = 0;
        bytes memory context = abi.encode(nonce);
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCallback(
            nonce,
            functionId,
            INPUT,
            context,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            fee
        );
        vm.prank(sender);
        TestConsumer(consumer).requestCallback{value: fee}(INPUT);

        assertEq(nonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        SuccinctGateway(gateway).fulfillCallback(
            nonce,
            functionId,
            inputHash,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            context,
            output,
            proof
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_Callback_WhenNoFeeVault() public {
        // Set feeVault (first 20 bytes of slot 253) to 0x0
        vm.store(gateway, bytes32(uint256(253)), bytes20(address(0)));

        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;
        bytes memory context = abi.encode(nonce);
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCallback(
            nonce,
            functionId,
            INPUT,
            context,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            fee
        );
        vm.prank(sender);
        TestConsumer(consumer).requestCallback{value: fee}(INPUT);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        SuccinctGateway(gateway).fulfillCallback(
            nonce,
            functionId,
            inputHash,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            context,
            output,
            proof
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_RevertCallback() public {
        uint32 nonce = SuccinctGateway(gateway).nonce();
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        bytes memory context = abi.encode(nonce);
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;

        // Fulfill
        vm.expectRevert();
        SuccinctGateway(gateway).fulfillCallback(
            nonce,
            functionId,
            inputHash,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            context,
            output,
            proof
        );
    }

    function test_Call() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector, OUTPUT, 0);
        uint32 callGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(input, callData);

        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit Call(functionId, INPUT_HASH, OUTPUT_HASH);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_Call_WhenNoFee() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector, OUTPUT, 0);
        uint32 callGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = 0;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(input, callData);

        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit Call(functionId, INPUT_HASH, OUTPUT_HASH);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_Call_WhenNoFeeVault() public {
        // Set feeVault (first 20 bytes of slot 253) to 0x0
        vm.store(gateway, bytes32(uint256(253)), bytes20(address(0)));

        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector, OUTPUT, 0);
        uint32 callGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(input, callData);

        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit Call(functionId, INPUT_HASH, OUTPUT_HASH);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_RevertCall() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector, OUTPUT, 0);

        // Fulfill
        vm.expectRevert();
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );
    }

    function test_VerifiedCall() public {
        bytes memory input = INPUT;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes32 inputHash = INPUT_HASH;

        // Set the SuccinctGateway's storage slots to avoid revert:
        // | verifiedFunctionId | bytes32 | 255
        // | verifiedInputHash  | bytes32 | 256
        // | verifiedOutput     | bytes   | 257
        vm.store(gateway, bytes32(uint256(255)), functionId);
        vm.store(gateway, bytes32(uint256(256)), inputHash);

        // Verifiy call
        TestConsumer(consumer).verifiedCall(input);
    }

    function test_RevertVerifiedCall_WhenNotSet() public {
        bytes memory input = INPUT;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();

        // Verifiy call
        vm.expectRevert(abi.encodeWithSelector(InvalidCall.selector, functionId, input));
        TestConsumer(consumer).verifiedCall(input);
    }

    function test_SetFeeVault() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector, OUTPUT, 0);
        uint32 callGasLimit = TestConsumer(consumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;
        address newFeeVault = address(new SuccinctFeeVault());

        // Set FeeVault
        vm.expectEmit(true, true, true, true, gateway);
        emit SetFeeVault(SuccinctGateway(gateway).feeVault(), newFeeVault);
        vm.prank(guardian);
        SuccinctGateway(gateway).setFeeVault(newFeeVault);

        assertEq(SuccinctGateway(gateway).feeVault(), newFeeVault);

        // Request with fee
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(input, callData);
    }

    function test_RevertSetFeeVault_WhenNotGuardian() public {
        address newFeeVault = address(new SuccinctFeeVault());

        // Set FeeVault
        vm.expectRevert();
        SuccinctGateway(gateway).setFeeVault(newFeeVault);
    }
}
