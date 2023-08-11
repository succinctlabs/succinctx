// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "script/misc/Base.s.sol";
import {StorageOracle} from "src/examples/storage/StorageOracle.sol";
import {Proxy} from "src/upgrades/Proxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeployStorageOracle is BaseScript {
    function run() external broadcaster {
        console.log("Deploying StorageOracle contract on chain %s", Strings.toString(block.chainid));

        // Check inputs
        address FUNCTION_GATEWAY = envAddress("FUNCTION_GATEWAY", block.chainid);
        bytes32 FUNCTION_ID = envBytes32("FUNCTION_ID");
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        bool UPGRADE = envBool("UPGRADE_VIA_EOA", false);

        // Deploy contract
        StorageOracle gatewayImpl = new StorageOracle{salt: CREATE2_SALT}();
        StorageOracle gateway;
        if (!UPGRADE) {
            gateway = StorageOracle(address(new Proxy{salt: CREATE2_SALT}(address(gatewayImpl), "")));
            gateway.initialize(FUNCTION_GATEWAY, FUNCTION_ID, TIMELOCK, GUARDIAN);
        } else {
            gateway = StorageOracle(envAddress("STORAGE_ORACLE", block.chainid));
            gateway.upgradeTo(address(gatewayImpl));
        }

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "STORAGE_ORACLE", address(gateway));
        writeEnvAddress(DEPLOYMENT_FILE, "STORAGE_ORACLE_IMPL", address(gatewayImpl));
    }
}
