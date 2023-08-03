// SPDX-License-Identifier: MIT

pragma solidity ^0.8.16;

import {IFunctionVerifier} from "./interfaces/IFunctionVerifier.sol";
import {IFunctionRegistry} from "./interfaces/IFunctionRegistry.sol";

contract FunctionRegistry is IFunctionRegistry {
    /// @dev Maps proof ids to their corresponding verifiers.
    mapping(bytes32 => IFunctionVerifier) public verifiers;

    /// @dev Maps proof ids to their corresponding owners.
    mapping(bytes32 => address) public verifierOwners;

    /// @dev Registers a proof with the registry.
    /// @param _functionId The id of the proof to be registered.
    /// @param _verifier The address of the verifier.
    /// @param _owner The owner of the verifier.
    function registerFunction(bytes32 _functionId, address _verifier, address _owner) external {
        if (address(verifiers[_functionId]) != address(0)) {
            revert FunctionAlreadyRegistered(_functionId);
        }
        verifiers[_functionId] = IFunctionVerifier(_verifier);
        verifierOwners[_functionId] = _owner;
    }

    /// @dev Updates the verifier of a proof.
    /// @param _functionId The id of the proof to be updated.
    /// @param _verifier The address of the verifier.
    function updateFunctionVerifier(bytes32 _functionId, address _verifier) external {
        if (address(verifiers[_functionId]) == address(0)) {
            revert FunctionNotRegistered(_functionId);
        } else if (msg.sender != verifierOwners[_functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[_functionId]);
        }
        verifiers[_functionId] = IFunctionVerifier(_verifier);
    }

    /// @dev Updates the owner of a proof.
    /// @param _functionId The id of the proof to be updated.
    /// @param _owner The owner of the verifier.
    function updateFunctionOwner(bytes32 _functionId, address _owner) external {
        if (address(verifiers[_functionId]) == address(0)) {
            revert FunctionNotRegistered(_functionId);
        } else if (msg.sender != verifierOwners[_functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[_functionId]);
        }
        verifierOwners[_functionId] = _owner;
    }
}
