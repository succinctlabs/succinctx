package succinct

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/big"
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint/solver"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/succinctlabs/sdk/gnarkx/builder"
	"github.com/succinctlabs/sdk/gnarkx/hash/sha256"
	"github.com/succinctlabs/sdk/gnarkx/types"
	"github.com/succinctlabs/sdk/gnarkx/utils/sha256utils"
	"github.com/succinctlabs/sdk/gnarkx/vars"
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

// The interface a circuit interacting with the Succinct Hub must implement. These methods are used
// for loading witnesses into the circuit, defining constraints, and reading and writing data to
// Ethereum.
type Circuit interface {
	SetWitness(inputBytes []byte)
	Define(api frontend.API) error
	GetInputBytes() *[]vars.Byte
	GetOutputBytes() *[]vars.Byte
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
	fmt.Println("inputHash", inputHash)
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
	// but we can't becaues this is handled by Gnark internally.
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
func (circuit *CircuitFunction) Build() {
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, circuit)
	if err != nil {
		panic(err)
	}

	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		panic(err)
	}

	// Make build directory.
	err = os.MkdirAll("build", 0755)
	if err != nil {
		fmt.Printf("Failed to create directory: %v\n", err)
		return
	}

	// Write R1CS.
	r1csFile, err := os.Create("build/r1cs.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer r1csFile.Close()

	_, err = r1cs.WriteTo(r1csFile)
	if err != nil {
		fmt.Println("Failed to write data:", err)
		return
	}

	// Create the proving key file.
	pkFile, err := os.Create("build/pkey.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer pkFile.Close()

	// Write proving key.
	_, err = pk.WriteTo(pkFile)
	if err != nil {
		fmt.Println("Failed to write data:", err)
		return
	}

	// Write verification key.
	vkFile, err := os.Create("build/vkey.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer vkFile.Close()

	_, err = vk.WriteTo(vkFile)
	if err != nil {
		fmt.Println("Failed to write data:", err)
		return
	}

	// Write verifier smart contract into a file.
	verifierFile, err := os.Create("build/FunctionVerifier.sol")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer verifierFile.Close()

	svk := &SuccinctVerifyingKey{VerifyingKey: vk}
	err = svk.ExportIFunctionVerifierSolidity(verifierFile)
	if err != nil {
		fmt.Println("Failed to export solidity verifier:", err)
		return
	}

}

// Generates a proof for f(inputs, witness) = outputs based on a circuit.
func (f *CircuitFunction) Prove(inputBytes []byte) {
	r1cs := groth16.NewCS(ecc.BN254)

	// Read the proving key file.
	pkFile, err := os.Open("build/pkey.bin")
	if err != nil {
		panic(fmt.Errorf("failed to open file: %w", err))
	}
	defer pkFile.Close()

	// Deserialize the proving key.
	pk := groth16.NewProvingKey(ecc.BN254)
	_, err = pk.ReadFrom(pkFile)
	if err != nil {
		panic(fmt.Errorf("failed to read data: %w", err))
	}

	// Read the R1CS file.
	r1csFile, err := os.Open("build/r1cs.bin")
	if err != nil {
		panic(fmt.Errorf("failed to open file: %w", err))
	}
	defer r1csFile.Close()

	// Deserialize the R1CS.
	_, err = r1cs.ReadFrom(r1csFile)
	if err != nil {
		panic(fmt.Errorf("failed to read data: %w", err))
	}

	// Register hints which are used for automatic constraint generation.
	solver.RegisterHint()

	// Create the witness.
	f.SetWitness(inputBytes)

	// Calculate the rest of the witness.
	witness, err := frontend.NewWitness(f, ecc.BN254.ScalarField())
	if err != nil {
		panic(fmt.Errorf("failed to create witness: %w", err))
	}

	// Generate the proof.
	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		panic(fmt.Errorf("failed to generate proof: %w", err))
	}

	// Write the proof to a JSON-compatible format.
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

	// Create the proof file.
	proofFile, err := os.Create("proof.json")
	if err != nil {
		panic(fmt.Errorf("failed to create file: %w", err))
	}
	defer proofFile.Close()

	// Marshal the proof to JSON.
	jsonString, err := json.Marshal(output)
	if err != nil {
		panic(fmt.Errorf("failed to marshal output: %w", err))
	}

	// Write the proof to the file.
	_, err = proofFile.Write(jsonString)
	if err != nil {
		panic(fmt.Errorf("failed to write data: %w", err))
	}
}
