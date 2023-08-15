package types

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/ethereum/go-ethereum/common/hexutil"
)

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

	// Marshal the fixture to JSON.
	jsonString, err := json.Marshal(f)
	if err != nil {
		panic(fmt.Errorf("failed to marshal output: %w", err))
	}

	// Write the fixture to the file.
	_, err = fixtureFile.Write(jsonString)
	if err != nil {
		panic(fmt.Errorf("failed to write data: %w", err))
	}

	return nil
}
