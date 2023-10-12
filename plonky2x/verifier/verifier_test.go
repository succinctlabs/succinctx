package main

import (
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"
)

// To run this test, you must first populate the data directory by running the following test
// in plonky2x: cargo test test_wrapper -- --nocapture
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
			VerifierDigest:    frontend.Variable(0), // Can be empty for defining the circuit
			InputHash:         frontend.Variable(0),
			OutputHash:        frontend.Variable(0),
			CommonCircuitData: commonCircuitDataDummy,
		}

		verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
			types.ReadVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json"),
		)
		proofWithPis := types.ReadProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")
		inputHash, outputHash := GetInputHashOutputHash(proofWithPis)

		proofWithPisVariable := variables.DeserializeProofWithPublicInputs(proofWithPis)

		witness := Plonky2xVerifierCircuit{
			ProofWithPis:   proofWithPisVariable,
			VerifierData:   verifierOnlyCircuitData,
			VerifierDigest: verifierOnlyCircuitData.CircuitDigest,
			InputHash:      frontend.Variable(inputHash),
			OutputHash:     frontend.Variable(outputHash),
		}
		return test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
	}

	assert.NoError(testCase())
}

func TestPlonky2xVerifierCircuitFails(t *testing.T) {
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
			VerifierDigest:    frontend.Variable(0), // Can be empty for defining the circuit
			InputHash:         frontend.Variable(0),
			OutputHash:        frontend.Variable(0),
			CommonCircuitData: commonCircuitDataDummy,
		}

		verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
			types.ReadVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json"),
		)
		proofWithPis := types.ReadProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")
		inputHash, outputHash := GetInputHashOutputHash(proofWithPis)

		proofWithPisVariable := variables.DeserializeProofWithPublicInputs(proofWithPis)

		witness := Plonky2xVerifierCircuit{
			ProofWithPis:   proofWithPisVariable,
			VerifierData:   verifierOnlyCircuitData,
			VerifierDigest: verifierOnlyCircuitData.CircuitDigest,
			InputHash:      frontend.Variable(inputHash),
			OutputHash:     frontend.Variable(outputHash),
		}
		return test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
	}

	assert.NoError(testCase())
}
