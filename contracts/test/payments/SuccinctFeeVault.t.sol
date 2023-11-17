// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {IFeeVault, IFeeVaultEvents, IFeeVaultErrors} from "src/payments/interfaces/IFeeVault.sol";
import {SuccinctFeeVault} from "src/payments/SuccinctFeeVault.sol";
import {Proxy} from "src/upgrades/Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {AccessControlUpgradeable} from
    "@openzeppelin-upgradeable/contracts/access/AccessControlUpgradeable.sol";

contract SuccinctFeeVaultTest is Test, IFeeVaultEvents, IFeeVaultErrors {
    uint256 constant FEE = 1 ether;

    address internal timelock;
    address internal guardian;
    address internal feeVault;
    address internal token1;
    address internal token2;
    address internal deductor;
    address internal collector;
    address internal spender1;
    address internal spender2;
    address internal account1;
    address internal account2;

    function setUp() public {
        // Init variables
        timelock = makeAddr("timelock");
        guardian = makeAddr("guardian");

        token1 = address(new ERC20("UnicornToken", "UNI"));
        token2 = address(new ERC20("DragonToken", "DRG"));
        deductor = makeAddr("deductor");
        collector = makeAddr("collector");
        spender1 = makeAddr("spender1");
        spender2 = makeAddr("spender2");
        account1 = makeAddr("account1");
        account2 = makeAddr("account2");

        // Deploy FeeVault
        address feeVaultImpl = address(new SuccinctFeeVault());
        feeVault = address(new Proxy(feeVaultImpl, ""));
        SuccinctFeeVault(feeVault).initialize(timelock, guardian);

        // Add deductor
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).addDeductor(deductor);

        // Give spenders some native
        vm.deal(spender1, FEE);
        vm.deal(spender2, FEE);

        // Give spenders some tokens
        deal(token1, spender1, FEE);
        deal(token2, spender1, FEE);
        deal(token1, spender2, FEE);
        deal(token2, spender2, FEE);
    }
}

contract SetUpTest is SuccinctFeeVaultTest {
    function test_SetUp() public {
        assertTrue(AccessControlUpgradeable(feeVault).hasRole(keccak256("TIMELOCK_ROLE"), timelock));
        assertTrue(AccessControlUpgradeable(feeVault).hasRole(keccak256("GUARDIAN_ROLE"), guardian));
        assertEq(SuccinctFeeVault(feeVault).allowedDeductors(deductor), true);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, account2), 0);
        assertEq(spender1.balance, FEE);
        assertEq(spender2.balance, FEE);
        assertEq(ERC20(token1).balanceOf(spender1), FEE);
        assertEq(ERC20(token2).balanceOf(spender1), FEE);
        assertEq(ERC20(token1).balanceOf(spender2), FEE);
        assertEq(ERC20(token2).balanceOf(spender2), FEE);
    }
}

contract DeductorTest is SuccinctFeeVaultTest {
    function test_AddDeductor() public {
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).addDeductor(spender1);
        assertEq(SuccinctFeeVault(feeVault).allowedDeductors(spender1), true);
    }

    function test_RevertAddDeductor_WhenNotGuardian() public {
        vm.expectRevert(abi.encodeWithSignature("OnlyGuardian(address)", spender1));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).addDeductor(spender1);
        assertEq(SuccinctFeeVault(feeVault).allowedDeductors(spender1), false);
    }

    function test_RemoveDeductor() public {
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).removeDeductor(deductor);
        assertEq(SuccinctFeeVault(feeVault).allowedDeductors(deductor), false);
    }

    function test_RevertRemoveDeductor_WhenNotGuardian() public {
        vm.expectRevert(abi.encodeWithSignature("OnlyGuardian(address)", spender1));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).removeDeductor(deductor);
        assertEq(SuccinctFeeVault(feeVault).allowedDeductors(deductor), true);
    }
}

contract DepositNativeTest is SuccinctFeeVaultTest {
    function test_DepositNative() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), 0);
        assertEq(spender1.balance, 0);
        assertEq(spender2.balance, FEE);
        assertEq(address(feeVault).balance, FEE);
    }

    function test_DepositNative_WhenSameAccount() public {
        vm.expectEmit(true, true, true, true);
        emit Received(spender1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(spender1);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), 0);
        assertEq(spender1.balance, 0);
        assertEq(spender2.balance, FEE);
        assertEq(address(feeVault).balance, FEE);
    }

    function test_DepositNative_WhenSameAccountMultipleSpenders() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), 0);
        assertEq(spender1.balance, 0);
        assertEq(spender2.balance, FEE);
        assertEq(address(feeVault).balance, FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender2);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), FEE * 2);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), 0);
        assertEq(spender1.balance, 0);
        assertEq(spender2.balance, 0);
        assertEq(address(feeVault).balance, FEE * 2);
    }

    function test_DepositNative_WhenMultipleAccountSameSpender() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE / 2);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE / 2}(account1);

        vm.expectEmit(true, true, true, true);
        emit Received(account2, address(0), FEE / 2);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE / 2}(account2);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), FEE / 2);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), FEE / 2);
        assertEq(spender1.balance, 0);
        assertEq(spender2.balance, FEE);
        assertEq(address(feeVault).balance, FEE);
    }

    function test_DepositNative_WhenZeroAmount() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), 0);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: 0}(account1);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account2), 0);
        assertEq(spender1.balance, FEE);
        assertEq(spender2.balance, FEE);
        assertEq(address(feeVault).balance, 0);
    }

    function test_RevertDepositNative_WhenZeroAccount() public {
        vm.expectRevert(abi.encodeWithSelector(InvalidAccount.selector, address(0)));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(address(0));
    }
}

contract DepositTest is SuccinctFeeVaultTest {
    function test_Deposit() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account2), 0);
        assertEq(ERC20(token1).balanceOf(spender1), 0);
        assertEq(ERC20(token1).balanceOf(spender2), FEE);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), FEE);
    }

    function test_Deposit_WhenSameAccount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(spender1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(spender1, token1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender2), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account2), 0);
        assertEq(ERC20(token1).balanceOf(spender1), 0);
        assertEq(ERC20(token1).balanceOf(spender2), FEE);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), FEE);
    }

    function test_Deposit_WhenSameAccountMultipleTokens() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.prank(spender1);
        ERC20(token2).approve(address(feeVault), FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token2, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token2, FEE);

        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, account1), FEE);
        assertEq(ERC20(token1).balanceOf(spender1), 0);
        assertEq(ERC20(token2).balanceOf(spender1), 0);
        assertEq(ERC20(token1).balanceOf(spender2), FEE);
        assertEq(ERC20(token2).balanceOf(spender2), FEE);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), FEE);
        assertEq(ERC20(token2).balanceOf(address(feeVault)), FEE);
    }

    function test_Deposit_WhenMultipleAccountsSameToken() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE / 2);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE / 2);

        vm.expectEmit(true, true, true, true);
        emit Received(account2, token1, FEE / 2);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account2, token1, FEE / 2);

        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), FEE / 2);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account2), FEE / 2);
        assertEq(ERC20(token1).balanceOf(spender1), 0);
        assertEq(ERC20(token1).balanceOf(spender2), FEE);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), FEE);
    }

    function test_Deposit_WhenZeroAmount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, 0);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, 0);

        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), 0);
        assertEq(ERC20(token1).balanceOf(spender1), FEE);
        assertEq(ERC20(token1).balanceOf(spender2), FEE);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), 0);
    }

    function test_RevertDeposit_WhenZeroAccount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);

        vm.expectRevert(abi.encodeWithSelector(InvalidAccount.selector, address(0)));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(address(0), token1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
    }

    function test_RevertDeposit_WhenNotApproved() public {
        vm.expectRevert(abi.encodeWithSelector(InsufficentAllowance.selector, token1, FEE));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
    }
}

contract DeductNativeTest is SuccinctFeeVaultTest {
    function test_DeductNative() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, address(0), FEE);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deductNative(account1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), 0);
    }

    function test_DeductNative_WhenNotFullAmount() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, address(0), FEE / 2);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deductNative(account1, FEE / 2);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), FEE / 2);
    }

    function test_DeductNative_WhenSameAccount() public {
        vm.expectEmit(true, true, true, true);
        emit Received(spender1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(spender1);

        vm.expectEmit(true, true, true, true);
        emit Deducted(spender1, address(0), FEE);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deductNative(spender1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), spender1), 0);
    }

    function test_DeductNative_WhenSameAccountMultipleSpenders() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender2);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, address(0), FEE * 2);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deductNative(account1, FEE * 2);
        assertEq(SuccinctFeeVault(feeVault).balances(address(0), account1), 0);
    }

    function test_DeductNative_WhenNotDeductor() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectRevert(abi.encodeWithSelector(OnlyDeductor.selector, spender1));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deductNative(account1, FEE);
    }

    function test_RevertDeductNative_WhenNotEnoughBalance() public {
        vm.expectRevert(abi.encodeWithSelector(InsufficientBalance.selector, address(0), FEE));
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deductNative(account1, FEE);
    }
}

contract DeductTest is SuccinctFeeVaultTest {
    function test_Deduct() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, token1, FEE);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account1, token1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), 0);
    }

    function test_Deduct_WhenNotFullAmount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, token1, FEE / 2);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account1, token1, FEE / 2);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), FEE / 2);
    }

    function test_Deduct_WhenSameAccount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(spender1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(spender1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Deducted(spender1, token1, FEE);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(spender1, token1, FEE);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, spender1), 0);
    }

    function test_Deduct_WhenSameAccountMultipleTokens() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.prank(spender1);
        ERC20(token2).approve(address(feeVault), FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token2, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token2, FEE);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, token1, FEE);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, token2, FEE);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account1, token2, FEE);

        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token2, account1), 0);
    }

    function test_Deduct_WhenMultipleAccountsSameToken() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);

        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE / 2);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE / 2);

        vm.expectEmit(true, true, true, true);
        emit Received(account2, token1, FEE / 2);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account2, token1, FEE / 2);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account1, token1, FEE / 2);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account1, token1, FEE / 2);

        vm.expectEmit(true, true, true, true);
        emit Deducted(account2, token1, FEE / 2);
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account2, token1, FEE / 2);

        assertEq(SuccinctFeeVault(feeVault).balances(token1, account1), 0);
        assertEq(SuccinctFeeVault(feeVault).balances(token1, account2), 0);
    }

    function test_RevertDeduct_WhenNotDeductor() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectRevert(abi.encodeWithSelector(OnlyDeductor.selector, spender1));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deduct(account1, token1, FEE);
    }

    function test_RevertDeduct_WhenNotEnoughBalance() public {
        vm.expectRevert(abi.encodeWithSelector(InsufficientBalance.selector, token1, FEE));
        vm.prank(deductor);
        SuccinctFeeVault(feeVault).deduct(account1, token1, FEE);
    }
}

contract CollectNativeTest is SuccinctFeeVaultTest {
    function test_CollectNative() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Collected(collector, address(0), FEE);
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collectNative(collector, FEE);
        assertEq(address(feeVault).balance, 0);
        assertEq(collector.balance, FEE);
    }

    function test_CollectNative_WhenNotFullAmount() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Collected(collector, address(0), FEE / 2);
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collectNative(collector, FEE / 2);
        assertEq(address(feeVault).balance, FEE / 2);
        assertEq(collector.balance, FEE / 2);
    }

    function test_CollectNative_WhenZeroAmount() public {
        vm.expectEmit(true, true, true, true);
        emit Received(account1, address(0), FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).depositNative{value: FEE}(account1);

        vm.expectEmit(true, true, true, true);
        emit Collected(collector, address(0), 0);
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collectNative(collector, 0);
        assertEq(address(feeVault).balance, FEE);
        assertEq(collector.balance, 0);
    }

    function test_RevertCollectNative_WhenNotOwner() public {
        vm.expectRevert(abi.encodeWithSignature("OnlyGuardian(address)", spender1));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).collectNative(collector, FEE);
    }

    function test_RevertCollectNative_WhenNotEnoughBalance() public {
        vm.expectRevert(abi.encodeWithSelector(InsufficientBalance.selector, address(0), FEE));
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collectNative(collector, FEE);
    }
}

contract CollectTest is SuccinctFeeVaultTest {
    function test_Collect() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Collected(collector, token1, FEE);
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collect(collector, token1, FEE);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), 0);
        assertEq(ERC20(token1).balanceOf(collector), FEE);
    }

    function test_Collect_WhenNotFullAmount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Collected(collector, token1, FEE / 2);
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collect(collector, token1, FEE / 2);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), FEE / 2);
        assertEq(ERC20(token1).balanceOf(collector), FEE / 2);
    }

    function test_Collect_WhenZeroAmount() public {
        vm.prank(spender1);
        ERC20(token1).approve(address(feeVault), FEE);
        vm.expectEmit(true, true, true, true);
        emit Received(account1, token1, FEE);
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).deposit(account1, token1, FEE);

        vm.expectEmit(true, true, true, true);
        emit Collected(collector, token1, 0);
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collect(collector, token1, 0);
        assertEq(ERC20(token1).balanceOf(address(feeVault)), FEE);
        assertEq(ERC20(token1).balanceOf(collector), 0);
    }

    function test_RevertCollect_WhenNotGuardian() public {
        vm.expectRevert(abi.encodeWithSignature("OnlyGuardian(address)", spender1));
        vm.prank(spender1);
        SuccinctFeeVault(feeVault).collect(collector, token1, FEE);
    }

    function test_RevertCollect_WhenNotEnoughBalance() public {
        vm.expectRevert(abi.encodeWithSelector(InsufficientBalance.selector, token1, FEE));
        vm.prank(guardian);
        SuccinctFeeVault(feeVault).collect(collector, token1, FEE);
    }
}
