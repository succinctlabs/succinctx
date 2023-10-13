package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"math/big"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	plonk_bn254 "github.com/consensys/gnark/backend/plonk/bn254"
	"github.com/ethereum/go-ethereum/common/hexutil"

	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/logger"
	gnark_verifier_types "github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"

	"github.com/succinctlabs/sdk/gnarkx/types"
)

func LoadProverData(path string) (constraint.ConstraintSystem, plonk.ProvingKey, error) {
	log := logger.Logger()
	r1csFile, err := os.Open(path + "/r1cs.bin")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to open r1cs file: %w", err)
	}
	r1cs := plonk.NewCS(ecc.BN254)
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
	pk := plonk.NewProvingKey(ecc.BN254)
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

func GetInputHashOutputHash(proofWithPis gnark_verifier_types.ProofWithPublicInputsRaw) (*big.Int, *big.Int) {
	publicInputs := proofWithPis.PublicInputs
	if len(publicInputs) != 64 {
		panic("publicInputs must be 64 bytes")
	}
	publicInputsBytes := make([]byte, 64)
	for i, v := range publicInputs {
		publicInputsBytes[i] = byte(v & 0xFF)
	}
	inputHash := new(big.Int).SetBytes(publicInputsBytes[0:32])
	outputHash := new(big.Int).SetBytes(publicInputsBytes[32:64])
	if inputHash.BitLen() > 253 {
		panic("inputHash must be at most 253 bits")
	}
	if outputHash.BitLen() > 253 {
		panic("outputHash must be at most 253 bits")
	}
	return inputHash, outputHash
}

func Prove(circuitPath string, r1cs constraint.ConstraintSystem, pk plonk.ProvingKey) (plonk.Proof, witness.Witness, error) {
	log := logger.Logger()

	verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
		gnark_verifier_types.ReadVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json"),
	)
	proofWithPis := gnark_verifier_types.ReadProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")
	proofWithPisVariable := variables.DeserializeProofWithPublicInputs(proofWithPis)

	inputHash, outputHash := GetInputHashOutputHash(proofWithPis)

	// Circuit assignment
	assignment := &Plonky2xVerifierCircuit{
		ProofWithPis:   proofWithPisVariable,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: verifierOnlyCircuitData.CircuitDigest,
		InputHash:      frontend.Variable(inputHash),
		OutputHash:     frontend.Variable(outputHash),
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
	proof, err := plonk.Prove(r1cs, pk, witness)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create proof: %w", err)
	}
	elapsed = time.Since(start)
	log.Info().Msg("Successfully created proof, time: " + elapsed.String())

	_proof := proof.(*plonk_bn254.Proof)
	log.Info().Msg("Saving proof to proof.json")
	jsonProof, err := json.Marshal(types.ProofResult{
		// Output will be filled in by plonky2x CLI
		Output: []byte{},
		Proof:  _proof.MarshalSolidity(),
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

	// Write proof with all the public inputs and save to disk.
	jsonProofWithWitness, err := json.Marshal(struct {
		InputHash      hexutil.Bytes `json:"input_hash"`
		OutputHash     hexutil.Bytes `json:"output_hash"`
		VerifierDigest hexutil.Bytes `json:"verifier_digest"`
		Proof          hexutil.Bytes `json:"proof"`
	}{
		InputHash:      inputHash.Bytes(),
		OutputHash:     outputHash.Bytes(),
		VerifierDigest: (verifierOnlyCircuitData.CircuitDigest).(*big.Int).Bytes(),
		Proof:          _proof.MarshalSolidity(),
	})
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal proof with witness: %w", err)
	}
	proofFile, err = os.Create("proof_with_witness.json")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create proof_with_witness file: %w", err)
	}
	_, err = proofFile.Write(jsonProofWithWitness)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to write proof_with_witness file: %w", err)
	}
	proofFile.Close()
	log.Info().Msg("Proof with witness")
	log.Info().Msg(string(jsonProofWithWitness))
	log.Info().Msg("Successfully saved proof_with_witness")

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
