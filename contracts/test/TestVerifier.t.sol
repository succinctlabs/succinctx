// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";
import {Verifier} from "./Verifier.sol";

contract VerifierTest is Test {
    function setUp() public {}

    function testVerifier() public {
        Verifier verifier = new Verifier();
        // bytes
        //     memory _proof = hex"22e78ec19c4dc89c54067a08c5d2310f2dbf10f8e5a6953c7e5ad99c95cb8c5c2960259b6fd4777ee8425810b0b8c12de4b9fceb8145674bdc409191216074672500729d177eb586e78e5fad740d58419062d87696bf0c3c0d2b125fe752b71f27efdd1436f3fa43a4328ce0c8fe8f3e60031a9358eea584204791183a7d4a9e0fffb73016d515f38fc5c4b8932aad5bc2863a516305a72a113aff6268328df611f705f1d515fdd3766e3490befbcd10386176d479e339a1cf4ed6b024795e4e2ac4a47a61cb5ed7c5311d5c67911f5955a7ef65df689eca293e685c907824820198e0266591a0034ae749db89c74098e324c1d2c01a81d28b71154c0707dde2";
        // (uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c) = abi
        //     .decode(_proof, (uint256[2], uint256[2][2], uint256[2]));
        // uint256[8] memory flatProof = [
        //     a[0],
        //     a[1],
        //     b[0][0],
        //     b[0][1],
        //     b[1][0],
        //     b[1][1],
        //     c[0],
        //     c[1]
        // ];

        bytes32[8] memory proof = [
            bytes32(
                0x0d818b30a3c11e1e7b4ec2313ec11e5b5fc714b2d6894616071581deac1099a6
            ),
            bytes32(
                0x1f1a8814267fb831c1fdf1c2d30c9f9d28fef7f7ccfd4633c98cf6c976dea5ee
            ),
            bytes32(
                0x09e4f48203d203e5799f4a3a33c892ea286f871d4ebe43901fce723dfa3cca20
            ),
            bytes32(
                0x0235ddfda84d839acfd81a1528df081c61cbb4e3034da7ca653fb2d044dcc4b0
            ),
            bytes32(
                0x0175f018aec73d2552833ab2f7e18b47bde74d0692033bbd525725bda6145e07
            ),
            bytes32(
                0x0c4d14f89db6f49712ae1e97536dbfbf3624492987bea9145c8263db3a3fede2
            ),
            bytes32(
                0x028f0bdd9ab38bbeaf9e34ddda2d49d7ac3bba1bef6ab001973cf77fa6008a2b
            ),
            bytes32(
                0x028deba5fbba3e357ddc2e565d77c0ed23fccd47e1160ba353b4039ba0f81a0a
            )
        ];
        bytes32[4] memory input = [
            bytes32(
                0x04555e588f5368a73a1f17ac707228c1d9de799a040439bcb2b3c460ba410f61
            ),
            bytes32(
                0x1c08790b2727738d6d86858ce7b584cb2a9f24b1911aac93ed5f66d9c01f0fda
            ),
            bytes32(
                0x0a6c6588fa01171b200740344d354e8548b7470061fb32a34f4feee470ec281f
            ),
            bytes32(
                0x07586e98fad27da0b9968bc039a1ef34c939b9b8e523a8bef89d478608c5ecf6
            )
        ];
        uint256[4] memory inputConverted;
        for (uint256 i = 0; i < 4; i++) {
            inputConverted[i] = uint256(input[i]);
        }
        uint256[8] memory proofConverted;
        for (uint256 i = 0; i < 8; i++) {
            proofConverted[i] = uint256(proof[i]);
        }
        verifier.verifyProof(proofConverted, inputConverted);
    }
}
