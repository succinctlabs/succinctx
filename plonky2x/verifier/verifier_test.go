package main

import (
	"bufio"
	"bytes"
	"os"
	"testing"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/std/rangecheck"
	"github.com/consensys/gnark/test"
	"github.com/rs/zerolog/log"
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

type MyCircuit struct {
	X frontend.Variable `gnark:",public"`
	Y frontend.Variable `gnark:",public"`
	Z frontend.Variable `gnark:",public"`
	A frontend.Variable `gnark:"-"`
}

func (circuit *MyCircuit) Define(api frontend.API) error {
	api.AssertIsEqual(circuit.Z, api.Add(circuit.X, circuit.Y))
	rangeChecker := rangecheck.New(api)
	rangeChecker.Check(circuit.X, 100)
	return nil
}

func TestSanity(t *testing.T) {
	circuit := MyCircuit{}

	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		panic(err)
	}

	log.Info().Msg("Running circuit setup")
	start := time.Now()
	_, vk, err := groth16.Setup(r1cs)
	if err != nil {
		panic(err)
	}
	elapsed := time.Since(start)
	log.Info().Msg("Successfully ran circuit setup, time: " + elapsed.String())

	buf := new(bytes.Buffer)
	err = vk.ExportSolidity(buf)
	if err != nil {
		panic(err)
	}
	content := buf.String()

	contractFile, err := os.Create("VerifierSanity6.sol")
	if err != nil {
		panic(err)
	}
	w := bufio.NewWriter(contractFile)
	// write the new content to the writer
	_, err = w.Write([]byte(content))
	if err != nil {
		panic(err)
	}

	contractFile.Close()
}
