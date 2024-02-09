// Useful reference files in gnark:
// https://github.com/Consensys/gnark-solidity-checker/blob/main/cmd/templates.go
// https://github.com/Consensys/gnark/blob/cfe83dbce12428ad0b095bcc33de55c6a9121949/test/assert_solidity.go#L60-L77
package system

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	plonk_bn254 "github.com/consensys/gnark/backend/plonk/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test"
)

type PlonkProofData struct {
	Proof  string   `json:"proof"`
	Inputs []string `json:"inputs"`
}

func TestPlonk(t *testing.T) {

	range_check := true

	circuit := MyCircuit{DoRangeCheck: range_check}

	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		panic(err)
	}
	srs, err := test.NewKZGSRS(r1cs)
	if err != nil {
		panic(err)
	}
	pk, vk, err := plonk.Setup(r1cs, srs)
	if err != nil {
		panic(err)
	}

	buf := new(bytes.Buffer)
	err = vk.ExportSolidity(buf)
	if err != nil {
		panic(err)
	}
	content := buf.String()

	filename := "VerifierPlonk.sol"
	if range_check {
		filename = "VerifierPlonkRangeCheck.sol"
	}
	contractFile, err := os.Create(filename)
	if err != nil {
		panic(err)
	}
	w := bufio.NewWriter(contractFile)
	// write the new content to the writer
	_, err = w.Write([]byte(content))
	if err != nil {
		panic(err)
	}
	contractFile.Close()

	assignment := MyCircuit{
		X: 1,
		Y: 2,
		Z: 3,
	}

	witness, _ := frontend.NewWitness(&assignment, ecc.BN254.ScalarField())
	proof, _ := plonk.Prove(r1cs, pk, witness)

	const fpSize = 4 * 8

	// Taken from: https://github.com/Consensys/gnark/blob/cfe83dbce12428ad0b095bcc33de55c6a9121949/test/assert_solidity.go#L72-L74
	_proof := proof.(*plonk_bn254.Proof)
	proofStr := "0x" + hex.EncodeToString(_proof.MarshalSolidity())

	// Taken from:
	// https://github.com/Consensys/gnark/blob/cfe83dbce12428ad0b095bcc33de55c6a9121949/test/assert_solidity.go#L80-L87
	publicWitness, _ := witness.Public()
	publicWitnessBytes, _ := publicWitness.MarshalBinary()
	publicWitnessBytes = publicWitnessBytes[12:] // We cut off the first 12 bytes because they encode length information

	nbInputs := len(publicWitnessBytes) / fpSize

	inputs := make([]string, nbInputs)
	for i := 0; i < nbInputs; i++ {
		inputs[i] = "0x" + hex.EncodeToString(publicWitnessBytes[i*fpSize:(i+1)*fpSize])
	}

	// Create the data struct and populate it
	data := PlonkProofData{
		Proof:  proofStr,
		Inputs: inputs,
	}

	// Marshal the data into JSON
	jsonData, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		fmt.Println("Error marshalling to JSON:", err)
		return
	}

	filename = "plonk_proof_data.json"
	if range_check {
		filename = "plonk_proof_data_range_check.json"
	}

	// Write the JSON to a file
	err = ioutil.WriteFile(filename, jsonData, 0644)
	if err != nil {
		fmt.Println("Error writing to file:", err)
	}
}
