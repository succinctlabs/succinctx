package main

import (
	"bufio"
	_ "embed"
	"flag"
	"fmt"
	"os"
	"strings"

	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/logger"
)

func main() {
	os.Setenv("USE_BIT_DECOMPOSITION_RANGE_CHECK", "true") // doesn't seem to work

	circuitPath := flag.String("circuit", "", "circuit data directory")
	dataPath := flag.String("data", "", "data directory")
	proofFlag := flag.Bool("prove", false, "create a proof")
	verifyFlag := flag.Bool("verify", false, "verify a proof")
	compileFlag := flag.Bool("compile", false, "Compile and save the universal verifier circuit")
	contractFlag := flag.Bool("contract", true, "Generate solidity contract")
	systemFlag := flag.String("system", "groth16", "proving system to use (groth16, plonk)")
	flag.Parse()

	_ = systemFlag

	log := logger.Logger()

	if *circuitPath == "" {
		log.Info().Msg("no circuitPath flag found, so user must input circuitPath via stdin")
	}

	if *dataPath == "" {
		log.Error().Msg("please specify a path to data dir (where the compiled gnark circuit data will be)")
		os.Exit(1)
	}

	// var system ProvingSystem
	// if *systemFlag != "groth16" {
	// 	system = NewGroth16System()
	// }

	log.Debug().Msg("Circuit path: " + *circuitPath)
	log.Debug().Msg("Data path: " + *dataPath)

	if *compileFlag {
		log.Info().Msg("compiling verifier circuit")
		// r1cs, _, _, err := CompileVerifierCircuit("./data/dummy")
		// if err != nil {
		// 	log.Error().Msg("failed to compile verifier circuit:" + err.Error())
		// 	os.Exit(1)
		// }

		// pk, vk := getExistingKeys(*dataPath)

		// err = SaveVerifierCircuit(*dataPath, r1cs, pk, vk)
		// if err != nil {
		// 	log.Error().Msg("failed to save verifier circuit:" + err.Error())
		// 	os.Exit(1)
		// }

		vk, err := LoadVerifierKey(*dataPath)
		if err != nil {
			panic(err)
		}

		if *contractFlag {
			log.Info().Msg("generating solidity contract")
			err := ExportIFunctionVerifierSolidity(*dataPath, vk)
			if err != nil {
				log.Error().Msg("failed to generate solidity contract:" + err.Error())
				os.Exit(1)
			}
		}
	}

	if *proofFlag {
		log.Info().Msg("loading the plonk proving key, circuit data and verifying key")
		r1cs, pk, err := LoadProverData(*dataPath)
		if err != nil {
			log.Err(err).Msg("failed to load the verifier circuit")
			os.Exit(1)
		}
		vk, err := LoadVerifierKey(*dataPath)
		if err != nil {
			log.Err(err).Msg("failed to load the verifier key")
			os.Exit(1)
		}

		// If the circuitPath is "" and not provided as part of the CLI flags, then we wait
		// for user input.
		if *circuitPath == "" {
			log.Info().Msg("Waiting for user to provide circuitPath from stdin")
			reader := bufio.NewReader(os.Stdin)
			str, err := reader.ReadString('\n')
			if err != nil {
				log.Err(err).Msg("failed to parse the user provided circuitPath")
			}
			trimmed := strings.TrimSuffix(str, "\n")
			circuitPath = &trimmed
		}

		log.Info().Msg(fmt.Sprintf("Generating the proof with circuitPath %s", *circuitPath))
		proof, publicWitness, err := Prove(*circuitPath, r1cs, pk)
		if err != nil {
			log.Err(err).Msg("failed to create the proof")
			os.Exit(1)
		}

		log.Info().Msg("Verifying proof")
		err = groth16.Verify(proof, vk, publicWitness)
		if err != nil {
			log.Err(err).Msg("failed to verify proof")
			os.Exit(1)
		}
		log.Info().Msg("Successfully verified proof")
	}

	if *verifyFlag {
		log.Info().Msg("loading the proof, verifying key and public inputs")
		vk, err := LoadVerifierKey(*dataPath)
		if err != nil {
			log.Err(err).Msg("failed to load the verifier key")
			os.Exit(1)
		}
		publicWitness, err := LoadPublicWitness(*circuitPath)
		if err != nil {
			log.Err(err).Msg("failed to load the public witness")
			os.Exit(1)
		}

		proof, err := LoadProof()
		if err != nil {
			log.Err(err).Msg("failed to load the proof")
			os.Exit(1)
		}
		err = groth16.Verify(proof, vk, publicWitness)
		if err != nil {
			log.Err(err).Msg("failed to verify proof")
			os.Exit(1)
		}
		log.Info().Msg("Successfully verified proof")
	}
}
