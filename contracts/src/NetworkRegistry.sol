// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {WhitelistStatus} from "./interfaces/ISuccinctGateway.sol";
import {NetworkStorage} from "./NetworkStorage.sol";

interface INetworkRegistryEvents {
    event VerifierRegistered(address indexed verifier, address owner);
    event CustomProverUpdated(address indexed verifier, address indexed prover, bool added);
    event WhitelistStatusUpdated(address indexed verifier, WhitelistStatus status);
}

interface INetworkRegistryErrors {
    error OnlyProver(address verifier, address prover);
    error BytecodeCannotBeEmpty();
    error VerifierCannotBeZero();
    error VerifierAlreadyExists(address verifier);
    error NotVerifierOwner(address expectedOwner, address actualOwner);
}

interface INetworkRegistry is INetworkRegistryEvents, INetworkRegistryErrors {
    function registerVerifier(address owner, address verifier) external;
    function deployAndRegisterVerifier(address owner, bytes memory bytecode, bytes32 salt)
        external
        returns (address verifier);
    function setWhitelistStatus(address verifier, WhitelistStatus status) external;
    function addCustomProver(address verifier, address prover) external;
    function removeCustomProver(address verifier, address prover) external;
}

abstract contract NetworkRegistry is INetworkRegistry, NetworkStorage {
    modifier onlyProver(address _verifier) {
        if (
            whitelistStatus[_verifier] == WhitelistStatus.Default
                && !allowedProvers[address(0)][msg.sender]
        ) {
            revert OnlyProver(_verifier, msg.sender);
        } else if (
            whitelistStatus[_verifier] == WhitelistStatus.Custom
                && !allowedProvers[_verifier][msg.sender]
        ) {
            revert OnlyProver(_verifier, msg.sender);
        }
        _;
    }

    function registerVerifier(address _owner, address _verifier) external override {
        _register(_owner, _verifier);
    }

    function deployAndRegisterVerifier(address _owner, bytes calldata _bytecode, bytes32 _salt)
        external
        override
        returns (address verifier)
    {
        verifier = _deploy(_bytecode, _salt);
        _register(_owner, verifier);
    }

    function setWhitelistStatus(address _verifier, WhitelistStatus _status) external override {
        if (msg.sender != verifierOwners[_verifier]) {
            revert NotVerifierOwner(msg.sender, verifierOwners[_verifier]);
        }
        whitelistStatus[_verifier] = _status;
        emit WhitelistStatusUpdated(_verifier, _status);
    }

    function addCustomProver(address _verifier, address _prover) external override {
        if (msg.sender != verifierOwners[_verifier]) {
            revert NotVerifierOwner(msg.sender, verifierOwners[_verifier]);
        }
        allowedProvers[_verifier][_prover] = true;
        emit CustomProverUpdated(_verifier, _prover, true);
    }

    function removeCustomProver(address _verifier, address _prover) external override {
        if (msg.sender != verifierOwners[_verifier]) {
            revert NotVerifierOwner(msg.sender, verifierOwners[_verifier]);
        }
        delete allowedProvers[_verifier][_prover];
        emit CustomProverUpdated(_verifier, _prover, false);
    }

    function _deploy(bytes memory _bytecode, bytes32 _salt)
        internal
        returns (address deployedAddr)
    {
        if (_bytecode.length == 0) revert BytecodeCannotBeEmpty();

        assembly {
            deployedAddr := create2(0, add(_bytecode, 32), mload(_bytecode), _salt)
        }
    }

    function _register(address _owner, address _verifier) internal {
        if (_verifier == address(0)) {
            revert VerifierCannotBeZero();
        }
        if (verifierOwners[_verifier] != address(0)) {
            revert VerifierAlreadyExists(_verifier);
        }
        verifierOwners[_verifier] = _owner;

        emit VerifierRegistered(_verifier, _owner);
    }
}
