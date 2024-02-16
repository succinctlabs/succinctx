package succinct

import (
	"fmt"
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint"
)

type CircuitBuild struct {
	pk   groth16.ProvingKey
	vk   groth16.VerifyingKey
	r1cs constraint.ConstraintSystem
}

// Export exports the R1CS, proving key, and verifying key to files.
func (build *CircuitBuild) Export() {
	// Make build directory.
	err := os.MkdirAll("build", 0755)
	if err != nil {
		fmt.Printf("Failed to create directory: %v\n", err)
		return
	}

	// Write R1CS.
	r1csFile, err := os.Create("build/r1cs.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer r1csFile.Close()

	_, err = build.r1cs.WriteTo(r1csFile)
	if err != nil {
		fmt.Println("Failed to write data:", err)
		return
	}

	// Create the proving key file.
	pkFile, err := os.Create("build/pkey.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer pkFile.Close()

	// Write proving key.
	_, err = build.pk.WriteTo(pkFile)
	if err != nil {
		fmt.Println("Failed to write data:", err)
		return
	}

	// Write verification key.
	vkFile, err := os.Create("build/vkey.bin")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer vkFile.Close()

	_, err = build.vk.WriteTo(vkFile)
	if err != nil {
		fmt.Println("Failed to write data:", err)
		return
	}

	// Write verifier smart contract into a file.
	verifierFile, err := os.Create("build/FunctionVerifier.sol")
	if err != nil {
		fmt.Println("Failed to create file:", err)
		return
	}
	defer verifierFile.Close()

	svk := &SuccinctVerifyingKey{VerifyingKey: build.vk}
	err = svk.ExportIFunctionVerifierSolidity(verifierFile)
	if err != nil {
		fmt.Println("Failed to export solidity verifier:", err)
		return
	}
}

// ImportCircuitBuild imports the R1CS, proving key, and verifying key from files.
func ImportCircuitBuild() (*CircuitBuild, error) {
	r1cs := groth16.NewCS(ecc.BN254)

	// Read the proving key file.
	pkFile, err := os.Open("build/pkey.bin")
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %w", err)
	}
	defer pkFile.Close()

	// Deserialize the proving key.
	pk := groth16.NewProvingKey(ecc.BN254)
	_, err = pk.ReadFrom(pkFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read data: %w", err)
	}

	vkFile, err := os.Open("build/vkey.bin")
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %w", err)
	}
	defer vkFile.Close()

	// Deserialize the verifying key.
	vk := groth16.NewVerifyingKey(ecc.BN254)
	_, err = vk.ReadFrom(vkFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read data: %w", err)
	}

	// Read the R1CS file.
	r1csFile, err := os.Open("build/r1cs.bin")
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %w", err)
	}
	defer r1csFile.Close()

	// Deserialize the R1CS.
	_, err = r1cs.ReadFrom(r1csFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read data: %w", err)
	}

	return &CircuitBuild{
		pk:   pk,
		vk:   vk,
		r1cs: r1cs,
	}, nil
}
