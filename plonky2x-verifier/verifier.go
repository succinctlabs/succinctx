package main

import (
	"bufio"
	"fmt"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/logger"
	"github.com/consensys/gnark/test"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)

type Plonky2xVerifierCircuit struct {
	ProofWithPis types.ProofWithPublicInputs
	VerifierData types.VerifierOnlyCircuitData

	verifierChip *verifier.VerifierChip `gnark:"-"`
	CircuitPath  string                 `gnark:"-"`
}

func (c *Plonky2xVerifierCircuit) Define(api frontend.API) error {
	commonCircuitData := verifier.DeserializeCommonCircuitData(c.CircuitPath + "/common_circuit_data.json")

	c.verifierChip = verifier.NewVerifierChip(api, commonCircuitData)

	c.verifierChip.Verify(c.ProofWithPis.Proof, c.ProofWithPis.PublicInputs, c.VerifierData, commonCircuitData)

	return nil
}

func VerifierCircuitTest(circuitPath string, dummyCircuitPath string) error {
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  dummyCircuitPath,
	}

	verifierOnlyCircuitData = verifier.DeserializeVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json")
	proofWithPis = verifier.DeserializeProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")
	witness := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  dummyCircuitPath,
	}
	return test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
}

func CompileVerifierCircuit(dummyCircuitPath string) (constraint.ConstraintSystem, groth16.ProvingKey, groth16.VerifyingKey, error) {
	log := logger.Logger()
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  dummyCircuitPath,
	}
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to compile circuit: %w", err)
	}

	log.Info().Msg("Running circuit setup")
	start := time.Now()
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		return nil, nil, nil, err
	}
	elapsed := time.Since(start)
	log.Info().Msg("Successfully ran circuit setup, time: " + elapsed.String())

	return r1cs, pk, vk, nil
}

func SaveVerifierCircuit(path string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey, vk groth16.VerifyingKey) error {
	log := logger.Logger()
	os.MkdirAll(path, 0755)
	log.Info().Msg("Saving circuit constraints to " + path + "/r1cs.bin")
	r1csFile, err := os.Create(path + "/r1cs.bin")
	if err != nil {
		return fmt.Errorf("failed to create r1cs file: %w", err)
	}
	r1csWriter := bufio.NewWriter(r1csFile)
	start := time.Now()
	r1cs.WriteTo(r1csWriter)
	r1csFile.Close()
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully saved circuit constraints, time: " + elapsed.String())

	log.Info().Msg("Saving proving key to " + path + "/pk.bin")
	pkFile, err := os.Create(path + "/pk.bin")
	if err != nil {
		return fmt.Errorf("failed to create pk file: %w", err)
	}
	pkWriter := bufio.NewWriter(pkFile)
	start = time.Now()
	pk.WriteRawTo(pkWriter)
	pkFile.Close()
	elapsed = time.Since(start)
	log.Debug().Msg("Successfully saved proving key, time: " + elapsed.String())
	
    log.Info().Msg("Saving verifying key to" + path + "/vk.bin")
	vkFile, err := os.Create(path + "/vk.bin")
	if err != nil {
		return fmt.Errorf("failed to create vk file: %w", err)
	}
	vkWriter := bufio.NewWriter(vkFile)
	start = time.Now()
	vk.WriteRawTo(vkWriter)
	vkFile.Close()
	elapsed = time.Since(start)
	log.Info().Msg("Successfully saved verifying key, time: " + elapsed.String())

	return nil
}

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
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  circuitPath,
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

	publicWitness, err := witness.Public()
	if err != nil {
		return nil, nil, fmt.Errorf("failed to get public witness: %w", err)
	}

	log.Info().Msg("Saving proof to " + circuitPath + "/proof.bin")
	proofFile, err := os.Create(circuitPath + "/proof.bin")
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create proof file: %w", err)
	}
	proofWriter := bufio.NewWriter(proofFile)
	_, err = proof.WriteRawTo(proofWriter)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to write proof file: %w", err)
	}
	proofFile.Close()
	log.Info().Msg("Successfully saved proof")

	log.Info().Msg("Saving public witness to " + circuitPath + "/public_witness.bin")
	witnessFile, err := os.Create(circuitPath + "/public_witness.bin")
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