package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var buildCmd = &cobra.Command{
	Use:   "build",
	Short: "Build and run the initialized succinct project",
	Run: func(cmd *cobra.Command, args []string) {
		buildCLI()
	},
}

func init() {
	rootCmd.AddCommand(buildCmd)
}

func buildCLI() {
	// Check for existence of initialized project
	if !isProjectInitialized() {
		fmt.Println("Project not initialized. Please run 'succinct init' first.")
		return
	}

	// Build and run the generated main.go file
	if err := buildAndRun(); err != nil {
		fmt.Printf("Failed to build and run the project: %v\n", err)
		return
	}

	fmt.Println("Project built and run successfully.")
}

func isProjectInitialized() bool {
	// Check for specific files or directories that indicate the project is initialized
	if _, err := os.Stat("circuit/main.go"); os.IsNotExist(err) {
		return false
	}
	return true
}

func buildAndRun() error {
	// Build the project
	buildCmd := exec.Command("go", "build", "-o", "project", "circuit/main.go")
	if err := buildCmd.Run(); err != nil {
		return fmt.Errorf("failed to build the project: %w", err)
	}

	// Run the built binary
	runCmd := exec.Command("./project")
	runCmd.Stdout = os.Stdout
	runCmd.Stderr = os.Stderr
	if err := runCmd.Run(); err != nil {
		return fmt.Errorf("failed to run the project: %w", err)
	}

	return nil
}
