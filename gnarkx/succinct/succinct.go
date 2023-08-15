package succinct

import (
	"flag"
	"fmt"

	"github.com/ethereum/go-ethereum/common/hexutil"
)

func Run(c SuccinctCircuit) {
	proveFlag := flag.Bool("prove", false, "prove the circuit")
	fixtureFlag := flag.Bool("fixture", false, "generate a test fixture")
	inputStr := flag.String("input", "", "input bytes to prove with 0x prefix")
	flag.Parse()

	circuit := NewCircuitFunction(c)

	if *proveFlag {
		fmt.Println("proving circuit for input:", hexutil.Encode([]byte(*inputStr)))
		circuitBuild, err := ImportCircuitBuild()
		if err != nil {
			fmt.Println("Failed to import circuit build:", err)
			return
		}
		proof, err := circuit.Prove([]byte(*inputStr), circuitBuild)
		if err != nil {
			fmt.Println("Failed to prove circuit:", err)
		}
		err = proof.Export("proof.json")
		if err != nil {
			fmt.Println("Failed to export proof:", err)
		}
		return
	}

	if *fixtureFlag {
		fmt.Println("generating fixture for input:", hexutil.Encode([]byte(*inputStr)))
		fixture, err := circuit.GenerateFixture([]byte(*inputStr))
		if err != nil {
			fmt.Println("Failed to generate fixture:", err)
		}
		err = fixture.Export("fixture.json")
		if err != nil {
			fmt.Println("Failed to export fixture:", err)
		}
		return
	}

	fmt.Println("compiling and building circuit artifacts")
	build, err := circuit.Build()
	if err != nil {
		fmt.Println("Failed to build circuit:", err)
		return
	}
	build.Export()
}
