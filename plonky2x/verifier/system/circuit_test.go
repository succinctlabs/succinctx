package system

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

	testCase := func(option int64) error {
		dummyCircuitPath := "./data/dummy"
		circuitPath := "./data/test_circuit"

		verifierOnlyCircuitDataDummy := variables.DeserializeVerifierOnlyCircuitData(
			types.ReadVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json"),
		)
		proofWithPisDummy := variables.DeserializeProofWithPublicInputs(
			types.ReadProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json"),
		)
		commonCircuitDataDummy := types.ReadCommonCircuitData(dummyCircuitPath + "/common_circuit_data.json")

		circuit := VerifierCircuit{
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

		witness := VerifierCircuit{
			ProofWithPis:   proofWithPisVariable,
			VerifierData:   verifierOnlyCircuitData,
			VerifierDigest: verifierOnlyCircuitData.CircuitDigest,
			InputHash:      frontend.Variable(inputHash),
			OutputHash:     frontend.Variable(outputHash),
		}

		// When an option is turned on, we fuzz the different witness values to check
		// that the circuit proof generation will be invalid when the inputs are incorrect.
		if option == 1 {
			witness.InputHash = frontend.Variable(0)
		} else if option == 2 {
			witness.OutputHash = frontend.Variable(0)
		} else if option == 3 {
			witness.VerifierDigest = verifierOnlyCircuitDataDummy.CircuitDigest
		} else if option == 4 {
			witness.VerifierDigest = frontend.Variable(0)
		} else if option == 5 {
			witness.ProofWithPis = proofWithPisDummy
		} else if option == 6 {
			witness.VerifierData = verifierOnlyCircuitDataDummy
		} else if option == 7 {
			witness.ProofWithPis = proofWithPisDummy
			witness.VerifierData = verifierOnlyCircuitDataDummy
			witness.VerifierDigest = verifierOnlyCircuitDataDummy.CircuitDigest
		} else if option == 8 {
			// Fuzz random parts of the proof
			proofWithPis.Proof.OpeningProof.FinalPoly.Coeffs[0][0] = 0
			witness.ProofWithPis = variables.DeserializeProofWithPublicInputs(
				proofWithPis,
			)
		}
		return test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
	}

	assert.NoError(testCase(0))
	for i := 1; i <= 8; i++ {
		assert.Error(testCase(int64(i)))
	}
}
