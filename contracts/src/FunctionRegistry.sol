// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionRegistry} from "./interfaces/IFunctionRegistry.sol";

abstract contract FunctionRegistry is IFunctionRegistry {
    /// @dev Maps function identifiers to their corresponding verifiers.
    mapping(bytes32 => address) public verifiers;

    /// @dev Maps function identifiers to their corresponding owners.
    mapping(bytes32 => address) public verifierOwners;

    /// @notice Registers a function, using a pre-deployed verifier.
    /// @param _owner The owner of the function.
    /// @param _verifier The address of the verifier.
    /// @param _name The name of the function to be registered.
    function registerFunction(address _owner, address _verifier, string memory _name)
        external
        returns (bytes32 functionId)
    {
        functionId = getFunctionId(_owner, _name);
        if (address(verifiers[functionId]) != address(0)) {
            revert FunctionAlreadyRegistered(functionId); // should call update instead
        }
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        verifierOwners[functionId] = _owner;
        verifiers[functionId] = _verifier;

        emit FunctionRegistered(functionId, _verifier, _name, _owner);
    }

    /// @notice Registers a function, using CREATE2 to deploy the verifier.
    /// @param _owner The owner of the function.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _name The name of the function to be registered.
    function deployAndRegisterFunction(address _owner, bytes memory _bytecode, string memory _name)
        external
        returns (bytes32 functionId, address verifier)
    {
        functionId = getFunctionId(_owner, _name);
        if (address(verifiers[functionId]) != address(0)) {
            revert FunctionAlreadyRegistered(functionId); // should call update instead
        }

        verifierOwners[functionId] = _owner;
        verifier = _deploy(_bytecode, functionId);
        verifiers[functionId] = verifier;

        emit FunctionRegistered(functionId, verifier, _name, _owner);
    }

    /// @notice Updates the function, using a pre-deployed verifier.
    /// @dev Only the owner of the function can update it.
    /// @param _verifier The address of the verifier.
    /// @param _name The name of the function to be updated.
    function updateFunction(address _verifier, string memory _name)
        external
        returns (bytes32 functionId)
    {
        functionId = getFunctionId(msg.sender, _name);
        if (msg.sender != verifierOwners[functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[functionId]);
        }
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        verifiers[functionId] = _verifier;

        emit FunctionVerifierUpdated(functionId, _verifier);
    }

    /// @notice Updates the function, using CREATE2 to deploy the new verifier.
    /// @dev Only the owner of the function can update it.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _name The name of the function to be updated.
    function deployAndUpdateFunction(bytes memory _bytecode, string memory _name)
        external
        returns (bytes32 functionId, address verifier)
    {
        functionId = getFunctionId(msg.sender, _name);
        if (msg.sender != verifierOwners[functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[functionId]);
        }
        verifier = _deploy(_bytecode, functionId);
        verifiers[functionId] = verifier;

        emit FunctionVerifierUpdated(functionId, verifier);
    }

    /// @notice Returns the functionId for a given owner and function name.
    /// @param _owner The owner of the function.
    /// @param _name The name of the function.
    function getFunctionId(address _owner, string memory _name)
        public
        pure
        returns (bytes32 functionId)
    {
        functionId = keccak256(abi.encode(_owner, _name));
    }

    function _deploy(bytes memory _bytecode, bytes32 _salt)
        internal
        returns (address deployedAddr)
    {
        if (_bytecode.length == 0) revert EmptyBytecode();

        assembly {
            deployedAddr := create2(0, add(_bytecode, 32), mload(_bytecode), _salt)
        }
        if (deployedAddr == address(0)) revert FailedDeploy();

        emit Deployed(keccak256(_bytecode), _salt, deployedAddr);
    }
}
