// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "script/misc/Base.s.sol";
import {SuccinctFeeVault} from "src/payments/SuccinctFeeVault.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeploySuccinctFeeVault is BaseScript {
    function run() external broadcaster {
        console.log("Deploying SuccinctFeeVault contract on chain %s", Strings.toString(block.chainid));

        // Check inputs
        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);

        // Deploy contracts
        SuccinctFeeVault vault = new SuccinctFeeVault{salt: CREATE2_SALT}(GUARDIAN);

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "SUCCINCT_FEE_VAULT", address(vault));
    }
}
