// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "../misc/Base.s.sol";
import {SuccinctGateway} from "../../src/SuccinctGateway.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeploySuccinctGateway is BaseScript {
    function run() external broadcaster {
        console.log(
            "Deploying SuccinctGateway contract on chain %s", Strings.toString(block.chainid)
        );

        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        address SUCCINCT_FEE_VAULT = envAddress("SUCCINCT_FEE_VAULT", block.chainid);
        address PROVER = envAddress("PROVER", block.chainid);

        // Deploy contract
        SuccinctGateway gateway = new SuccinctGateway{salt: CREATE2_SALT}();
        gateway.initialize(GUARDIAN, SUCCINCT_FEE_VAULT, PROVER);

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "SUCCINCT_GATEWAY", address(gateway));
    }
}
