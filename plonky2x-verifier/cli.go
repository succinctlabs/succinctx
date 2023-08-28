package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/consensys/gnark/logger"
)

func main() {
	circuitName := flag.String("circuit", "", "Circuit data directory")
	proofFlag := flag.Bool("verify", false, "profile the circuit")
	testFlag := flag.Bool("test", false, "test the circuit")
	compileFlag := flag.Bool("compile", false, "Compile and save the universal verifier circuit")
	flag.Parse()

	log := logger.Logger()

	if *circuitName == "" {
		log.Error().Msg("Please specify a circuit name")
		os.Exit(1)
	}

	fmt.Println("Circuit path is", "./data/"+*circuitName)

	if *testFlag {
		log.Debug().Msg("Testing circuit")
		err := VerifierCircuitTest("./data/"+*circuitName, "./data/dummy")
		if err != nil {
			fmt.Println("Verifier test failed:", err)
			os.Exit(1)
		}
		log.Debug().Msg("Verifier test succeeded!")
	}

	if *compileFlag {
		fmt.Println("Compiling verifier circuit")
		r1cs, pk, vk, err := CompileVerifierCircuit("./data/dummy")
		if err != nil {
		    log.Error().Msg("Failed to compile verifier circuit:" + err.Error())
			os.Exit(1)
		}
		err = SaveVerifierCircuit("./build", r1cs, pk, vk)
		if err != nil {
			log.Error().Msg("Failed to save verifier circuit:" + err.Error())
			os.Exit(1)
		}
	}

	if *proofFlag {
		log.Info().Msg("Verifying circuit")
	}

}
