package main

import (
	"embed"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"text/template"
)

//go:embed entrypoint.tmpl
var entrypoint embed.FS

type Config struct {
	ImportPath  string `json:"importPath"`
	PackageName string `json:"packageName"`
}

func main() {
	configFile := "succinct.json"
	config := Config{}

	// Check if succinct.json is in the current directory
	if _, err := os.Stat(configFile); err != nil {
		if os.IsNotExist(err) {
			// Create an empty succinct.json and inform the user to fill it out
			emptyConfig, _ := json.MarshalIndent(Config{}, "", "  ")
			err := ioutil.WriteFile(configFile, emptyConfig, 0644)
			if err != nil {
				panic(err)
			}
			fmt.Println("An empty succinct.json has been created. Please fill it out and rerun the command.")
			return
		}
		panic(err)
	}

	// Read from succinct.json in current directory
	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		panic(err)
	}
	err = json.Unmarshal(data, &config)
	if err != nil {
		panic(err)
	}

	var tmplFile = "entrypoint.tmpl"
	tmpl, err := template.ParseFS(entrypoint, tmplFile)
	if err != nil {
		panic(err)
	}

	err = os.MkdirAll("succinct", 0755) // 0755 is the usual permission for directories
	if err != nil {
		panic(err)
	}
	// Create or overwrite succinct/main.go in current directory
	outputFile, err := os.Create("succinct/main.go")
	if err != nil {
		panic(err)
	}
	defer outputFile.Close()

	err = tmpl.Execute(outputFile, config)
	if err != nil {
		panic(err)
	}

	fmt.Println("File succinct/main.go has been successfully generated.")
}
