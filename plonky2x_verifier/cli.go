package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	circuitName := flag.String("circuit", "", "Circuit data directory")
	proofFlag := flag.Bool("verify", false, "profile the circuit")
	testFlag := flag.Bool("test", false, "test the circuit")
	compileFlag := flag.Bool("compile", false, "Compile the universal verifier circuit")
	serializeFlag := flag.Bool("serialize", false, "Serialize the universal verifier circuit")
	flag.Parse()

	if *circuitName == "" {
		fmt.Println("Please specify a circuit name")
		os.Exit(1)
	}

	fmt.Println("Circuit path is", "./data/"+*circuitName)

	if *testFlag {
		fmt.Println("Testing circuit")
		err := VerifierCircuitTest("./data/"+*circuitName, "./data/dummy")
		if err != nil {
			fmt.Println("Verifier test failed:", err)
			os.Exit(1)
		}
		fmt.Println("Verifier test succeeded!")
	}

	if *compileFlag {
		fmt.Println("Checking for an existing verifier circuit")
		fmt.Println("Compiling verifier circuit")
		if *serializeFlag {
			fmt.Println("Serializing verifier circuit")
		}
	}

	if *proofFlag {
		fmt.Println("Verifying circuit")
	}

}
