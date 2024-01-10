// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {SuccinctGateway} from "src/SuccinctGateway.sol";
import {
    WhitelistStatus,
    ISuccinctGateway,
    ISuccinctGatewayEvents,
    ISuccinctGatewayErrors
} from "src/interfaces/ISuccinctGateway.sol";
import {
    TestConsumer,
    AttackConsumer,
    TestFunctionVerifier1,
    TestFunctionVerifier2
} from "test/TestUtils.sol";
import {
    IFunctionRegistry,
    IFunctionRegistryEvents,
    IFunctionRegistryErrors
} from "src/interfaces/IFunctionRegistry.sol";
import {TestConsumer, TestFunctionVerifier1} from "test/TestUtils.sol";
import {SuccinctFeeVault} from "src/payments/SuccinctFeeVault.sol";

contract SuccinctGatewayTest is Test, ISuccinctGatewayEvents, ISuccinctGatewayErrors {
    // Example Function Request and expected values.
    bytes32 internal constant VERIFIER_SALT = bytes32(uint256(1));
    bytes internal constant INPUT = bytes("function-input");
    bytes32 internal constant INPUT_HASH = sha256(INPUT);
    bytes internal constant OUTPUT = abi.encode(true);
    bytes32 internal constant OUTPUT_HASH = sha256(OUTPUT);
    bytes internal constant PROOF = hex"";

    uint256 internal constant DEFAULT_FEE = 0.1 ether;
    uint32 internal constant DEFAULT_GAS_LIMIT = 1_000_000;

    address internal timelock;
    address internal owner;
    address internal feeVault;
    address internal gateway;
    address internal verifier;
    address payable internal consumer;
    address payable internal sender;
    address internal functionOwner;
    address internal defaultProver;

    function setUp() public virtual {
        // Init variables
        owner = makeAddr("owner");
        sender = payable(makeAddr("sender"));
        functionOwner = makeAddr("function-owner");
        defaultProver = makeAddr("default-prover");

        // Deploy FeeVault
        feeVault = address(new SuccinctFeeVault(owner));

        // Deploy SuccinctGateway
        gateway = address(new SuccinctGateway(owner, feeVault, defaultProver));

        // Deploy Verifier
        bytes32 functionId;
        vm.prank(sender);
        (functionId, verifier) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, VERIFIER_SALT
        );

        // Deploy TestConsumer
        consumer = payable(address(new TestConsumer(gateway, functionId, INPUT)));

        vm.deal(sender, DEFAULT_FEE);
        vm.deal(consumer, DEFAULT_FEE);
    }
}

contract SetupTest is SuccinctGatewayTest {
    function test_SetUp() public {
        assertEq(SuccinctGateway(gateway).owner(), owner);
        assertEq(SuccinctGateway(gateway).feeVault(), feeVault);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), defaultProver), true);

        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        assertTrue(SuccinctGateway(gateway).whitelistStatus(functionId) == WhitelistStatus.Default);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId), verifier);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId), functionOwner);
        assertEq(IFunctionRegistry(gateway).getFunctionId(functionOwner, VERIFIER_SALT), functionId);
    }
}

contract RequestTest is SuccinctGatewayTest {
    function test_Callback() public {
        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
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
        TestConsumer(consumer).requestCallback{value: fee}(callbackGasLimit);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        vm.prank(defaultProver);
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

    function test_Callback_WhenCustomProver() public {
        address customProver = makeAddr("custom-prover");

        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
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
        TestConsumer(consumer).requestCallback{value: fee}(DEFAULT_GAS_LIMIT);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        vm.prank(functionOwner);
        SuccinctGateway(gateway).setWhitelistStatus(functionId, WhitelistStatus.Custom);
        vm.prank(functionOwner);
        SuccinctGateway(gateway).addCustomProver(functionId, customProver);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        vm.prank(customProver);
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

    function test_Callback_WhenDisabledProver() public {
        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
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
        TestConsumer(consumer).requestCallback{value: fee}(callbackGasLimit);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        vm.prank(functionOwner);
        SuccinctGateway(gateway).setWhitelistStatus(functionId, WhitelistStatus.Disabled);

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
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
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
        TestConsumer(consumer).requestCallback{value: fee}(callbackGasLimit);

        assertEq(nonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        vm.prank(defaultProver);
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
        // Set feeVault (first 20 bytes of slot 3) to 0x0
        vm.store(gateway, bytes32(uint256(3)), bytes20(address(0)));

        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
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
        TestConsumer(consumer).requestCallback{value: fee}(callbackGasLimit);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestFulfilled(nonce, functionId, inputHash, OUTPUT_HASH);
        vm.prank(defaultProver);
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

        // Recover ETH
        vm.prank(owner);
        SuccinctGateway(gateway).recover(owner, fee);

        assertEq(owner.balance, fee);
    }

    function test_Callback_WhenCallbackGasLimitTooLow() public {
        uint32 prevNonce = SuccinctGateway(gateway).nonce();
        assertEq(prevNonce, 0);

        uint32 nonce = prevNonce;
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = 0;
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
        TestConsumer(consumer).requestCallback{value: fee}(callbackGasLimit);

        assertEq(prevNonce + 1, SuccinctGateway(gateway).nonce());
        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectRevert();
        vm.prank(defaultProver);
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

    function test_RevertCallback_WhenNoRequest() public {
        uint32 nonce = SuccinctGateway(gateway).nonce();
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
        bytes memory context = abi.encode(nonce);
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;

        // Fulfill
        vm.expectRevert();
        vm.prank(defaultProver);
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

    function test_RevertCallback_WhenNotProver() public {
        uint32 nonce = SuccinctGateway(gateway).nonce();
        bytes32 inputHash = INPUT_HASH;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address callbackAddress = consumer;
        bytes4 callbackSelector = TestConsumer.handleCallback.selector;
        uint32 callbackGasLimit = DEFAULT_GAS_LIMIT;
        uint256 fee = DEFAULT_FEE;
        bytes memory context = abi.encode(nonce);
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;

        // Request
        TestConsumer(consumer).requestCallback{value: fee}(callbackGasLimit);

        // Fulfill
        vm.expectRevert(abi.encodeWithSelector(OnlyProver.selector, functionId, sender));
        vm.prank(sender);
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
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector);
        uint32 callGasLimit = DEFAULT_GAS_LIMIT;
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(callGasLimit);

        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit Call(functionId, INPUT_HASH, OUTPUT_HASH);
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_Call_WhenNoRequest() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector);

        // Fulfill
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );
    }

    function test_Call_WhenNoFee() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector);
        uint32 callGasLimit = DEFAULT_GAS_LIMIT;
        uint256 fee = 0;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(callGasLimit);

        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit Call(functionId, INPUT_HASH, OUTPUT_HASH);
        vm.prank(defaultProver);
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
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector);
        uint32 callGasLimit = DEFAULT_GAS_LIMIT;
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(callGasLimit);

        assertEq(TestConsumer(consumer).handledRequests(0), false);

        // Fulfill
        vm.expectEmit(true, true, true, true, gateway);
        emit Call(functionId, INPUT_HASH, OUTPUT_HASH);
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );

        assertEq(TestConsumer(consumer).handledRequests(0), true);
    }

    function test_RevertCall_WhenNotProver() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        address callAddress = consumer;
        uint32 callGasLimit = DEFAULT_GAS_LIMIT;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector);
        uint256 fee = DEFAULT_FEE;

        // Request
        TestConsumer(consumer).requestCall{value: fee}(callGasLimit);

        // Fulfill
        vm.expectRevert(abi.encodeWithSelector(OnlyProver.selector, functionId, sender));
        vm.prank(sender);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );
    }

    function test_VerifiedCall() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes32 inputHash = INPUT_HASH;

        // Set the SuccinctGateway's storage slots to avoid revert:
        // verifiedFunctionId | bytes32 | 5
        // verifiedInputHash  | bytes32 | 6
        // verifiedOutput     | bytes   | 7
        vm.store(gateway, bytes32(uint256(5)), functionId);
        vm.store(gateway, bytes32(uint256(6)), inputHash);

        // Verifiy call
        TestConsumer(consumer).verifiedCall();
    }

    function test_RevertVerifiedCall_WhenNotSet() public {
        bytes memory input = INPUT;
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();

        // Verify call
        vm.expectRevert(abi.encodeWithSelector(InvalidCall.selector, functionId, input));
        TestConsumer(consumer).verifiedCall();
    }
}

contract AttackSuccinctGatewayTest is SuccinctGatewayTest {
    address payable internal attackConsumer;

    function setUp() public override {
        super.setUp();

        // Deploy Verifier
        bytes32 functionId;
        vm.prank(sender);
        (functionId, verifier) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "attack-verifier"
        );

        // Deploy AttackConsumer
        attackConsumer = payable(address(new AttackConsumer(gateway, functionId, INPUT)));

        vm.deal(attackConsumer, DEFAULT_FEE);
    }

    function test_RevertCallbackReenterCallback() public {
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        bytes32 functionId = AttackConsumer(attackConsumer).FUNCTION_ID();
        bytes32 inputHash = INPUT_HASH;
        address callbackAddress = attackConsumer;
        bytes4 callbackSelector = AttackConsumer.handleCallbackReenterCallback.selector;
        uint32 callbackGasLimit = AttackConsumer(attackConsumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.prank(sender);
        AttackConsumer(attackConsumer).requestCallbackReenterCallback{value: fee}();

        // Fulfill (test fails this doesn't revert with ReentrantFulfill() error)
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCallback(
            0,
            functionId,
            inputHash,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            "",
            output,
            proof
        );
    }

    function test_RevertCallbackReenterCall() public {
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        bytes32 functionId = AttackConsumer(attackConsumer).FUNCTION_ID();
        bytes32 inputHash = INPUT_HASH;
        address callbackAddress = attackConsumer;
        bytes4 callbackSelector = AttackConsumer.handleCallbackReenterCall.selector;
        uint32 callbackGasLimit = AttackConsumer(attackConsumer).CALLBACK_GAS_LIMIT();
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.prank(sender);
        AttackConsumer(attackConsumer).requestCallbackReenterCall{value: fee}();

        // Fulfill (test fails this doesn't revert with ReentrantFulfill() error)
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCallback(
            0,
            functionId,
            inputHash,
            callbackAddress,
            callbackSelector,
            callbackGasLimit,
            "",
            output,
            proof
        );
    }

    function test_RevertCallReenterCallback() public {
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        bytes32 functionId = AttackConsumer(attackConsumer).FUNCTION_ID();
        address callAddress = attackConsumer;
        bytes memory callData =
            abi.encodeWithSelector(AttackConsumer.handleCallReenterCallback.selector);
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.prank(sender);
        AttackConsumer(attackConsumer).requestCallReenterCallback{value: fee}();

        // Fulfill (test fails this doesn't revert with ReentrantFulfill() error)
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );
    }

    function test_RevertCallReenterCall() public {
        bytes memory input = INPUT;
        bytes memory output = OUTPUT;
        bytes memory proof = PROOF;
        bytes32 functionId = AttackConsumer(attackConsumer).FUNCTION_ID();
        address callAddress = attackConsumer;
        bytes memory callData =
            abi.encodeWithSelector(AttackConsumer.handleCallReenterCall.selector);
        uint256 fee = DEFAULT_FEE;

        // Request
        vm.prank(sender);
        AttackConsumer(attackConsumer).requestCallReenterCall{value: fee}();

        // Fulfill (test fails this doesn't revert with ReentrantFulfill() error)
        vm.prank(defaultProver);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );
    }
}

contract FunctionRegistryTest is
    SuccinctGatewayTest,
    IFunctionRegistryEvents,
    IFunctionRegistryErrors
{
    function test_RegisterFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");

        // Deploy verifier
        address verifier1;
        bytes memory bytecode = type(TestFunctionVerifier1).creationCode;
        bytes32 salt = expectedFunctionId1;
        assembly {
            verifier1 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Register function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionRegistered(expectedFunctionId1, verifier1, "test-verifier1", functionOwner);
        bytes32 functionId1 =
            IFunctionRegistry(gateway).registerFunction(functionOwner, verifier1, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(expectedFunctionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(expectedFunctionId1), functionOwner);
    }

    function test_RegisterFunction_WhenOwnerIsSender() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");

        // Deploy verifier
        address verifier1;
        bytes memory bytecode = type(TestFunctionVerifier1).creationCode;
        bytes32 salt = expectedFunctionId1;
        assembly {
            verifier1 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Register function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionRegistered(expectedFunctionId1, verifier1, "test-verifier1", functionOwner);
        vm.prank(functionOwner);
        bytes32 functionId1 =
            IFunctionRegistry(gateway).registerFunction(functionOwner, verifier1, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(expectedFunctionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(expectedFunctionId1), functionOwner);
    }

    function test_RevertRegisterFunction_WhenAlreadyRegistered() public {
        // Deploy verifier
        address verifier1;
        bytes memory bytecode = type(TestFunctionVerifier1).creationCode;
        bytes32 salt = IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");
        assembly {
            verifier1 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Register function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionRegistered(salt, verifier1, "test-verifier1", functionOwner);
        IFunctionRegistry(gateway).registerFunction(functionOwner, verifier1, "test-verifier1");

        // Register function again
        vm.expectRevert(abi.encodeWithSelector(FunctionAlreadyRegistered.selector, salt));
        IFunctionRegistry(gateway).registerFunction(functionOwner, verifier1, "test-verifier1");
    }

    function test_DeployAndRegisterFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");

        // Deploy verifier and register function
        vm.expectEmit(true, false, false, true, gateway);
        emit Deployed(
            keccak256(type(TestFunctionVerifier1).creationCode), expectedFunctionId1, address(0)
        );
        vm.expectEmit(true, true, true, false, gateway);
        emit FunctionRegistered(expectedFunctionId1, address(0), "test-verifier1", functionOwner);
        (bytes32 functionId1, address verifier1) = IFunctionRegistry(gateway)
            .deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), functionOwner);
    }

    function test_DeployAndRegisterFunction_WhenOwnerIsSender() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");

        // Deploy verifier and register function
        vm.expectEmit(true, false, false, true, gateway);
        emit Deployed(
            keccak256(type(TestFunctionVerifier1).creationCode), expectedFunctionId1, address(0)
        );
        vm.expectEmit(true, true, true, false, gateway);
        emit FunctionRegistered(expectedFunctionId1, address(0), "test-verifier1", functionOwner);
        vm.prank(functionOwner);
        (bytes32 functionId1, address verifier1) = IFunctionRegistry(gateway)
            .deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), functionOwner);
    }

    function test_RevertDeployAndRegisterFunction_WhenAlreadyRegistered() public {
        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and register function again
        // Note: Deployment fails instead of hitting "FunctionAlreadyRegistered" because
        // a contract is already deployed to that address.
        vm.expectRevert(FailedDeploy.selector);
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );
    }

    function test_UpdateFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");

        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier
        address verifier2;
        bytes memory bytecode = type(TestFunctionVerifier2).creationCode;
        bytes32 salt = expectedFunctionId1;
        assembly {
            verifier2 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Update function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionVerifierUpdated(expectedFunctionId1, verifier2);
        vm.prank(functionOwner);
        bytes32 functionId1 = IFunctionRegistry(gateway).updateFunction(verifier2, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier2);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), functionOwner);
    }

    function test_RevertUpdateFunction_WhenNotOwner() public {
        // Deploy verifier and register function
        (bytes32 functionId,) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier
        address verifier2;
        bytes memory bytecode = type(TestFunctionVerifier2).creationCode;
        bytes32 salt = functionId;
        assembly {
            verifier2 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Update function
        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(NotFunctionOwner.selector, sender, address(0)));
        IFunctionRegistry(gateway).updateFunction(verifier2, "test-verifier1");
    }

    function test_RevertUpdateFunction_WhenNeverRegistered() public {
        // Deploy verifier
        address verifier2;
        bytes memory bytecode = type(TestFunctionVerifier2).creationCode;
        bytes32 salt = bytes32(0);
        assembly {
            verifier2 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Update function
        vm.expectRevert(
            abi.encodeWithSelector(NotFunctionOwner.selector, functionOwner, address(0))
        );
        vm.prank(functionOwner);
        IFunctionRegistry(gateway).updateFunction(verifier2, "test-verifier1");
    }

    function test_RevertUpdateFunction_WhenVerifierSame() public {
        // Deploy verifier and register function
        (bytes32 functionId1, address verifier1) = IFunctionRegistry(gateway)
            .deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Update function
        vm.expectRevert(abi.encodeWithSelector(VerifierAlreadyUpdated.selector, functionId1));
        vm.prank(functionOwner);
        IFunctionRegistry(gateway).updateFunction(verifier1, "test-verifier1");
    }

    function test_deployAndUpdateFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(functionOwner, "test-verifier1");

        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and update function
        vm.expectEmit(true, false, false, true, gateway);
        emit Deployed(
            keccak256(type(TestFunctionVerifier2).creationCode), expectedFunctionId1, address(0)
        );
        vm.expectEmit(true, true, true, false, gateway);
        emit FunctionVerifierUpdated(expectedFunctionId1, address(0));
        vm.prank(functionOwner);
        (bytes32 functionId1, address verifier2) = IFunctionRegistry(gateway)
            .deployAndUpdateFunction(type(TestFunctionVerifier2).creationCode, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier2);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), functionOwner);
    }

    function test_RevertDeployAndUpdateFunction_WhenNotOwner() public {
        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and update function
        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(NotFunctionOwner.selector, sender, address(0)));
        IFunctionRegistry(gateway).deployAndUpdateFunction(
            type(TestFunctionVerifier2).creationCode, "test-verifier1"
        );
    }

    function test_RevertDeployAndUpdateFunction_WhenNeverRegistered() public {
        // Deploy verifier and update function
        vm.expectRevert(
            abi.encodeWithSelector(NotFunctionOwner.selector, functionOwner, address(0))
        );
        vm.prank(functionOwner);
        IFunctionRegistry(gateway).deployAndUpdateFunction(
            type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );
    }

    function test_RevertDeployAndUpdateFunction_WhenBytecodeSame() public {
        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            functionOwner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and update function
        vm.expectRevert(abi.encodeWithSelector(FailedDeploy.selector));
        vm.prank(functionOwner);
        IFunctionRegistry(gateway).deployAndUpdateFunction(
            type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );
    }
}

contract UpdateProverTest is SuccinctGatewayTest {
    function test_AddDefaultProver() public {
        address newDefaultProver = makeAddr("default-prover");

        vm.expectEmit(true, true, true, true, gateway);
        emit ProverUpdated(bytes32(0), newDefaultProver, true);
        vm.prank(owner);
        SuccinctGateway(gateway).addDefaultProver(newDefaultProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), defaultProver), true);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), newDefaultProver), true);
    }

    function test_RevertAddDefaultProver_WhenNotGuardian() public {
        address newDefaultProver = makeAddr("new-default-prover");

        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(sender);
        SuccinctGateway(gateway).addDefaultProver(newDefaultProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), defaultProver), true);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), newDefaultProver), false);
    }

    function test_RemoveDefaultProver() public {
        emit ProverUpdated(bytes32(0), defaultProver, true);
        vm.prank(owner);
        SuccinctGateway(gateway).removeDefaultProver(defaultProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), defaultProver), false);
    }

    function test_RevertRemoveDefaultProver_WhenNotGuardian() public {
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(sender);
        SuccinctGateway(gateway).removeDefaultProver(defaultProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(bytes32(0), defaultProver), true);
    }

    function test_AddCustomProver() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address customProver = makeAddr("custom-prover");

        vm.expectEmit(true, true, true, true, gateway);
        emit ProverUpdated(functionId, customProver, true);
        vm.prank(functionOwner);
        SuccinctGateway(gateway).addCustomProver(functionId, customProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(functionId, customProver), true);
    }

    function test_RevertAddCustomProver_WhenNotOwner() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        address customProver = makeAddr("custom-prover");

        vm.expectRevert(
            abi.encodeWithSignature("NotFunctionOwner(address,address)", sender, functionOwner)
        );
        vm.prank(sender);
        SuccinctGateway(gateway).addCustomProver(functionId, customProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(functionId, customProver), false);
    }

    function test_RemoveCustomProver() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();

        emit ProverUpdated(functionId, defaultProver, true);
        vm.prank(functionOwner);
        SuccinctGateway(gateway).removeCustomProver(functionId, defaultProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(functionId, defaultProver), false);
    }

    function test_RevertRemoveCustomProver_WhenNotOwner() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();

        vm.expectRevert(
            abi.encodeWithSignature("NotFunctionOwner(address,address)", sender, functionOwner)
        );
        vm.prank(sender);
        SuccinctGateway(gateway).removeCustomProver(functionId, defaultProver);
    }
}

contract SetWhitelistStatusTest is SuccinctGatewayTest {
    function test_SetWhitelistStatus() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        WhitelistStatus status = WhitelistStatus.Custom;

        vm.expectEmit(true, true, true, true, gateway);
        emit WhitelistStatusUpdated(functionId, status);
        vm.prank(functionOwner);
        SuccinctGateway(gateway).setWhitelistStatus(functionId, status);
        assertTrue(SuccinctGateway(gateway).whitelistStatus(functionId) == status);
    }

    function test_RevertSetWhitelistStatus_WhenNotOwner() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        WhitelistStatus status = WhitelistStatus.Custom;

        vm.expectRevert(
            abi.encodeWithSignature("NotFunctionOwner(address,address)", sender, functionOwner)
        );
        vm.prank(sender);
        SuccinctGateway(gateway).setWhitelistStatus(functionId, status);
        assertTrue(SuccinctGateway(gateway).whitelistStatus(functionId) == WhitelistStatus.Default);
    }
}

contract SetFeeVaultTest is SuccinctGatewayTest {
    function test_SetFeeVault() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes memory input = INPUT;
        address callAddress = consumer;
        bytes memory callData = abi.encodeWithSelector(TestConsumer.handleCall.selector);
        uint32 callGasLimit = DEFAULT_GAS_LIMIT;
        uint256 fee = DEFAULT_FEE;
        address newFeeVault = address(new SuccinctFeeVault(functionOwner));

        // Set FeeVault
        vm.expectEmit(true, true, true, true, gateway);
        emit SetFeeVault(SuccinctGateway(gateway).feeVault(), newFeeVault);
        vm.prank(owner);
        SuccinctGateway(gateway).setFeeVault(newFeeVault);

        assertEq(SuccinctGateway(gateway).feeVault(), newFeeVault);

        // Request with fee
        vm.expectEmit(true, true, true, true, gateway);
        emit RequestCall(functionId, input, callAddress, callData, callGasLimit, consumer, fee);
        TestConsumer(consumer).requestCall{value: fee}(callGasLimit);
    }

    function test_RevertSetFeeVault_WhenNotGuardian() public {
        address newFeeVault = address(new SuccinctFeeVault(functionOwner));

        // Set FeeVault
        vm.expectRevert();
        SuccinctGateway(gateway).setFeeVault(newFeeVault);
    }
}

contract RecoverTest is SuccinctGatewayTest {
    function test_Recover() public {
        uint256 fee = DEFAULT_FEE;
        vm.deal(gateway, fee);

        // Recover ETH
        vm.prank(owner);
        SuccinctGateway(gateway).recover(owner, fee);

        assertEq(owner.balance, fee);
    }

    function test_RevertRecover_WhenNotGuardian() public {
        uint256 fee = DEFAULT_FEE;
        vm.deal(gateway, fee);

        // Recover ETH
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(sender);
        SuccinctGateway(gateway).recover(owner, fee);
    }
}
