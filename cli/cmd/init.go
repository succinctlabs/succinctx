package cmd

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"text/template"

	"github.com/spf13/cobra"
	"github.com/succinctlabs/sdk/cli/assets"
	"github.com/succinctlabs/sdk/cli/config"
)

var initCmd = &cobra.Command{
	Use:   "init <preset> [--gomodule <gomodule>] [--dir <dir>]",
	Short: "Initialize a new succinct project. Preset must be one of: " + fmt.Sprint(config.AllPresets),
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		preset := args[0]
		if _, ok := config.DefaultConfigs[config.PresetType(preset)]; !ok {
			fmt.Printf("Preset must be one of: %v\n", config.AllPresets)
			return
		}
		initCLI(config.PresetType(preset))
	},
}

var dirName *string
var moduleName *string

func init() {
	moduleName = initCmd.Flags().StringP("gomodule", "g", "", "Go module name (ex. github.com/succinctlabs/myproject)")
	dirName = initCmd.Flags().StringP("dir", "d", "circuit", "Directory to create the circuit source files in")
	rootCmd.AddCommand(initCmd)
}

func initCLI(preset config.PresetType) {
	circuit, err := template.ParseFS(assets.Circuit, "circuit.tmpl")
	if err != nil {
		panic(err)
	}

	main, err := template.ParseFS(assets.Main, "main.tmpl")
	if err != nil {
		panic(err)
	}

	err = os.MkdirAll(*dirName, 0755)
	if err != nil {
		panic(err)
	}

	// Create or overwrite main.go in current directory
	circuitFile, err := os.Create(*dirName + "/circuit.go")
	if err != nil {
		panic(err)
	}
	defer circuitFile.Close()

	mainFile, err := os.Create(*dirName + "/main.go")
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

	jsonFile, err := os.Create("succinct.json")
	if err != nil {
		panic(err)
	}
	defer jsonFile.Close()

	// TODO: read cli arg for preset
	configContent := config.DefaultConfigs[preset]
	encoder := json.NewEncoder(jsonFile)
	encoder.SetIndent("", "    ")
	encoder.SetEscapeHTML(false)
	err = encoder.Encode(configContent)

	if err := initGoModule(); err != nil {
		panic(err)
	}

	if err := getGoModule("github.com/consensys/gnark@develop"); err != nil {
		panic(err)
	}

	if err := tidyGoModule(); err != nil {
		panic(err)
	}

	fmt.Println("Scaffold files have been successfully generated.")
}

// Initialize a new Go module in the project directory
func initGoModule() error {
	// Check if go.mod already exists
	if _, err := os.Stat("go.mod"); err == nil {
		fmt.Println("go.mod already exists, adding package to existing module")
		return nil
	}

	cmd := exec.Command("go", "mod", "init", *moduleName)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// Run 'go mod tidy' to set up dependencies
func tidyGoModule() error {
	// Check if go.mod already exists
	if _, err := os.Stat("go.mod"); err != nil {
		fmt.Println("go.mod doesn't exists, skipping tidy")
		return nil
	}

	cmd := exec.Command("go", "mod", "tidy")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// Runs 'go get <module>' to add a dependency to the project
func getGoModule(module string) error {
	cmd := exec.Command("go", "get", module)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}
