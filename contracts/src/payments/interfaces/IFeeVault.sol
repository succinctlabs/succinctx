// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

interface IFeeVault {
    /// @notice Returns the amount of active balance that an account has.
    /// @param token The address of the token to check the balance of. To check native currency
    ///        balance, use address(0) as the token address.
    /// @param account The address of the account to check the balance of.
    function balances(address token, address account) external view returns (uint256);

    /// @notice Deposit the specified amount of native currency from the caller.
    /// @dev The native currency is represented by address(0) in balances.
    /// @param account The account to deposit the native currency for.
    function depositNative(address account) external payable;

    /// @notice Deposit the specified amount of the specified token from the caller.
    /// @dev MUST approve this contract to spend at least `amount` of `token` before calling this.
    /// @param account The account to deposit the tokens to.
    /// @param token The address of the token to deposit.
    /// @param amount The amount of the token to deposit.
    function deposit(address account, address token, uint256 amount) external;
}
