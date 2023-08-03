package succ

import (
	"bytes"
	crypto_sha256 "crypto/sha256"
	"encoding/json"
	"fmt"
	"math/big"
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint/solver"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/succinctlabs/gnark-gadgets/hash/sha256"
	"github.com/succinctlabs/gnark-gadgets/succinct"
	"github.com/succinctlabs/gnark-gadgets/types"
	"github.com/succinctlabs/gnark-gadgets/vars"
)

type SuccinctCircuit interface {
	Assign(inputBytes []byte) error
	Define(api frontend.API) error
	GetInputBytes() []vars.Byte
	GetOutputBytes() []vars.Byte
}

type OuterCircuit struct {
	InputHash  [32]vars.Byte `gnark:"input_hash,public"`
	OutputHash [32]vars.Byte `gnark:"output_hash,public"`
	Subcircuit SuccinctCircuit
}

func NewOuterCircuit(s SuccinctCircuit) OuterCircuit {
	circuit := OuterCircuit{}
	circuit.InputHash = vars.NewBytes32()
	circuit.OutputHash = vars.NewBytes32()
	circuit.Subcircuit = s
	return circuit
}

func (circuit *OuterCircuit) Assign(inputBytes []byte) error {
	circuit.Subcircuit.Assign(inputBytes)

	h := crypto_sha256.New()
	h.Write(inputBytes)
	computedInputHash := h.Sum(nil)
	var sizedInputHash [32]byte
	copy(sizedInputHash[:], computedInputHash)
	vars.Bytes32(circuit.InputHash).Set(sizedInputHash)

	outputBytes := circuit.Subcircuit.GetOutputBytes()
	outputBytesValues := vars.Bytes(outputBytes).GetValue()
	h = crypto_sha256.New()
	h.Write(outputBytesValues)
	computedOutputHash := h.Sum(nil)
	var sizedOutputHash [32]byte
	copy(sizedOutputHash[:], computedOutputHash)
	vars.Bytes32(circuit.OutputHash).Set(sizedOutputHash)

	return nil
}

func (circuit *OuterCircuit) Define(baseApi frontend.API) error {
	api := succinct.NewAPI(baseApi)
	circuit.Subcircuit.Define(baseApi)
	computedInputHash := sha256.Hash(*api, circuit.Subcircuit.GetInputBytes())
	computedOutputHash := sha256.Hash(*api, circuit.Subcircuit.GetOutputBytes())
	// printVarBytes(baseApi, computedOutputHash[:])
	for i := 0; i < 32; i++ {
		api.AssertIsEqualByte(circuit.InputHash[i], computedInputHash[i])
		api.AssertIsEqualByte(circuit.OutputHash[i], computedOutputHash[i])
	}
	return nil
}

func (circuit *OuterCircuit) Prove(inputBytes []byte) {
	r1cs := groth16.NewCS(ecc.BN254)

	// Read proving key.
	pkFile, err := os.Open("build/pkey.bin")
	if err != nil {
		fmt.Println("Failed to open file:", err)
		return
	}
	defer pkFile.Close()

	pk := groth16.NewProvingKey(ecc.BN254)
	_, err = pk.ReadFrom(pkFile)
	if err != nil {
		fmt.Println("Failed to read data:", err)
		return
	}

	r1csFile, err := os.Open("build/r1cs.bin")
	if err != nil {
		fmt.Println("Failed to open file:", err)
		return
	}
	defer r1csFile.Close()

	_, err = r1cs.ReadFrom(r1csFile)
	if err != nil {
		fmt.Println("Failed to read data:", err)
		return
	}

	solver.RegisterHint()

	circuit.Assign(inputBytes)

	witness, err := frontend.NewWitness(circuit, ecc.BN254.ScalarField())
	if err != nil {
		fmt.Println("Failed to create witness:", err)
		return
	}

	fmt.Println("proving")
	proof, err := groth16.Prove(r1cs, pk, witness)

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

	jsonString, err := json.Marshal(output)
	fmt.Println(string(jsonString))
	// write to file
	proofFile, err := os.Create("proof.json")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer proofFile.Close()

	_, err = proofFile.Write(jsonString)
	return
}

func (circuit *OuterCircuit) Build() {
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

	// Write proving key.
	pkFile, err := os.Create("build/pkey.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer pkFile.Close()

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
}
