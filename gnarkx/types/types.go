package types

import (
	"encoding/json"
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common"
)

type Groth16Proof struct {
	A          [2]*big.Int    `json:"a"`
	B          [2][2]*big.Int `json:"b"`
	C          [2]*big.Int    `json:"c"`
	InputHash  common.Hash    `json:"inputs"`
	OutputHash common.Hash    `json:"output"`
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
