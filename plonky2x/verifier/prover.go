package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"math/big"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/logger"
	"github.com/ethereum/go-ethereum/accounts/abi"
	gnark_verifier_types "github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"

	"github.com/succinctlabs/sdk/gnarkx/types"
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
	r1csFile.Close()
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
	pkFile.Close()
	elapsed = time.Since(start)
	log.Debug().Msg("Successfully loaded proving key, time: " + elapsed.String())

	return r1cs, pk, nil
}

func Prove(circuitPath string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey) (groth16.Proof, witness.Witness, error) {
	log := logger.Logger()

	verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
		gnark_verifier_types.ReadVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json"),
	)
	proofWithPis := variables.DeserializeProofWithPublicInputs(
		gnark_verifier_types.ReadProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json"),
	)

	// Circuit assignment
	assignment := &Plonky2xVerifierCircuit{
		ProofWithPis:   proofWithPis,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: frontend.Variable(0),
		InputHash:      frontend.Variable(0),
		OutputHash:     frontend.Variable(0),
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

	const fpSize = 4 * 8
	var buf bytes.Buffer
	proof.WriteRawTo(&buf)
	proofBytes := buf.Bytes()
	output := &types.Groth16Proof{}
	output.A[0] = new(big.Int).SetBytes(proofBytes[fpSize*0 : fpSize*1])
	output.A[1] = new(big.Int).SetBytes(proofBytes[fpSize*1 : fpSize*2])
	output.B[0][0] = new(big.Int).SetBytes(proofBytes[fpSize*2 : fpSize*3])
	output.B[0][1] = new(big.Int).SetBytes(proofBytes[fpSize*3 : fpSize*4])
	output.B[1][0] = new(big.Int).SetBytes(proofBytes[fpSize*4 : fpSize*5])
	output.B[1][1] = new(big.Int).SetBytes(proofBytes[fpSize*5 : fpSize*6])
	output.C[0] = new(big.Int).SetBytes(proofBytes[fpSize*6 : fpSize*7])
	output.C[1] = new(big.Int).SetBytes(proofBytes[fpSize*7 : fpSize*8])

	// abi.encode(proof.A, proof.B, proof.C)
	uint256Array, err := abi.NewType("uint256[2]", "", nil)
	if err != nil {
		log.Fatal().AnErr("Failed to create uint256[2] type", err)
	}
	uint256ArrayArray, err := abi.NewType("uint256[2][2]", "", nil)
	if err != nil {
		log.Fatal().AnErr("Failed to create uint256[2][2] type", err)
	}
	args := abi.Arguments{
		{Type: uint256Array},
		{Type: uint256ArrayArray},
		{Type: uint256Array},
	}
	encodedProofBytes, err := args.Pack(output.A, output.B, output.C)
	if err != nil {
		log.Fatal().AnErr("Failed to encode proof", err)
	}

	log.Info().Msg("Saving proof to proof.json")
	jsonProof, err := json.Marshal(types.ProofResult{
		// Output will be filled in by plonky2x CLI
		Output: []byte{},
		Proof:  encodedProofBytes,
	})
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal proof: %w", err)
	}
	proofFile, err := os.Create("proof.json")
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

	log.Info().Msg("Saving public witness to public_witness.bin")
	witnessFile, err := os.Create("public_witness.bin")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create public witness file: %w", err)
	}
	_, err = publicWitness.WriteTo(witnessFile)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to write public witness file: %w", err)
	}
	witnessFile.Close()
	log.Info().Msg("Successfully saved public witness")

	return proof, publicWitness, nil
}
