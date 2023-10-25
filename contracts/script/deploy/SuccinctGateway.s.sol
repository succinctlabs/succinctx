// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "../misc/Base.s.sol";
import {SuccinctGateway} from "../../src/SuccinctGateway.sol";
import {Proxy} from "../../src/upgrades/Proxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeploySuccinctGateway is BaseScript {
    function run() external broadcaster {
        console.log(
            "Deploying SuccinctGateway contract on chain %s", Strings.toString(block.chainid)
        );

        address SUCCINCT_FEE_VAULT = envAddress("SUCCINCT_FEE_VAULT", block.chainid);
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        bool UPGRADE = envBool("UPGRADE_VIA_EOA", false);

        // Deploy contract
        SuccinctGateway gatewayImpl = new SuccinctGateway{salt: CREATE2_SALT}();
        SuccinctGateway gateway;
        if (!UPGRADE) {
            gateway =
                SuccinctGateway(address(new Proxy{salt: CREATE2_SALT}(address(gatewayImpl), "")));
            gateway.initialize(SUCCINCT_FEE_VAULT, TIMELOCK, GUARDIAN);
        } else {
            gateway = SuccinctGateway(envAddress("SUCCINCT_GATEWAY", block.chainid));
            gateway.upgradeTo(address(gatewayImpl));
        }

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "SUCCINCT_GATEWAY", address(gateway));
        writeEnvAddress(DEPLOYMENT_FILE, "SUCCINCT_GATEWAY_IMPL", address(gatewayImpl));
    }
}
