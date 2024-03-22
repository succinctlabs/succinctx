// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {NetworkStorage} from "./NetworkStorage.sol";
import {Initializable} from "@openzeppelin-upgradeable/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgradeable/contracts/access/OwnableUpgradeable.sol";

interface INetworkAdminEvents {
    event FeeVaultSet(address oldFeeVault, address newFeeVault);
    event DefaultProverUpdated(address indexed prover, bool added);
}

interface INetworkAdminErrors {
    error RecoverFailed();
}

interface INetworkAdmin is INetworkAdminEvents, INetworkAdminErrors {
    function addDefaultProver(address prover) external;
    function removeDefaultProver(address prover) external;
    function setFeeVault(address feeVault) external;
    function recover(address to, uint256 amount) external;
}

abstract contract NetworkAdmin is
    INetworkAdmin,
    NetworkStorage,
    Initializable,
    OwnableUpgradeable
{
    function __NetworkAdmin_init(address _owner, address _feeVault, address _defaultProver)
        internal
    {
        _transferOwnership(_owner);
        feeVault = _feeVault;
        allowedProvers[address(0)][_defaultProver] = true;
    }

    function addDefaultProver(address _prover) external onlyOwner {
        allowedProvers[address(0)][_prover] = true;
        emit DefaultProverUpdated(_prover, true);
    }

    function removeDefaultProver(address _prover) external onlyOwner {
        delete allowedProvers[address(0)][_prover];
        emit DefaultProverUpdated(_prover, false);
    }

    function setFeeVault(address _feeVault) external onlyOwner {
        emit FeeVaultSet(feeVault, _feeVault);
        feeVault = _feeVault;
    }

    function recover(address _to, uint256 _amount) external onlyOwner {
        (bool success,) = _to.call{value: _amount}("");
        if (!success) {
            revert RecoverFailed();
        }
    }
}
