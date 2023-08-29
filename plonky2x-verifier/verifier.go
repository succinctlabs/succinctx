package main

import (
	"bufio"
	"fmt"
	"os"
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
		return nil,  fmt.Errorf("failed to read vk file: %w", err)
	}
	vkFile.Close()
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully loaded verifying key, time: " + elapsed.String())

	return vk, nil
}

func LoadPublicWitness(circuitPath string) (witness.Witness, error) {
	log := logger.Logger()
	witnessFile, err := os.Open(circuitPath + "/public_witness.bin")
	if err != nil {
		return nil, fmt.Errorf("failed to open public witness file: %w", err)
	}
	publicWitness, err := witness.New(ecc.BN254.ScalarField())
	if err != nil {
		return nil, fmt.Errorf("failed to create public witness: %w", err)
	}
	start := time.Now()
	_, err = publicWitness.ReadFrom(witnessFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read public witness file: %w", err)
	}
	witnessFile.Close()
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully loaded public witness, time: " + elapsed.String())

	return publicWitness, nil
}

func LoadProof(circuitPath string) (groth16.Proof, error) {
	log := logger.Logger()
	proofFile, err := os.Open(circuitPath + "/proof.bin")
	if err != nil {
		return nil, fmt.Errorf("failed to open proof file: %w", err)
	}
	proof := groth16.NewProof(ecc.BN254)
	proofReader := bufio.NewReader(proofFile)
	start := time.Now()
	_, err = proof.ReadFrom(proofReader)
	if err != nil {
		return nil, fmt.Errorf("failed to read proof file: %w", err)
	}
	proofFile.Close()
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully loaded proof, time: " + elapsed.String())

	return proof, nil
}