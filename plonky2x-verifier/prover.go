package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/logger"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)

func LoadProverData(path string) (constraint.ConstraintSystem, groth16.ProvingKey, error) {
	log := logger.Logger()
	r1csFile, err := os.Open(path + "/r1cs.bin")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to open r1cs file: %w", err)
	}
	r1cs := groth16.NewCS(ecc.BN254)
	start := time.Now()
	r1csReader := bufio.NewReader(r1csFile)
	_, err = r1cs.ReadFrom(r1csReader)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to read r1cs file: %w", err)
	}
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully loaded constraint system, time: " + elapsed.String())

	pkFile, err := os.Open(path + "/pk.bin")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to open pk file: %w", err)
	}
	pk := groth16.NewProvingKey(ecc.BN254)
	start = time.Now()
	pkReader := bufio.NewReader(pkFile)
	_, err = pk.ReadFrom(pkReader)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to read pk file: %w", err)
	}
	elapsed = time.Since(start)
	log.Debug().Msg("Successfully loaded proving key, time: " + elapsed.String())

	return r1cs, pk, nil
}

func Prove(circuitPath string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey) (groth16.Proof, witness.Witness, error) {
	log := logger.Logger()

	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")

	// Circuit assignment
	assignment := &Plonky2xVerifierCircuit{
		ProofWithPis:   proofWithPis,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: new(frontend.Variable),
		InputHash:      new(frontend.Variable),
		OutputHash:     new(frontend.Variable),
		CircuitPath:    circuitPath,
	}

	log.Debug().Msg("Generating witness")
	start := time.Now()
	witness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		return nil, nil, fmt.Errorf("failed to generate witness: %w", err)
	}
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully generated witness, time: " + elapsed.String())

	log.Debug().Msg("Creating proof")
	start = time.Now()
	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create proof: %w", err)
	}
	elapsed = time.Since(start)
	log.Info().Msg("Successfully created proof, time: " + elapsed.String())

	log.Info().Msg("Saving proof to " + circuitPath + "/proof.json")
	jsonProof, err := json.Marshal(proof)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal proof: %w", err)
	}
	proofFile, err := os.Create(circuitPath + "/proof.json")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create proof file: %w", err)
	}
	_, err = proofFile.Write(jsonProof)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to write proof file: %w", err)
	}
	proofFile.Close()
	log.Info().Msg("Successfully saved proof")

	publicWitness, err := witness.Public()
	if err != nil {
		return nil, nil, fmt.Errorf("failed to get public witness: %w", err)
	}

	jsonPublicWitness, err := json.Marshal(publicWitness)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal public witness: %w", err)
	}
	log.Info().Msg("Saving public witness to " + circuitPath + "/public_witness.json")
	witnessFile, err := os.Create(circuitPath + "/public_witness.json")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create public witness file: %w", err)
	}
	_, err = witnessFile.Write(jsonPublicWitness)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to write public witness file: %w", err)
	}
	witnessFile.Close()
	log.Info().Msg("Successfully saved public witness")

	return proof, publicWitness, nil
}
