package main

import (
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/test"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)

func TestPlonky2xVerifierCircuit(t *testing.T) {
	assert := test.NewAssert(t)

	testCase := func() {
	dummyCircuitPath := "./data/dummy"
	circuitPath := "./data/test_circuit"

	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath: dummyCircuitPath,
	}

	verifierOnlyCircuitData = verifier.DeserializeVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json")
	proofWithPis = verifier.DeserializeProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")
	witness := Plonky2xVerifierCircuit{
		ProofWithPis: proofWithPis,
		VerifierData: verifierOnlyCircuitData,
		CircuitPath: dummyCircuitPath,
	}
	err := test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
	assert.NoError(err)
}
testCase()
}