package main

import (
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"
)

func TestPlonky2xVerifierCircuit(t *testing.T) {
	assert := test.NewAssert(t)

	testCase := func() error {
		dummyCircuitPath := "./data/dummy"
		circuitPath := "./data/test_circuit"

		verifierOnlyCircuitDataDummy := variables.DeserializeVerifierOnlyCircuitData(
			types.ReadVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json"),
		)
		proofWithPisDummy := variables.DeserializeProofWithPublicInputs(
			types.ReadProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json"),
		)
		commonCircuitDataDummy := types.ReadCommonCircuitData(dummyCircuitPath + "/common_circuit_data.json")

		circuit := Plonky2xVerifierCircuit{
			ProofWithPis:      proofWithPisDummy,
			VerifierData:      verifierOnlyCircuitDataDummy,
			VerifierDigest:    new(frontend.Variable),
			InputHash:         new(frontend.Variable),
			OutputHash:        new(frontend.Variable),
			CommonCircuitData: commonCircuitDataDummy,
		}

		verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
			types.ReadVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json"),
		)
		proofWithPis := variables.DeserializeProofWithPublicInputs(
			types.ReadProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json"),
		)

		witness := Plonky2xVerifierCircuit{
			ProofWithPis:   proofWithPis,
			VerifierData:   verifierOnlyCircuitData,
			VerifierDigest: new(frontend.Variable),
			InputHash:      new(frontend.Variable),
			OutputHash:     new(frontend.Variable),
		}
		return test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
	}

	assert.NoError(testCase())
}
