package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var fixtureCmd = &cobra.Command{
	Use:   "fixture",
	Short: "Generate a fixture for the circuit",
	Run: func(cmd *cobra.Command, args []string) {
		fixtureCLI()
	},
}

func init() {
	fixtureCmd.Flags().StringVarP(&inputBytes, "input", "i", "", "input bytes to fixture with 0x prefix")
	fixtureCmd.Flags().StringVarP(&inputABI, "abi", "a", "", "ABI signature of the input types, e.g. \"(uint256,address,uint8,bool,string)\"")
	fixtureCmd.Flags().StringVarP(&inputValues, "values", "v", "", "comma-separated values corresponding to the ABI signature")
	rootCmd.AddCommand(fixtureCmd)
}

func fixtureCLI() {
	// Check for existence of initialized project
	if !isProjectInitialized() {
		fmt.Println("Project not initialized. Please run 'succinct init' first.")
		return
	}

	input, err := parseInput(inputBytes, inputABI, inputValues)
	if err != nil {
		fmt.Printf("Failed to parse input: %v\n", err)
		return
	}

	// Prove the circuit
	if err := fixtureCircuit(input); err != nil {
		fmt.Printf("Failed to fixture the circuit: %v\n", err)
		return
	}

	fmt.Println("Circuit fixtured successfully.")
}

// Run the generated main.go file with the --fixture flag and input bytes
func fixtureCircuit(input string) error {
	args := []string{"run", "./circuit", "--fixture", "--input", input}
	fixtureCmd := exec.Command("go", args...)
	fixtureCmd.Stdout = os.Stdout
	fixtureCmd.Stderr = os.Stderr
	if err := fixtureCmd.Run(); err != nil {
		return fmt.Errorf("failed to generate fixture for the circuit: %w", err)
	}

	return nil
}
