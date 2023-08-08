// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

/// @title Versioned
/// @notice Contract that provides a VERSION to inheritors.
/// @dev This is used for easier management and version compatibility checking between contracts.
abstract contract Versioned {
    function VERSION() external pure virtual returns (string memory) {
        return "1.0.0";
    }
}
