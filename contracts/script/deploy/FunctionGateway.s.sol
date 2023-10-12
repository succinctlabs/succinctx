// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "../misc/Base.s.sol";
import {FunctionGateway} from "../../src/FunctionGateway.sol";
import {Proxy} from "../../src/upgrades/Proxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract DeployFunctionGateway is BaseScript {
    function run() external broadcaster {
        console.log(
            "Deploying FunctionGateway contract on chain %s", Strings.toString(block.chainid)
        );

        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        bool UPGRADE = envBool("UPGRADE_VIA_EOA", false);

        // Deploy contract
        FunctionGateway gatewayImpl = new FunctionGateway{salt: CREATE2_SALT}();
        FunctionGateway gateway;
        if (!UPGRADE) {
            gateway =
                FunctionGateway(address(new Proxy{salt: CREATE2_SALT}(address(gatewayImpl), "")));
            gateway.initialize(TIMELOCK, GUARDIAN);
        } else {
            gateway = FunctionGateway(envAddress("FUNCTION_GATEWAY", block.chainid));
            gateway.upgradeTo(address(gatewayImpl));
        }

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "FUNCTION_GATEWAY", address(gateway));
        writeEnvAddress(DEPLOYMENT_FILE, "FUNCTION_GATEWAY_IMPL", address(gatewayImpl));
    }
}
