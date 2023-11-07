// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {IFunctionRegistry} from "./interfaces/IFunctionRegistry.sol";

abstract contract FunctionRegistry is IFunctionRegistry {
    /// @notice Maps function IDs to their corresponding verifiers.
    mapping(bytes32 => address) public verifiers;

    /// @notice Maps function IDs to their corresponding owners.
    mapping(bytes32 => address) public verifierOwners;

    /// @notice Registers a function, using a pre-deployed verifier.
    /// @param _owner The owner of the function.
    /// @param _verifier The address of the verifier.
    /// @param _salt The salt to use for calculatingthe function ID.
    function registerFunction(address _owner, address _verifier, bytes32 _salt)
        external
        returns (bytes32 functionId)
    {
        functionId = getFunctionId(_owner, _salt);
        if (address(verifiers[functionId]) != address(0)) {
            revert FunctionAlreadyRegistered(functionId); // should call update instead
        }
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        verifierOwners[functionId] = _owner;
        verifiers[functionId] = _verifier;

        emit FunctionRegistered(functionId, _verifier, _salt, _owner);
    }

    /// @notice Registers a function, using CREATE2 to deploy the verifier.
    /// @param _owner The owner of the function.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _salt The salt to use for calculatingthe function ID.
    function deployAndRegisterFunction(address _owner, bytes memory _bytecode, bytes32 _salt)
        external
        returns (bytes32 functionId, address verifier)
    {
        functionId = getFunctionId(_owner, _salt);
        if (address(verifiers[functionId]) != address(0)) {
            revert FunctionAlreadyRegistered(functionId); // should call update instead
        }

        verifierOwners[functionId] = _owner;
        verifier = _deploy(_bytecode, functionId);
        verifiers[functionId] = verifier;

        emit FunctionRegistered(functionId, verifier, _salt, _owner);
    }

    /// @notice Updates the function, using a pre-deployed verifier.
    /// @dev Only the owner of the function can update it.
    /// @param _verifier The address of the verifier.
    /// @param _salt The salt that was used when registering this function ID.
    function updateFunction(address _verifier, bytes32 _salt)
        external
        returns (bytes32 functionId)
    {
        functionId = getFunctionId(msg.sender, _salt);
        if (msg.sender != verifierOwners[functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[functionId]);
        }
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        if (_verifier == verifiers[functionId]) {
            revert VerifierAlreadyUpdated(functionId);
        }
        verifiers[functionId] = _verifier;

        emit FunctionVerifierUpdated(functionId, _verifier);
    }

    /// @notice Updates the function, using CREATE2 to deploy the new verifier.
    /// @dev Only the owner of the function can update it.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _salt The salt that was used when registering this function ID.
    function deployAndUpdateFunction(bytes memory _bytecode, bytes32 _salt)
        external
        returns (bytes32 functionId, address verifier)
    {
        functionId = getFunctionId(msg.sender, _salt);
        if (msg.sender != verifierOwners[functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[functionId]);
        }
        verifier = _deploy(_bytecode, functionId);
        verifiers[functionId] = verifier;

        emit FunctionVerifierUpdated(functionId, verifier);
    }

    /// @notice Returns the functionId for a given owner and function name.
    /// @param _owner The owner of the function.
    /// @param _salt The name of the function.
    function getFunctionId(address _owner, bytes32 _salt)
        public
        pure
        returns (bytes32 functionId)
    {
        functionId = keccak256(abi.encode(_owner, _salt));
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
