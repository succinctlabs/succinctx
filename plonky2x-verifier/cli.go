package main

import (
	_ "embed"
	"flag"
	"fmt"
	"os"
	"time"

	"github.com/consensys/gnark/logger"
)


func main() {
	circuitName := flag.String("circuit", "", "circuit data directory")
	proofFlag := flag.Bool("prove", false, "create a proof")
	verifyFlag := flag.Bool("verify", false, "verify a proof")
	testFlag := flag.Bool("test", false, "test the circuit")
	compileFlag := flag.Bool("compile", false, "Compile and save the universal verifier circuit")
	flag.Parse()

	log := logger.Logger()

	if *circuitName == "" {
		log.Error().Msg("please specify a circuit name")
		os.Exit(1)
	}

	log.Debug().Msg("Circuit path: " + "./data/" + *circuitName)

	if *testFlag {
		log.Debug().Msg("testing circuit")
		start := time.Now()
		err := VerifierCircuitTest("./data/"+*circuitName, "./data/dummy")
		if err != nil {
			fmt.Println("verifier test failed:", err)
			os.Exit(1)
		}
		elasped := time.Since(start)
		log.Debug().Msg("verifier test succeeded, time: " + elasped.String())
	}

	if *compileFlag {
		log.Info().Msg("compiling verifier circuit")
		r1cs, pk, vk, err := CompileVerifierCircuit("./data/dummy")
		if err != nil {
			log.Error().Msg("failed to compile verifier circuit:" + err.Error())
			os.Exit(1)
		}
		err = SaveVerifierCircuit("./build", r1cs, pk, vk)
		if err != nil {
			log.Error().Msg("failed to save verifier circuit:" + err.Error())
			os.Exit(1)
		}
	}

	if *proofFlag {
		log.Info().Msg("loading the groth16 proving key and circuit data")
		r1cs, pk, err := LoadProverData("./build")
		if err != nil {
			log.Err(err).Msg("failed to load the verifier circuit")
			os.Exit(1)
		}
		log.Info().Msg("creating the groth16 verifier proof")
		_, _, err = Prove("./data/"+*circuitName, r1cs, pk)
		if err != nil {
			log.Err(err).Msg("failed to create the proof")
			os.Exit(1)
		}

		log.Info().Msg("Successfully verified proof")
		*verifyFlag = true
	}

	if *verifyFlag {
		// log.Info().Msg("loading the proof, verifying key and public inputs")
		// vk, publicWitness, err := LoadVerifierData("./data/"+*circuitName)
		// if err != nil {
		// 	log.Err(err).Msg("failed to load the verifier data")
		// 	os.Exit(1)
		// }
		// log.Info().Msg("loading the proof")
		// proof := groth16.NewProof(ecc.BN254)
		// proofFile, err := os.Open("./data/" + *circuitName + "/proof.bin")
		// if err != nil {
		// 	log.Err(err).Msg("failed to open proof file")
		// 	os.Exit(1)
		// }
		// _, err = proof.ReadFrom(proofFile)
		// if err != nil {
		// 	log.Err(err).Msg("failed to read proof file")
		// 	os.Exit(1)
		// }
		// log.Info().Msg("verifying the proof")
		// err = groth16.Verify(proof, vk, publicWitness)
		// if err != nil {
		// 	log.Err(err).Msg("failed to verify proof")
		// 	os.Exit(1)
		// }
		// log.Info().Msg("Successfully verified proof")
	}
}
