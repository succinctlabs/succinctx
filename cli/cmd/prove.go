package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var proveCmd = &cobra.Command{
	Use:   "prove",
	Short: "Generate a proof for the circuit",
	Run: func(cmd *cobra.Command, args []string) {
		proveCLI()
	},
}

func init() {
	proveCmd.Flags().StringVarP(&inputBytes, "input", "i", "", "input bytes to prove with 0x prefix")
	proveCmd.Flags().StringVarP(&inputABI, "abi", "a", "", "ABI signature of the input types, e.g. \"(uint256,address,uint8,bool,string)\"")
	proveCmd.Flags().StringVarP(&inputValues, "values", "v", "", "comma-separated values corresponding to the types in the ABI signature")
	rootCmd.AddCommand(proveCmd)
}

func proveCLI() {
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
	if err := proveCircuit(input); err != nil {
		fmt.Printf("Failed to generate a proof for the circuit: %v\n", err)
		return
	}

	fmt.Println("Proof generated successfully.")
}

// Run the generated main.go file with the --prove flag and input bytes
func proveCircuit(input string) error {
	args := []string{"run", "./circuit", "--prove", "--input", input}
	proveCmd := exec.Command("go", args...)
	proveCmd.Stdout = os.Stdout
	proveCmd.Stderr = os.Stderr
	if err := proveCmd.Run(); err != nil {
		return fmt.Errorf("failed to run proof generation: %w", err)
	}

	return nil
}
