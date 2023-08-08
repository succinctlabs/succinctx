// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "script/misc/Base.s.sol";
import {Timelock} from "src/upgrade/Timelock.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeployTimelock is BaseScript {
    function run() external broadcaster {
        console.log("Deploying Timelock contract on chain %s", Strings.toString(block.chainid));

        // Check inputs

        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        uint256 MINIMUM_DELAY = envUint256("MINIMUM_DELAY");
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);

        address[] memory PROPOSERS = new address[](1);
        PROPOSERS[0] = GUARDIAN;
        address[] memory EXECUTORS = new address[](1);
        EXECUTORS[0] = GUARDIAN;

        // Deploy contract
        Timelock timelock = new Timelock{salt: CREATE2_SALT}(
            MINIMUM_DELAY, PROPOSERS, EXECUTORS, address(0)
        );

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "TIMELOCK", address(timelock));
    }
}
