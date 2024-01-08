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
import {Proxy} from "src/upgrades/Proxy.sol";
import {SuccinctFeeVault} from "src/payments/SuccinctFeeVault.sol";
import {AccessControlUpgradeable} from
    "@openzeppelin-upgradeable/contracts/access/AccessControlUpgradeable.sol";

contract SuccinctGatewayTest is Test, ISuccinctGatewayEvents, ISuccinctGatewayErrors {
    // Example Function Request and expected values.
    bytes internal constant INPUT = bytes("function-input");
    bytes32 internal constant INPUT_HASH = sha256(INPUT);
    bytes internal constant OUTPUT = abi.encode(true);
    bytes32 internal constant OUTPUT_HASH = sha256(OUTPUT);
    bytes internal constant PROOF = hex"";

    uint256 internal constant DEFAULT_FEE = 0.1 ether;
    uint32 internal constant DEFAULT_GAS_LIMIT = 1_000_000;

    address internal timelock;
    address internal guardian;
    address internal feeVault;
    address internal gateway;
    address internal verifier;
    address payable internal consumer;
    address payable internal sender;
    address internal owner;
    address internal prover;

    function setUp() public virtual {
        // Init variables
        timelock = makeAddr("timelock");
        guardian = makeAddr("guardian");
        sender = payable(makeAddr("sender"));
        owner = makeAddr("owner");
        prover = makeAddr("prover");

        // Deploy FeeVault
        address feeVaultImpl = address(new SuccinctFeeVault());
        feeVault = address(new Proxy(feeVaultImpl, ""));
        SuccinctFeeVault(feeVault).initialize(timelock, guardian);

        // Deploy SuccinctGateway
        address gatewayImpl = address(new SuccinctGateway());
        gateway = address(new Proxy(gatewayImpl, ""));
        SuccinctGateway(gateway).initialize(feeVault, timelock, guardian);

        // Add prover
        vm.prank(guardian);
        SuccinctGateway(gateway).addProver(prover);

        // Deploy Verifier
        bytes32 functionId;
        vm.prank(sender);
        (functionId, verifier) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier"
        );

        // Deploy TestConsumer
        consumer = payable(address(new TestConsumer(gateway, functionId, INPUT)));

        vm.deal(sender, DEFAULT_FEE);
        vm.deal(consumer, DEFAULT_FEE);
    }
}

contract SetupTest is SuccinctGatewayTest {
    function test_SetUp() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        assertEq(IFunctionRegistry(gateway).verifiers(functionId), verifier);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId), owner);
        assertEq(SuccinctGateway(gateway).allowedProvers(prover), true);
        assertTrue(AccessControlUpgradeable(gateway).hasRole(keccak256("TIMELOCK_ROLE"), timelock));
        assertTrue(AccessControlUpgradeable(gateway).hasRole(keccak256("GUARDIAN_ROLE"), guardian));
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.expectRevert(abi.encodeWithSelector(OnlyProver.selector, sender));
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.expectRevert(abi.encodeWithSelector(OnlyProver.selector, sender));
        vm.prank(sender);
        SuccinctGateway(gateway).fulfillCall(
            functionId, input, output, proof, callAddress, callData
        );
    }

    function test_VerifiedCall() public {
        bytes32 functionId = TestConsumer(consumer).FUNCTION_ID();
        bytes32 inputHash = INPUT_HASH;

        // Set the SuccinctGateway's storage slots to avoid revert:
        // | verifiedFunctionId | bytes32 | 255
        // | verifiedInputHash  | bytes32 | 256
        // | verifiedOutput     | bytes   | 257
        vm.store(gateway, bytes32(uint256(255)), functionId);
        vm.store(gateway, bytes32(uint256(256)), inputHash);

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
            owner, type(TestFunctionVerifier1).creationCode, "attack-verifier"
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
        vm.prank(prover);
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
            IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");

        // Deploy verifier
        address verifier1;
        bytes memory bytecode = type(TestFunctionVerifier1).creationCode;
        bytes32 salt = expectedFunctionId1;
        assembly {
            verifier1 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Register function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionRegistered(expectedFunctionId1, verifier1, "test-verifier1", owner);
        bytes32 functionId1 =
            IFunctionRegistry(gateway).registerFunction(owner, verifier1, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(expectedFunctionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(expectedFunctionId1), owner);
    }

    function test_RegisterFunction_WhenOwnerIsSender() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");

        // Deploy verifier
        address verifier1;
        bytes memory bytecode = type(TestFunctionVerifier1).creationCode;
        bytes32 salt = expectedFunctionId1;
        assembly {
            verifier1 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Register function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionRegistered(expectedFunctionId1, verifier1, "test-verifier1", owner);
        vm.prank(owner);
        bytes32 functionId1 =
            IFunctionRegistry(gateway).registerFunction(owner, verifier1, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(expectedFunctionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(expectedFunctionId1), owner);
    }

    function test_RevertRegisterFunction_WhenAlreadyRegistered() public {
        // Deploy verifier
        address verifier1;
        bytes memory bytecode = type(TestFunctionVerifier1).creationCode;
        bytes32 salt = IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");
        assembly {
            verifier1 := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // Register function
        vm.expectEmit(true, true, true, true, gateway);
        emit FunctionRegistered(salt, verifier1, "test-verifier1", owner);
        IFunctionRegistry(gateway).registerFunction(owner, verifier1, "test-verifier1");

        // Register function again
        vm.expectRevert(abi.encodeWithSelector(FunctionAlreadyRegistered.selector, salt));
        IFunctionRegistry(gateway).registerFunction(owner, verifier1, "test-verifier1");
    }

    function test_DeployAndRegisterFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");

        // Deploy verifier and register function
        vm.expectEmit(true, false, false, true, gateway);
        emit Deployed(
            keccak256(type(TestFunctionVerifier1).creationCode), expectedFunctionId1, address(0)
        );
        vm.expectEmit(true, true, true, false, gateway);
        emit FunctionRegistered(expectedFunctionId1, address(0), "test-verifier1", owner);
        (bytes32 functionId1, address verifier1) = IFunctionRegistry(gateway)
            .deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), owner);
    }

    function test_DeployAndRegisterFunction_WhenOwnerIsSender() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");

        // Deploy verifier and register function
        vm.expectEmit(true, false, false, true, gateway);
        emit Deployed(
            keccak256(type(TestFunctionVerifier1).creationCode), expectedFunctionId1, address(0)
        );
        vm.expectEmit(true, true, true, false, gateway);
        emit FunctionRegistered(expectedFunctionId1, address(0), "test-verifier1", owner);
        vm.prank(owner);
        (bytes32 functionId1, address verifier1) = IFunctionRegistry(gateway)
            .deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier1);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), owner);
    }

    function test_RevertDeployAndRegisterFunction_WhenAlreadyRegistered() public {
        // Deploy verifier and register function
        (bytes32 functionId1,) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and register function again
        vm.expectRevert(abi.encodeWithSelector(FunctionAlreadyRegistered.selector, functionId1));
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );
    }

    function test_UpdateFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");

        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
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
        vm.prank(owner);
        bytes32 functionId1 = IFunctionRegistry(gateway).updateFunction(verifier2, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier2);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), owner);
    }

    function test_RevertUpdateFunction_WhenNotOwner() public {
        // Deploy verifier and register function
        (bytes32 functionId,) = IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
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
        vm.expectRevert(abi.encodeWithSelector(NotFunctionOwner.selector, owner, address(0)));
        vm.prank(owner);
        IFunctionRegistry(gateway).updateFunction(verifier2, "test-verifier1");
    }

    function test_RevertUpdateFunction_WhenVerifierSame() public {
        // Deploy verifier and register function
        (bytes32 functionId1, address verifier1) = IFunctionRegistry(gateway)
            .deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Update function
        vm.expectRevert(abi.encodeWithSelector(VerifierAlreadyUpdated.selector, functionId1));
        vm.prank(owner);
        IFunctionRegistry(gateway).updateFunction(verifier1, "test-verifier1");
    }

    function test_deployAndUpdateFunction() public {
        bytes32 expectedFunctionId1 =
            IFunctionRegistry(gateway).getFunctionId(owner, "test-verifier1");

        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and update function
        vm.expectEmit(true, false, false, true, gateway);
        emit Deployed(
            keccak256(type(TestFunctionVerifier2).creationCode), expectedFunctionId1, address(0)
        );
        vm.expectEmit(true, true, true, false, gateway);
        emit FunctionVerifierUpdated(expectedFunctionId1, address(0));
        vm.prank(owner);
        (bytes32 functionId1, address verifier2) = IFunctionRegistry(gateway)
            .deployAndUpdateFunction(type(TestFunctionVerifier2).creationCode, "test-verifier1");

        assertEq(functionId1, expectedFunctionId1);
        assertEq(IFunctionRegistry(gateway).verifiers(functionId1), verifier2);
        assertEq(IFunctionRegistry(gateway).verifierOwners(functionId1), owner);
    }

    function test_RevertDeployAndUpdateFunction_WhenNotOwner() public {
        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
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
        vm.expectRevert(abi.encodeWithSelector(NotFunctionOwner.selector, owner, address(0)));
        vm.prank(owner);
        IFunctionRegistry(gateway).deployAndUpdateFunction(
            type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );
    }

    function test_RevertDeployAndUpdateFunction_WhenBytecodeSame() public {
        // Deploy verifier and register function
        IFunctionRegistry(gateway).deployAndRegisterFunction(
            owner, type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );

        // Deploy verifier and update function
        vm.expectRevert(abi.encodeWithSelector(FailedDeploy.selector));
        vm.prank(owner);
        IFunctionRegistry(gateway).deployAndUpdateFunction(
            type(TestFunctionVerifier1).creationCode, "test-verifier1"
        );
    }
}

contract UpdateProverTest is SuccinctGatewayTest {
    function test_AddProver() public {
        address newProver = makeAddr("new-prover");

        vm.prank(guardian);
        SuccinctGateway(gateway).addProver(newProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(newProver), true);
    }

    function test_RevertAddProver_WhenNotGuardian() public {
        address newProver = makeAddr("new-prover");

        vm.expectRevert(abi.encodeWithSignature("OnlyGuardian(address)", sender));
        vm.prank(sender);
        SuccinctGateway(gateway).addProver(newProver);
        assertEq(SuccinctGateway(gateway).allowedProvers(newProver), false);
    }

    function test_RemoveProver() public {
        vm.prank(guardian);
        SuccinctGateway(gateway).removeProver(prover);
        assertEq(SuccinctGateway(gateway).allowedProvers(prover), false);
    }

    function test_RevertRemoveProver_WhenNotGuardian() public {
        vm.expectRevert(abi.encodeWithSignature("OnlyGuardian(address)", sender));
        vm.prank(sender);
        SuccinctGateway(gateway).removeProver(prover);
        assertEq(SuccinctGateway(gateway).allowedProvers(prover), true);
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
        TestConsumer(consumer).requestCall{value: fee}(callGasLimit);
    }

    function test_RevertSetFeeVault_WhenNotGuardian() public {
        address newFeeVault = address(new SuccinctFeeVault());

        // Set FeeVault
        vm.expectRevert();
        SuccinctGateway(gateway).setFeeVault(newFeeVault);
    }
}
