// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFeeVault} from "./interfaces/IFeeVault.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title SuccinctFeeVault
/// @author Succinct Labs
/// @notice Endpoint for sending fees when using Succinct services. For requests to get processed,
///         the sender must have enough balance in the FeeVault.
///         This can be deposited:
///            1) Before requesting, by calling the FeeVault deposit function.
///            2) When requesting, by sending msg.value with the call to the request function.
///            3) After requesting, by calling the FeeVault deposit function.
///         It is recommended to use (1) or (2), as (3) may delay the processing of the request.
///         Any overspent fees will be used for future requests, so it may be more suitable to
///         make a bulk deposit.
/// @dev Address(0) is used to represent native currency in places where token address is specified.
contract SuccinctFeeVault is IFeeVault, Ownable {
    using SafeERC20 for IERC20;

    /// @notice Tracks the amount of active balance that an account has for Succinct services.
    /// @dev balances[token][account] returns the amount of balance for token the account has. To
    ///      check native currency balance, use address(0) as the token address.
    mapping(address => mapping(address => uint256)) public override balances;
    /// @notice The allowed senders for the deduct functions.
    mapping(address => bool) public allowedDeductors;

    modifier onlyDeductor() {
        if (!allowedDeductors[msg.sender]) {
            revert OnlyDeductor(msg.sender);
        }
        _;
    }

    /// @dev Initializes the contract.
    /// @param _owner The address of the owner of the contract.
    constructor(address _owner) {
        transferOwnership(_owner);
    }

    /// @notice Add the specified deductor.
    /// @param _deductor The address of the deductor to add.
    function addDeductor(address _deductor) external onlyOwner {
        allowedDeductors[_deductor] = true;
    }

    /// @notice Remove the specified deductor.
    /// @param _deductor The address of the deductor to remove.
    function removeDeductor(address _deductor) external onlyOwner {
        allowedDeductors[_deductor] = false;
    }

    /// @notice Deposit the specified amount of native currency from the caller.
    /// @dev The native currency is represented by address(0) in balances.
    /// @param _account The account to deposit the native currency for.
    function depositNative(address _account) external payable override {
        if (_account == address(0)) {
            revert InvalidAccount(_account);
        }

        balances[address(0)][_account] += msg.value;

        emit Received(_account, address(0), msg.value);
    }

    /// @notice Deposit the specified amount of the specified token from the caller.
    /// @dev MUST approve this contract to spend at least _amount of _token before calling this.
    /// @param _account The account to deposit the tokens to.
    /// @param _token The address of the token to deposit.
    /// @param _amount The amount of the token to deposit.
    function deposit(address _account, address _token, uint256 _amount) external override {
        if (_account == address(0)) {
            revert InvalidAccount(_account);
        }
        if (_token == address(0)) {
            revert InvalidToken(_token);
        }

        IERC20 token = IERC20(_token);
        uint256 allowance = token.allowance(msg.sender, address(this));
        if (allowance < _amount) {
            revert InsufficentAllowance(_token, _amount);
        }

        token.safeTransferFrom(msg.sender, address(this), _amount);
        balances[_token][_account] += _amount;

        emit Received(_account, _token, _amount);
    }

    /// @notice Deduct the specified amount of native currency from the specified account.
    /// @param _account The account to deduct the native currency from.
    /// @param _amount The amount of native currency to deduct.
    function deductNative(address _account, uint256 _amount) external onlyDeductor {
        if (_account == address(0)) {
            revert InvalidAccount(_account);
        }
        if (balances[address(0)][_account] < _amount) {
            revert InsufficientBalance(address(0), _amount);
        }

        balances[address(0)][_account] -= _amount;

        emit Deducted(_account, address(0), _amount);
    }

    /// @notice Deduct the specified amount of the specified token from the specified account.
    /// @param _account The account to deduct the tokens from.
    /// @param _token The address of the token to deduct.
    /// @param _amount The amount of the token to deduct.
    function deduct(address _account, address _token, uint256 _amount) external onlyDeductor {
        if (_account == address(0)) {
            revert InvalidAccount(_account);
        }
        if (_token == address(0)) {
            revert InvalidToken(_token);
        }
        if (balances[_token][_account] < _amount) {
            revert InsufficientBalance(_token, _amount);
        }

        balances[_token][_account] -= _amount;

        emit Deducted(_account, _token, _amount);
    }

    /// @notice Collect the specified amount of native currency.
    /// @param _to The address to send the collected native currency to.
    /// @param _amount The amount of native currency to collect.
    function collectNative(address _to, uint256 _amount) external onlyOwner {
        if (address(this).balance < _amount) {
            revert InsufficientBalance(address(0), _amount);
        }

        (bool success,) = _to.call{value: _amount}("");
        if (!success) {
            revert FailedToSendNative(_amount);
        }

        emit Collected(_to, address(0), _amount);
    }

    /// @notice Collect the specified amount of the specified token.
    /// @param _to The address to send the collected tokens to.
    /// @param _token The address of the token to collect.
    /// @param _amount The amount of the token to collect.
    function collect(address _to, address _token, uint256 _amount) external onlyOwner {
        if (_token == address(0)) {
            revert InvalidToken(_token);
        }
        if (IERC20(_token).balanceOf(address(this)) < _amount) {
            revert InsufficientBalance(_token, _amount);
        }

        IERC20(_token).transfer(_to, _amount);

        emit Collected(_to, _token, _amount);
    }
}
