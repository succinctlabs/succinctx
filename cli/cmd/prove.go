package cmd

import (
	"fmt"
	"os/exec"

	"github.com/spf13/cobra"
)

var proveCmd = &cobra.Command{
	Use:   "prove",
	Short: "Prove the circuit",
	Run: func(cmd *cobra.Command, args []string) {
		proveCLI()
	},
}

func init() {
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
	// Run the generated main.go file with the --prove flag
	proveCmd := exec.Command("go", "run", "./circuit", "--prove")
	if err := proveCmd.Run(); err != nil {
		return fmt.Errorf("failed to prove the circuit: %w", err)
	}

	return nil
}
