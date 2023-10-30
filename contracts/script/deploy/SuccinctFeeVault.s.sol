// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "../misc/Base.s.sol";
import {SuccinctFeeVault} from "../../src/payments/SuccinctFeeVault.sol";
import {Proxy} from "../../src/upgrades/Proxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeploySuccinctFeeVault is BaseScript {
    function run() external broadcaster {
        console.log(
            "Deploying SuccinctFeeVault contract on chain %s", Strings.toString(block.chainid)
        );

        // Check inputs
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        bool UPGRADE = envBool("UPGRADE_VIA_EOA", false);

        // Deploy contract
        SuccinctFeeVault vaultImpl = new SuccinctFeeVault{salt: CREATE2_SALT}();
        SuccinctFeeVault vault;
        if (!UPGRADE) {
            vault = SuccinctFeeVault(address(new Proxy{salt: CREATE2_SALT}(address(vaultImpl), "")));
            vault.initialize(TIMELOCK, GUARDIAN);
        } else {
            vault = SuccinctFeeVault(envAddress("SUCCINCT_FEE_VAULT", block.chainid));
            vault.upgradeTo(address(vaultImpl));
        }

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "SUCCINCT_FEE_VAULT", address(vault));
        writeEnvAddress(DEPLOYMENT_FILE, "SUCCINCT_FEE_VAULT_IMPL", address(vaultImpl));
    }
}
