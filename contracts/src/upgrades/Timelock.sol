// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";

/// @dev Identical to an TimelockController, with a more readable name.
contract Timelock is TimelockController {
    /// @notice Initializes the contract with a given minimum delay and roles.
    /// @param _minDelay initial minimum delay for operations
    /// @param _proposers accounts to be granted proposer and canceller roles
    /// @param _executors accounts to be granted executor role
    /// @param _admin optional account to be granted admin role; disable with zero address
    constructor(uint256 _minDelay, address[] memory _proposers, address[] memory _executors, address _admin)
        TimelockController(_minDelay, _proposers, _executors, _admin)
    {}
}
