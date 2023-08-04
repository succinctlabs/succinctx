package cmd

import (
	"fmt"
	"os"
	"os/exec"
	"text/template"

	"github.com/spf13/cobra"
	"github.com/succinctlabs/sdk/cli/assets"
)

// Where all the files are saved.
const dirName = "circuit"

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize a new succinct project",
	Run: func(cmd *cobra.Command, args []string) {
		initProject()
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}

func initProject() {
	circuit, err := template.ParseFS(assets.Circuit, "circuit.tmpl")
	if err != nil {
		panic(err)
	}

	main, err := template.ParseFS(assets.Main, "main.tmpl")
	if err != nil {
		panic(err)
	}

	err = os.MkdirAll(dirName, 0755)
	if err != nil {
		panic(err)
	}

	// Create or overwrite main.go in current directory
	circuitFile, err := os.Create(dirName + "/circuit.go")
	if err != nil {
		panic(err)
	}
	defer circuitFile.Close()

	mainFile, err := os.Create(dirName + "/main.go")
	if err != nil {
		panic(err)
	}
	defer mainFile.Close()

	err = circuit.Execute(circuitFile, nil)
	if err != nil {
		panic(err)
	}

	err = main.Execute(mainFile, nil)
	if err != nil {
		panic(err)
	}

	// Initialize a new Go module in the project directory
	if err := initGoModule("github.com/yourusername/yourproject"); err != nil {
		panic(err)
	}

	// Run 'go mod tidy' to set up dependencies
	if err := tidyGoModule(); err != nil {
		panic(err)
	}

	fmt.Println("Scaffold files been successfully generated.")
}

func initGoModule(modulePath string) error {
	// Check if go.mod already exists
	if _, err := os.Stat("go.mod"); err == nil {
		fmt.Println("go.mod already exists, adding package to existing module")
		return nil
	}

	cmd := exec.Command("go", "mod", "init")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func tidyGoModule() error {
	cmd := exec.Command("go", "mod", "tidy")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}
