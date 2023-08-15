package types

import (
	"encoding/json"
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common/hexutil"
)

type Groth16Proof struct {
	A      [2]*big.Int    `json:"a"`
	B      [2][2]*big.Int `json:"b"`
	C      [2]*big.Int    `json:"c"`
	Input  hexutil.Bytes  `json:"input"`
	Output hexutil.Bytes  `json:"output"`
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

type Fixture struct {
	Input  hexutil.Bytes `json:"input"`
	Output hexutil.Bytes `json:"output"`
}

func (f *Fixture) Export(file string) error {
	// Write the fixture to a JSON-compatible format.

	fixtureFile, err := os.Create(file)
	if err != nil {
		panic(fmt.Errorf("failed to create file: %w", err))
	}
	defer fixtureFile.Close()

	// Marshal the proof to JSON.
	jsonString, err := json.Marshal(f)
	if err != nil {
		panic(fmt.Errorf("failed to marshal output: %w", err))
	}

	// Write the proof to the file.
	_, err = fixtureFile.Write(jsonString)
	if err != nil {
		panic(fmt.Errorf("failed to write data: %w", err))
	}

	return nil
}
