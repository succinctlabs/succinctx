package main

import (
	"fmt"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
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
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  dummyCircuitPath,
	}
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		return nil, nil, nil, err
	}

	fmt.Println("Running circuit setup", time.Now())
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		return nil, nil, nil, err
	}

	return r1cs, pk, vk, nil
}

func SaveVerifierCircuit(path string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey, vk groth16.VerifyingKey) error {

	fmt.Println("Saving circuit constraints to", path+"/r1cs.bin")
	r1csFile, err := os.Create(path + "/r1cs.bin")
	if err != nil {
		fmt.Println("error in creating r1cs file", err)
		os.Exit(1)
	}
	r1cs.WriteTo(r1csFile)
	r1csFile.Close()
	fmt.Println("Successfully saved circuit constraints")

	fmt.Println("Saving proving key to", path+"/pk.bin")
	pkFile, err := os.Create(path + "/pk.bin")
	if err != nil {
		return err
	}
	pk.WriteRawTo(pkFile)
	pkFile.Close()
	fmt.Println("Successfully saved proving key")

	fmt.Println("Saving verifying key to", path+"/vk.bin")
	vkFile, err := os.Create(path + "/vk.bin")
	if err != nil {
		return err
	}
	vk.WriteRawTo(vkFile)
	vkFile.Close()
	fmt.Println("Successfully saved verifying key")

	return nil
}

func Prove(circuitPath string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey) (groth16.Proof, error) {
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")

	// Circuit assignment
	assignment := &Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  circuitPath,
	}

	fmt.Println("Generating witness")
	start := time.Now()
	witness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		return nil, err
	}
	fmt.Println("Successfully generated witness, time: ", time.Since(start))

	fmt.Println("Creating proof")
	start = time.Now()
	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		return nil, err
	}
	fmt.Println("Successfully created proof, time: ", time.Since(start))

	return proof, nil
}
