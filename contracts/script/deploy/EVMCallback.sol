// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "../misc/Base.s.sol";
import {EVMCallback} from "../../src/examples/EVMCallback.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeployEVMCallback is BaseScript {
    function run() external broadcaster {
        console.log("Deploying EVMCallback contract on chain %s", Strings.toString(block.chainid));

        // Check inputs
        address GATEWAY = envAddress("SUCCINCT_GATEWAY", block.chainid);
        bytes32 FUNCTION_ID = envBytes32("FUNCTION_ID");

        // Deploy contract
        EVMCallback evm = new EVMCallback();
        evm.initialize(GATEWAY, FUNCTION_ID);

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "EVM_CALLBACK", address(evm));
    }
}
