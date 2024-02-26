// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "forge-std/Vm.sol";
import "forge-std/console.sol";
import {SuccinctGateway} from "../../src/SuccinctGateway.sol";

// NOTE: Update FunctionVerifier to the verifier you want to deploy and register from the
// Succinct platform.
contract FunctionVerifier {}

contract DeployAndRegisterFunction is Script {
    function run() external returns (bytes32, address) {
        vm.startBroadcast();

        // Get the bytecode of the FunctionVerifier contract.
        bytes memory bytecode = type(FunctionVerifier).creationCode;

        // SuccinctGateway address
        address GATEWAY = vm.envAddress("SUCCINCT_GATEWAY");
        console.logAddress(GATEWAY);

        // Create2 salt
        bytes32 SALT = vm.envBytes32("CREATE2_SALT");

        address OWNER = msg.sender;

        (bytes32 functionId, address verifier) =
            SuccinctGateway(GATEWAY).deployAndRegisterFunction(OWNER, bytecode, SALT);

        return (functionId, verifier);
    }
}
