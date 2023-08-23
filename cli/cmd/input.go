package cmd

import (
	"fmt"
	"strings"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/succinctlabs/sdk/cli/utils/abi"
)

var (
	inputBytes  string
	inputABI    string
	inputValues string
)

// Parse the input bytes from the CLI flags
func parseInput(inputBytes string, inputABI string, inputValues string) (string, error) {
	if inputBytes != "" {
		return inputBytes, nil
	} else if inputABI != "" && inputValues != "" {
		values := strings.Split(inputValues, ",")
		input, err := abi.EncodePacked(inputABI, values)
		if err != nil {
			return "", fmt.Errorf("error encoding input values: %w", err)
		}
		return hexutil.Encode(input), nil
	} else {
		return "", fmt.Errorf("must provide either input bytes, or input signature + values")
	}
}
