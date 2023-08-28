package main

import (
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
	r1cs.WriteTo(r1csFile)
	r1csFile.Close()
	log.Info().Msg("Successfully saved circuit constraints")

	log.Info().Msg("Saving proving key to " + path + "/pk.bin")
	pkFile, err := os.Create(path + "/pk.bin")
	if err != nil {
		return fmt.Errorf("failed to create pk file: %w", err)
	}
	pk.WriteRawTo(pkFile)
	pkFile.Close()
	log.Info().Msg("Successfully saved proving key")

	fmt.Println("Saving verifying key to", path+"/vk.bin")
	vkFile, err := os.Create(path + "/vk.bin")
	if err != nil {
		return fmt.Errorf("failed to create vk file: %w", err)
	}
	vk.WriteRawTo(vkFile)
	vkFile.Close()
	log.Info().Msg("Successfully saved verifying key")

	return nil
}

func LoadVerifierCircuit(path string) (constraint.ConstraintSystem, groth16.ProvingKey, groth16.VerifyingKey, error) {
	r1csFile, err := os.Open(path + "/r1cs.bin")
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to open r1cs file: %w", err)
	}
	r1cs := groth16.NewCS(ecc.BN254)
	_, err = r1cs.ReadFrom(r1csFile)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to read r1cs file: %w", err)
	}

	pkFile, err := os.Open(path + "/pk.bin")
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to open pk file: %w", err)
	}
	pk := groth16.NewProvingKey(ecc.BN254)
	_, err = pk.ReadFrom(pkFile)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to read pk file: %w", err)
	}

	vkFile, err := os.Open(path + "/vk.bin")
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to open vk file: %w", err)
	}
	vk := groth16.NewVerifyingKey(ecc.BN254)
	_, err = vk.ReadFrom(vkFile)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to read vk file: %w", err)
	}

	return r1cs, pk, vk, nil
}

func Prove(circuitPath string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey, vk groth16.VerifyingKey) (groth16.Proof, witness.Witness, error) {
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
	log.Info().Msg("Successfully created proof, time: " +  elapsed.String())

	publicWitness, err := witness.Public()
	if err != nil {
		return nil, nil, fmt.Errorf("failed to get public witness: %w", err)
	}
	err = groth16.Verify(proof, vk, publicWitness)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to verify proof: %w", err)
	}
	log.Info().Msg("Successfully verified proof")

	return proof, publicWitness, nil
}
