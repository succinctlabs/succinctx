// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "forge-std/Vm.sol";
import "forge-std/console.sol";
import {FunctionVerifier} from "./FunctionVerifier.sol";
import {SuccinctGateway} from "../../src/SuccinctGateway.sol";

contract DeployAndRegisterFunction is Script {
    function run() external {
        vm.startBroadcast();

        // Get the bytecode of the FunctionVerifier contract.
        bytes memory bytecode = type(FunctionVerifier).creationCode;

        // SuccinctGateway address
        address gateway = vm.envAddress("SUCCINCT_GATEWAY");
        console.logAddress(gateway);

        // Create2 salt
        bytes32 salt = vm.envBytes32("CREATE2_SALT");

        (bytes32 functionId,
            address verifier) = SuccinctGateway(gateway).deployAndRegisterFunction(msg.sender, bytecode, salt);

        console.log("Function ID: ");
        console.logBytes32(functionId);
        console.log("Verifier Address: ");
        console.logAddress(verifier);
    }
}