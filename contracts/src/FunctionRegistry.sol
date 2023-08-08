// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionRegistry} from "./interfaces/IFunctionRegistry.sol";

contract FunctionRegistry is IFunctionRegistry {
    /// @dev Maps functionId's to their corresponding verifiers.
    mapping(bytes32 => address) public verifiers;

    /// @dev Maps functionId's to their corresponding owners.
    mapping(bytes32 => address) public verifierOwners;

    /// @notice Registers a function with the registry.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _name The name of the function to be registered.
    function registerFunction(bytes memory _bytecode, string memory _name) external returns (address verifierAddr) {
        bytes32 functionId = getFunctionId(msg.sender, _name);
        if (address(verifiers[functionId]) != address(0)) {
            revert FunctionAlreadyHasVerifier(functionId); // should call update instead
        }

        verifierAddr = _deploy(_bytecode, functionId);
        verifiers[functionId] = verifierAddr;
        verifierOwners[functionId] = msg.sender;

        emit FunctionRegistered(functionId, verifierAddr, _name, msg.sender);
    }

    /// @notice Updates the function with a new verifier.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _name The name of the function to be updated.
    function updateFunction(bytes memory _bytecode, string memory _name) external returns (address verifierAddr) {
        bytes32 functionId = getFunctionId(msg.sender, _name);
        if (msg.sender != verifierOwners[functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[functionId]);
        }
        verifierAddr = _deploy(_bytecode, functionId);
        verifiers[functionId] = verifierAddr;

        emit FunctionVerifierUpdated(functionId, verifierAddr);
    }

    /// @notice Returns the functionId for a given owner and function name.
    /// @param _owner The owner of the function (sender of registerFunction).
    /// @param _name The name of the function.
    function getFunctionId(address _owner, string memory _name) public pure returns (bytes32 functionId) {
        functionId = keccak256(abi.encode(_owner, _name));
    }

    function _deploy(bytes memory _bytecode, bytes32 _salt) internal returns (address deployedAddr) {
        if (_bytecode.length == 0) revert EmptyBytecode();

        assembly {
            deployedAddr := create2(0, add(_bytecode, 32), mload(_bytecode), _salt)
        }
        if (deployedAddr == address(0)) revert FailedDeploy();

        emit Deployed(keccak256(_bytecode), _salt, deployedAddr);
    }
}
