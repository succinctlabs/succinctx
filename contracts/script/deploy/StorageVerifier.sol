// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "script/misc/Base.s.sol";
import {StorageVerifier} from "src/examples/storage/StorageVerifier.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IFunctionRegistry} from "src/interfaces/IFunctionRegistry.sol";

contract DeployStorageVerifier is BaseScript {
    function run() external broadcaster {
        console.log("Deploying StorageVerifier contract on chain %s", Strings.toString(block.chainid));

        // Check inputs
        address FUNCTION_GATEWAY = envAddress("FUNCTION_GATEWAY", block.chainid);
        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");

        // Deploy contract
        StorageVerifier verifier = new StorageVerifier{salt: CREATE2_SALT}();
        bytes32 functionId = IFunctionRegistry(FUNCTION_GATEWAY).registerFunction(address(verifier), "storage");
        console.log("FunctionId:");
        console.logBytes32(functionId);

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "STORAGE_VERIFIER", address(verifier));
    }
}
