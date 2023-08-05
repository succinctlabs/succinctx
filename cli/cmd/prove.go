package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var inputBytesString string

var proveCmd = &cobra.Command{
	Use:   "prove",
	Short: "Generate a proof for the circuit",
	Run: func(cmd *cobra.Command, args []string) {
		proveCLI()
	},
}

func init() {
	proveCmd.Flags().StringVarP(&inputBytesString, "input", "i", "", "input bytes to prove with 0x prefix")
	rootCmd.AddCommand(proveCmd)
}

func proveCLI() {
	// Check for existence of initialized project
	if !isProjectInitialized() {
		fmt.Println("Project not initialized. Please run 'succinct init' first.")
		return
	}

	// Prove the circuit
	if err := proveCircuit(); err != nil {
		fmt.Printf("Failed to prove the circuit: %v\n", err)
		return
	}

	fmt.Println("Circuit proved successfully.")
}

func proveCircuit() error {
	// Run the generated main.go file with the --prove flag and optional input bytes
	args := []string{"run", "./circuit", "--prove"}
	if inputBytesString != "" {
		args = append(args, "--input", inputBytesString)
	}
	proveCmd := exec.Command("go", args...)
	proveCmd.Stdout = os.Stdout
	proveCmd.Stderr = os.Stderr
	if err := proveCmd.Run(); err != nil {
		return fmt.Errorf("failed to prove the circuit: %w", err)
	}

	return nil
}
