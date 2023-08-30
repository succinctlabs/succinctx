package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/logger"
)

func LoadVerifierKey(path string) (groth16.VerifyingKey, error) {
	log := logger.Logger()
	vkFile, err := os.Open(path + "/vk.bin")
	if err != nil {
		return nil, fmt.Errorf("failed to open vk file: %w", err)
	}
	vk := groth16.NewVerifyingKey(ecc.BN254)
	start := time.Now()
	_, err = vk.ReadFrom(vkFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read vk file: %w", err)
	}
	vkFile.Close()
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully loaded verifying key, time: " + elapsed.String())

	return vk, nil
}

func LoadPublicWitness(circuitPath string) (witness.Witness, error) {
	log := logger.Logger()
	witnessFile, err := os.Open(circuitPath + "/public_witness.json")
	if err != nil {
		return nil, fmt.Errorf("failed to open public witness file: %w", err)
	}
	publicWitness, err := witness.New(ecc.BN254.ScalarField())
	if err != nil {
		return nil, fmt.Errorf("failed to create public witness: %w", err)
	}
	jsonPublicWitness, err := io.ReadAll(witnessFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read public witness file: %w", err)
	}
	err = json.Unmarshal(jsonPublicWitness, publicWitness)
	if err != nil {
		return nil, fmt.Errorf("failed to read public witness file: %w", err)
	}
	witnessFile.Close()
	log.Debug().Msg("Successfully loaded public witness")

	return publicWitness, nil
}

func LoadProof(circuitPath string) (groth16.Proof, error) {
	log := logger.Logger()
	proofFile, err := os.Open(circuitPath + "/proof.json")
	if err != nil {
		return nil, fmt.Errorf("failed to open proof file: %w", err)
	}
	proof := groth16.NewProof(ecc.BN254)
	jsonProof, err := io.ReadAll(proofFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read proof file: %w", err)
	}
	err = json.Unmarshal(jsonProof, proof)
	if err != nil {
		return nil, fmt.Errorf("failed to read proof file: %w", err)
	}
	proofFile.Close()
	log.Debug().Msg("Successfully loaded proof")

	return proof, nil
}

func ExportIFunctionVerifierSolidity(path string, vk groth16.VerifyingKey) error {
	log := logger.Logger()
	// Create a new buffer and export the VerifyingKey into it as a Solidity contract and
	// convert the buffer content to a string for further manipulation.
	buf := new(bytes.Buffer)
	err := vk.ExportSolidity(buf)
	if err != nil {
		log.Err(err).Msg("failed to export verifying key to solidity")
		return err
	}
	content := buf.String()

	contractFile, err := os.Create(path + "/FunctionVerifier.sol")
	if err != nil {
		return err
	}
	w := bufio.NewWriter(contractFile)

	// Custom replacements to make compatible with IFunctionVerifier.
	content = strings.ReplaceAll(content, "uint256[2] calldata input", "uint256[2] memory input")
	content = strings.ReplaceAll(content, "pragma solidity ^0.8.0;", "pragma solidity ^0.8.16;")
	// write the new content to the writer
	_, err = w.Write([]byte(content))
	if err != nil {
		return err
	}

	// Generate the IFunctionVerifier interface and FunctionVerifier contract.
	solidityIFunctionVerifier := `
interface IFunctionVerifier {
    function verify(bytes32 _circuitDigest, bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier, Verifier {
    function verify(bytes32 _circuitDigest, bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool) {
        (uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c) =
            abi.decode(_proof, (uint256[2], uint256[2][2], uint256[2]));

        uint256[3] memory input = [uint256(_circuitDigest), uint256(_inputHash), uint256(_outputHash)];
        input[0] = input[0] & ((1 << 253) - 1);
        input[1] = input[1] & ((1 << 253) - 1);
		input[2] = input[2] & ((1 << 253) - 1); 

        return verifyProof(a, b, c, input);
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256(abi.encode(verifyingKey()));
    }
}
`
	// write the IFunctionVerifier and FunctionVerifier to the writer

	_, err = w.Write([]byte(solidityIFunctionVerifier))
	contractFile.Close()
	return err
}
