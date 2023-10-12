package main

import (
	"fmt"
	"math/big"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/logger"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)

type Plonky2xVerifierCircuit struct {
	// A digest of the plonky2x circuit that is being verified.
	VerifierDigest frontend.Variable `gnark:"verifierDigest,public"`

	// The input hash is the hash of all onchain inputs into the function.
	InputHash frontend.Variable `gnark:"inputHash,public"`

	// The output hash is the hash of all outputs from the function.
	OutputHash frontend.Variable `gnark:"outputHash,public"`

	// Private inputs to the circuit
	ProofWithPis variables.ProofWithPublicInputs
	VerifierData variables.VerifierOnlyCircuitData

	// Circuit configuration that is not part of the circuit itself.
	CommonCircuitData types.CommonCircuitData `gnark:"-"`
}

func (c *Plonky2xVerifierCircuit) Define(api frontend.API) error {
	// initialize the verifier chip
	verifierChip := verifier.NewVerifierChip(api, c.CommonCircuitData)
	// verify the plonky2 proof
	verifierChip.Verify(c.ProofWithPis.Proof, c.ProofWithPis.PublicInputs, c.VerifierData)

	// We assume that the publicInputs have 64 bytes
	// publicInputs[0:32] is a big-endian representation of a SHA256 hash that has been truncated to 253 bits.
	// Note that this truncation happens in the `WrappedCircuit` when computing the `input_hash`
	// The reason for truncation is that we only want 1 public input on-chain for the input hash
	// to save on gas costs
	publicInputs := c.ProofWithPis.PublicInputs

	if len(publicInputs) != 64 {
		return fmt.Errorf("expected 64 public inputs, got %d", len(publicInputs))
	}

	inputDigest := frontend.Variable(0)
	for i := 0; i < 32; i++ {
		pubByte := publicInputs[31-i].Limb
		inputDigest = api.Add(inputDigest, api.Mul(pubByte, frontend.Variable(new(big.Int).Lsh(big.NewInt(1), uint(8*i)))))

	}
	api.AssertIsEqual(c.InputHash, inputDigest)

	outputDigest := frontend.Variable(0)
	for i := 0; i < 32; i++ {
		pubByte := publicInputs[63-i].Limb
		outputDigest = api.Add(outputDigest, api.Mul(pubByte, frontend.Variable(new(big.Int).Lsh(big.NewInt(1), uint(8*i)))))
	}
	api.AssertIsEqual(c.OutputHash, outputDigest)

	// We have to assert that the VerifierData we verified the proof with
	// matches the VerifierDigest public input.
	api.AssertIsEqual(c.VerifierDigest, c.VerifierData.CircuitDigest)

	return nil
}

func CompileVerifierCircuit(dummyCircuitPath string) (constraint.ConstraintSystem, groth16.ProvingKey, groth16.VerifyingKey, error) {
	log := logger.Logger()
	verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
		types.ReadVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json"),
	)
	proofWithPis := variables.DeserializeProofWithPublicInputs(
		types.ReadProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json"),
	)
	commonCircuitData := types.ReadCommonCircuitData(dummyCircuitPath + "/common_circuit_data.json")

	circuit := Plonky2xVerifierCircuit{
		ProofWithPis:      proofWithPis,
		VerifierData:      verifierOnlyCircuitData,
		VerifierDigest:    new(frontend.Variable),
		InputHash:         new(frontend.Variable),
		OutputHash:        new(frontend.Variable),
		CommonCircuitData: commonCircuitData,
	}
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("failed to compile circuit: %w", err)
	}

	log.Info().Msg("Running circuit setup")
	start := time.Now()
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		return nil, nil, nil, err
	}
	elapsed := time.Since(start)
	log.Info().Msg("Successfully ran circuit setup, time: " + elapsed.String())

	return r1cs, pk, vk, nil
}

func SaveVerifierCircuit(path string, r1cs constraint.ConstraintSystem, pk groth16.ProvingKey, vk groth16.VerifyingKey) error {
	log := logger.Logger()
	os.MkdirAll(path, 0755)
	log.Info().Msg("Saving circuit constraints to " + path + "/r1cs.bin")
	r1csFile, err := os.Create(path + "/r1cs.bin")
	if err != nil {
		return fmt.Errorf("failed to create r1cs file: %w", err)
	}
	start := time.Now()
	r1cs.WriteTo(r1csFile)
	r1csFile.Close()
	elapsed := time.Since(start)
	log.Debug().Msg("Successfully saved circuit constraints, time: " + elapsed.String())

	log.Info().Msg("Saving proving key to " + path + "/pk.bin")
	pkFile, err := os.Create(path + "/pk.bin")
	if err != nil {
		return fmt.Errorf("failed to create pk file: %w", err)
	}
	start = time.Now()
	pk.WriteRawTo(pkFile)
	pkFile.Close()
	elapsed = time.Since(start)
	log.Debug().Msg("Successfully saved proving key, time: " + elapsed.String())

	log.Info().Msg("Saving verifying key to " + path + "/vk.bin")
	vkFile, err := os.Create(path + "/vk.bin")
	if err != nil {
		return fmt.Errorf("failed to create vk file: %w", err)
	}
	start = time.Now()
	vk.WriteRawTo(vkFile)
	vkFile.Close()
	elapsed = time.Since(start)
	log.Info().Msg("Successfully saved verifying key, time: " + elapsed.String())

	return nil
}
