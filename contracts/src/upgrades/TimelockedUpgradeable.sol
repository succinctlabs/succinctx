// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {Versioned} from "src/upgrades/Versioned.sol";
import {Initializable} from "@openzeppelin-upgradeable/contracts/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgradeable/contracts/proxy/utils/UUPSUpgradeable.sol";
import {AccessControlUpgradeable} from "@openzeppelin-upgradeable/contracts/access/AccessControlUpgradeable.sol";

/// @title TimelockedUpgradeable
/// @notice A base contract that has modifiers to specify that certain functions are only callable
///         by a sender with a timelock or guardian role.
abstract contract TimelockedUpgradeable is Versioned, Initializable, UUPSUpgradeable, AccessControlUpgradeable {
    /// @notice A random constant used to identify addresses with the permission of a 'timelock'.
    /// @dev Should be set to a 'timelock' contract, which may only execute calls after being
    ///      for a certain amount of time.
    bytes32 public constant TIMELOCK_ROLE = keccak256("TIMELOCK_ROLE");
    /// @notice A random constant used to identify addresses with the permission of a 'guardian'.
    /// @dev Can be set to any address, which may execute calls immediately.
    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");

    error OnlyTimelock(address sender);
    error OnlyGuardian(address sender);

    modifier onlyTimelock() {
        if (!hasRole(TIMELOCK_ROLE, msg.sender)) {
            revert OnlyTimelock(msg.sender);
        }
        _;
    }

    modifier onlyGuardian() {
        if (!hasRole(GUARDIAN_ROLE, msg.sender)) {
            revert OnlyGuardian(msg.sender);
        }
        _;
    }

    /// @notice Prevents the implementation contract from being initialized outside of an
    ///         upgradeable proxy.
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev The DEFAULT_ADMIN_ROLE needs to be set but should be unused.
    function __TimelockedUpgradeable_init(address _timelock, address _guardian) internal onlyInitializing {
        __AccessControl_init();
        __UUPSUpgradeable_init();
        _grantRole(DEFAULT_ADMIN_ROLE, _timelock);
        _grantRole(TIMELOCK_ROLE, _timelock);
        _grantRole(GUARDIAN_ROLE, _guardian);
    }

    /// @notice Authorizes an upgrade for the implementation contract.
    function _authorizeUpgrade(address newImplementation) internal virtual override onlyTimelock {}
}
