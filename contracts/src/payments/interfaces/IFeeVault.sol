// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

interface IFeeVaultEvents {
    event Received(address indexed account, address indexed token, uint256 amount);
    event Deducted(address indexed account, address indexed token, uint256 amount);
    event Collected(address indexed to, address indexed token, uint256 amount);
}

interface IFeeVaultErrors {
    error InvalidAccount(address account);
    error InvalidToken(address token);
    error InsufficentAllowance(address token, uint256 amount);
    error InsufficientBalance(address token, uint256 amount);
    error FailedToSendNative(uint256 amount);
    error OnlyDeductor(address sender);
}

interface IFeeVault is IFeeVaultEvents, IFeeVaultErrors {
    function balances(address token, address account) external view returns (uint256);
    function depositNative(address account) external payable;
    function deposit(address account, address token, uint256 amount) external;
}
