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
    /// @param _salt The salt to use for calculating the function ID.
    function registerFunction(address _owner, address _verifier, bytes32 _salt)
        external
        override
        returns (bytes32 functionId)
    {
        functionId = getFunctionId(_owner, _salt);
        _register(functionId, _owner, _verifier);
        emit FunctionRegistered(functionId, _verifier, _salt, _owner);
    }

    /// @notice Registers a function, using CREATE2 to deploy the verifier.
    /// @param _owner The owner of the function.
    /// @param _bytecode The bytecode of the verifier.
    /// @param _salt The salt to use for calculating the function ID.
    function deployAndRegisterFunction(address _owner, bytes memory _bytecode, bytes32 _salt)
        external
        override
        returns (bytes32 functionId, address verifier)
    {
        functionId = getFunctionId(_owner, _salt);
        verifier = _deploy(_bytecode, functionId);
        _register(functionId, _owner, verifier);
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
        _update(functionId, _verifier);
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
        verifier = _deploy(_bytecode, functionId);
        _update(functionId, verifier);
        emit FunctionVerifierUpdated(functionId, verifier);
    }

    /// @notice Returns the function ID for a given owner and salt.
    /// @param _owner The owner of the function.
    /// @param _salt The salt to use.
    function getFunctionId(address _owner, bytes32 _salt)
        public
        pure
        override
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

    function _register(bytes32 functionId, address _owner, address _verifier) internal {
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        if (address(verifiers[functionId]) != address(0)) {
            revert FunctionAlreadyRegistered(functionId); // should call update instead
        }
        verifierOwners[functionId] = _owner;
        verifiers[functionId] = _verifier;
    }

    function _update(bytes32 functionId, address _verifier) internal {
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        if (msg.sender != verifierOwners[functionId]) {
            revert NotFunctionOwner(msg.sender, verifierOwners[functionId]);
        }
        if (_verifier == verifiers[functionId]) {
            revert VerifierAlreadyUpdated(functionId);
        }
        verifiers[functionId] = _verifier;
    }
}
