package succinct

import (
	"bytes"
	"fmt"
	"math/big"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/succinctlabs/succinctx/gnarkx/builder"
	"github.com/succinctlabs/succinctx/gnarkx/hash/sha256"
	"github.com/succinctlabs/succinctx/gnarkx/types"
	"github.com/succinctlabs/succinctx/gnarkx/utils/sha256utils"
	"github.com/succinctlabs/succinctx/gnarkx/vars"
)

// Circuit functions are circuits that want to be deployed as onchain functions.
type CircuitFunction struct {
	// The input hash is the hash of all onchain inputs into the function.
	InputHash vars.Variable `gnark:"inputHash,public"`

	// The output hash is the hash of all outputs from the function.
	OutputHash vars.Variable `gnark:"outputHash,public"`

	// The circuit definies the computation of the function.
	Circuit Circuit
}

// Creates a new circuit function based on a circuit that implements the Circuit interface.
func NewCircuitFunction(c Circuit) CircuitFunction {
	function := CircuitFunction{}
	function.InputHash = vars.NewVariable()
	function.OutputHash = vars.NewVariable()
	function.Circuit = c
	return function
}

// Generate and set witnesses for the circuit function. In particular, this function will set the
// input hash and output hash variables (which will be public values). Recall that all functions
// have the form f(inputs, witness) = outputs. Both inputsHash and outputsHash are h(inputs) and
// h(outputs) respectively, where h is a hash function.
func (f *CircuitFunction) SetWitness(inputBytes []byte) {
	// Set the input bytes.
	vars.SetBytes(f.Circuit.GetInputBytes(), inputBytes)

	// Assign the circuit.
	f.Circuit.SetWitness(inputBytes)

	// Set inputHash = sha256(inputBytes) && ((1 << 253) - 1).
	inputHash := sha256utils.HashAndTruncate(inputBytes, 253)
	f.InputHash.Set(inputHash)

	// Set outputHash = sha256(outputBytes) && ((1 << 253) - 1).
	outputBytes := f.Circuit.GetOutputBytes()
	outputBytesValues := vars.GetValuesUnsafe(*outputBytes)
	outputHash := sha256utils.HashAndTruncate(outputBytesValues, 253)
	f.OutputHash.Set(outputHash)
}

// Define the circuit. All circuit functions automatically constraint h(inputBytes) == inputHash
// and h(outputBytes) == outputHash.
func (f *CircuitFunction) Define(baseApi frontend.API) error {
	// Define the circuit using the Gnark standard API. Ideally, we would pass in builder.API
	// but we can't because this is handled by Gnark internally.
	f.Circuit.Define(baseApi)

	// Automatically handle the input and output hashes and assert that they must be consistent.
	api := builder.NewAPI(baseApi)
	inputHash := sha256.HashAndTruncate(*api, *f.Circuit.GetInputBytes(), 253)
	outputHash := sha256.HashAndTruncate(*api, *f.Circuit.GetOutputBytes(), 253)
	api.AssertIsEqual(f.InputHash, inputHash)
	api.AssertIsEqual(f.OutputHash, outputHash)
	return nil
}

// Build the circuit and serialize the r1cs, proving key, and verifying key to files.
func (circuit *CircuitFunction) Build() (*CircuitBuild, error) {
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, circuit)
	if err != nil {
		return nil, err
	}

	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		return nil, err
	}

	return &CircuitBuild{
		pk:   pk,
		vk:   vk,
		r1cs: r1cs,
	}, nil
}

// Generates a proof for f(inputs, witness) = outputs based on a circuit.
func (f *CircuitFunction) Prove(inputBytes []byte, build *CircuitBuild) (*types.Groth16Proof, error) {
	// Fill in the witness values.
	f.SetWitness(inputBytes)

	// Calculate the actual witness.
	witness, err := frontend.NewWitness(f, ecc.BN254.ScalarField())
	if err != nil {
		return nil, fmt.Errorf("failed to create witness: %w", err)
	}

	// Generate the proof.
	proof, err := groth16.Prove(build.r1cs, build.pk, witness)
	if err != nil {
		return nil, fmt.Errorf("failed to generate proof: %w", err)
	}

	const fpSize = 4 * 8
	var buf bytes.Buffer
	proof.WriteRawTo(&buf)
	proofBytes := buf.Bytes()
	output := &types.Groth16Proof{}
	output.A[0] = new(big.Int).SetBytes(proofBytes[fpSize*0 : fpSize*1])
	output.A[1] = new(big.Int).SetBytes(proofBytes[fpSize*1 : fpSize*2])
	output.B[0][0] = new(big.Int).SetBytes(proofBytes[fpSize*2 : fpSize*3])
	output.B[0][1] = new(big.Int).SetBytes(proofBytes[fpSize*3 : fpSize*4])
	output.B[1][0] = new(big.Int).SetBytes(proofBytes[fpSize*4 : fpSize*5])
	output.B[1][1] = new(big.Int).SetBytes(proofBytes[fpSize*5 : fpSize*6])
	output.C[0] = new(big.Int).SetBytes(proofBytes[fpSize*6 : fpSize*7])
	output.C[1] = new(big.Int).SetBytes(proofBytes[fpSize*7 : fpSize*8])

	output.Input = inputBytes
	output.Output = vars.GetValuesUnsafe(*f.Circuit.GetOutputBytes())

	return output, nil
}

// Generates a JSON fixture for use in Solidity tests with MockSuccinctGateway.sol.
func (f *CircuitFunction) GenerateFixture(inputBytes []byte) (types.Fixture, error) {
	f.SetWitness(inputBytes)
	fixture := types.Fixture{
		Input:  inputBytes,
		Output: vars.GetValuesUnsafe(*f.Circuit.GetOutputBytes()),
	}
	return fixture, nil
}
