package cmd

import (
	"fmt"
	"os"
	"text/template"

	"github.com/spf13/cobra"
	"github.com/succinctlabs/sdk/cli/assets"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize a new succinct project",
	Run: func(cmd *cobra.Command, args []string) {
		initializeCLI()
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}

func initializeCLI() {
	var circuitTmpl = "assets/circuit.tmpl"
	circuit, err := template.ParseFS(assets.Circuit, circuitTmpl)
	if err != nil {
		panic(err)
	}

	var mainTmpl = "assets/main.tmpl"
	main, err := template.ParseFS(assets.Main, mainTmpl)
	if err != nil {
		panic(err)
	}

	err = os.MkdirAll("circuit", 0755)
	if err != nil {
		panic(err)
	}

	// Create or overwrite main.go in current directory
	circuitFile, err := os.Create("circuit/circuit.go")
	if err != nil {
		panic(err)
	}
	defer circuitFile.Close()

	mainFile, err := os.Create("circuit/main.go")
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

	fmt.Println("Scaffold files been successfully generated.")
}
