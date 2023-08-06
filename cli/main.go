package main

import (
	"embed"
	"fmt"
	"os"
	"text/template"
)

//go:embed circuit.tmpl
var circuitTmpl embed.FS

//go:embed main.tmpl
var mainTmpl embed.FS

func main() {
	var cTmpl = "circuit.tmpl"
	circuit, err := template.ParseFS(circuitTmpl, cTmpl)
	if err != nil {
		panic(err)
	}

	var mTmpl = "main.tmpl"
	main, err := template.ParseFS(mainTmpl, mTmpl)
	if err != nil {
		panic(err)
	}

	err = os.MkdirAll("circuit", 0755) // 0755 is the usual permission for directories
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
