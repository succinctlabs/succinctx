// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {Verifier as Groth16Verifier} from "./VerifierGroth16.sol";
import {PlonkVerifier} from "./VerifierPlonk.sol";
import {PlonkVerifier as PlonkRangeCheckVerifier} from "./VerifierPlonkRangeCheck.sol";
import {VmSafe} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";

contract VerifierTest is Test {
    function testVerifierGroth16() public {
        Groth16Verifier verifier = new Groth16Verifier();

        string memory groth16Json = vm.readFile(
            "test/verifiers/groth16_proof_data.json"
        );
        uint256[] memory proof = stdJson.readUintArray(groth16Json, "$.proof");
        uint256[] memory input = stdJson.readUintArray(groth16Json, "$.inputs");

        uint256[8] memory proofConverted;
        for (uint256 i = 0; i < 8; i++) {
            proofConverted[i] = uint256(proof[i]);
        }

        uint256[3] memory inputConverted;
        for (uint256 i = 0; i < 3; i++) {
            inputConverted[i] = uint256(input[i]);
        }
        uint256 startGas = gasleft();
        verifier.verifyProof(proofConverted, inputConverted);
        uint256 endGas = gasleft();
        console.log("gas used: %d", startGas - endGas);

        uint256[4] memory compressedProof = verifier.compressProof(
            proofConverted
        );
        startGas = gasleft();

        verifier.verifyCompressedProof(compressedProof, inputConverted);
        endGas = gasleft();
        console.log(
            "gas used for verifying compressed proof: %d",
            startGas - endGas
        );
    }

    function testVerifierPlonk() public {
        PlonkVerifier verifier = new PlonkVerifier();
        string memory proofJson = vm.readFile(
            "test/verifiers/plonk_proof_data.json"
        );
        bytes memory proof = stdJson.readBytes(proofJson, "$.proof");
        uint256[] memory input = stdJson.readUintArray(proofJson, "$.inputs");
        uint256 startGas = gasleft();
        require(verifier.Verify(proof, input));
        uint256 endGas = gasleft();
        console.log("gas used: %d", startGas - endGas);
    }

    function testVerifierPlonkRangeCheck() public {
        PlonkRangeCheckVerifier verifier = new PlonkRangeCheckVerifier();
        string memory proofJson = vm.readFile(
            "test/verifiers/plonk_proof_data_range_check.json"
        );
        bytes memory proof = stdJson.readBytes(proofJson, "$.proof");
        uint256[] memory input = stdJson.readUintArray(proofJson, "$.inputs");
        uint256 startGas = gasleft();
        require(verifier.Verify(proof, input));
        uint256 endGas = gasleft();
        console.log("gas used: %d", startGas - endGas);
    }
}
