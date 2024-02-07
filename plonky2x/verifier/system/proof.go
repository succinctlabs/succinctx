package system

import (
	"encoding/json"
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common/hexutil"
	gnark_verifier_types "github.com/succinctlabs/gnark-plonky2-verifier/types"
)

type ProofResult struct {
	Proof  hexutil.Bytes `json:"proof"`
	Output hexutil.Bytes `json:"output"`
}

type Groth16Proof struct {
	A      [2]*big.Int    `json:"a"`
	B      [2][2]*big.Int `json:"b"`
	C      [2]*big.Int    `json:"c"`
	Input  hexutil.Bytes  `json:"input,omitempty"`
	Output hexutil.Bytes  `json:"output,omitempty"`
}

// Export saves the proof to a file.
func (g *Groth16Proof) Export(file string) error {
	// Write the proof to a JSON-compatible format.

	// Create the proof file.
	proofFile, err := os.Create(file)
	if err != nil {
		panic(fmt.Errorf("failed to create file: %w", err))
	}
	defer proofFile.Close()

	// Marshal the proof to JSON.
	jsonString, err := json.Marshal(g)
	if err != nil {
		panic(fmt.Errorf("failed to marshal output: %w", err))
	}

	// Write the proof to the file.
	_, err = proofFile.Write(jsonString)
	if err != nil {
		panic(fmt.Errorf("failed to write data: %w", err))
	}

	return nil
}

func GetInputHashOutputHash(proofWithPis gnark_verifier_types.ProofWithPublicInputsRaw) (*big.Int, *big.Int) {
	publicInputs := proofWithPis.PublicInputs
	if len(publicInputs) != 64 {
		panic("publicInputs must be 64 bytes")
	}
	publicInputsBytes := make([]byte, 64)
	for i, v := range publicInputs {
		publicInputsBytes[i] = byte(v & 0xFF)
	}
	inputHash := new(big.Int).SetBytes(publicInputsBytes[0:32])
	outputHash := new(big.Int).SetBytes(publicInputsBytes[32:64])
	if inputHash.BitLen() > 253 {
		panic("inputHash must be at most 253 bits")
	}
	if outputHash.BitLen() > 253 {
		panic("outputHash must be at most 253 bits")
	}
	return inputHash, outputHash
}
