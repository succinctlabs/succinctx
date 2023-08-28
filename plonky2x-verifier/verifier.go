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

func Compile(dummyCircuitPath string) (constraint.ConstraintSystem, groth16.ProvingKey, groth16.VerifyingKey) {
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath:  dummyCircuitPath,
	}
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		fmt.Println("error in building circuit", err)
		os.Exit(1)
	}

	fmt.Println("Running circuit setup", time.Now())
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	return r1cs, pk, vk
}

func Save(path string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey, vk groth16.VerifyingKey) {
// 	fmt.Println("Saving proving key to", path+"/pk.bin")
// 	pk.Write(path + "/pk.bin")
// 	fmt.Println("Saving verifying key to", path+"/vk.bin")
// 	vk.Write(path + "/vk.bin")
// 
}
