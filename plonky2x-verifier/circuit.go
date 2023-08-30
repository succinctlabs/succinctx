package main

import (
	"fmt"
	"os"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/logger"
	"github.com/consensys/gnark/test"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)

type Plonky2xVerifierCircuit struct {
	ProofWithPis types.ProofWithPublicInputs
	VerifierData types.VerifierOnlyCircuitData

	// A digest of the verifier information of the plonky2x circuit.
	VerifierDigest frontend.Variable `gnark:"verifierDigest,public"`

	// The input hash is the hash of all onchain inputs into the function.
	InputHash frontend.Variable `gnark:"inputHash,public"`

	// The output hash is the hash of all outputs from the function.
	OutputHash frontend.Variable `gnark:"outputHash,public"`

	verifierChip *verifier.VerifierChip `gnark:"-"`
	CircuitPath  string                 `gnark:"-"`
}

func (c *Plonky2xVerifierCircuit) Define(api frontend.API) error {
	// load the common circuit data
	commonCircuitData := verifier.DeserializeCommonCircuitData(c.CircuitPath + "/common_circuit_data.json")
	// initialize the verifier chip
	c.verifierChip = verifier.NewVerifierChip(api, commonCircuitData)
	// verify the plonky2 proof
	c.verifierChip.Verify(c.ProofWithPis.Proof, c.ProofWithPis.PublicInputs, c.VerifierData, commonCircuitData)

	publicInputs := c.ProofWithPis.PublicInputs

	verifierDigestBytes := make([]frontend.Variable, 32)
	for i := range verifierDigestBytes {
		pubByte := publicInputs[i].Limb
		verifierDigestBytes[i] = pubByte
	}
	verifierDigest := frontend.Variable(0)
	for i := range verifierDigestBytes {
		verifierDigest = api.Add(verifierDigest, api.Mul(verifierDigestBytes[i], frontend.Variable(1<<(8*i))))
	}
	c.VerifierDigest = verifierDigest

	inputDigestBytes := make([]frontend.Variable, 32)
	for i := range inputDigestBytes {
		pubByte := publicInputs[i+32].Limb
		inputDigestBytes[i] = pubByte
	}
	inputDigest := frontend.Variable(0)
	for i := range inputDigestBytes {
		inputDigest = api.Add(inputDigest, api.Mul(inputDigestBytes[i], frontend.Variable(1<<(8*i))))
	}
	c.InputHash = inputDigest

	outputDigestBytes := make([]frontend.Variable, 32)
	for i := range outputDigestBytes {
		pubByte := publicInputs[i+64].Limb
		outputDigestBytes[i] = pubByte
	}
	outputDigest := frontend.Variable(0)
	for i := range outputDigestBytes {
		outputDigest = api.Add(outputDigest, api.Mul(outputDigestBytes[i], frontend.Variable(1<<(8*i))))
	}
	c.OutputHash = outputDigest

	return nil
}

func VerifierCircuitTest(circuitPath string, dummyCircuitPath string) error {
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis:   proofWithPis,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: new(frontend.Variable),
		InputHash:      new(frontend.Variable),
		OutputHash:     new(frontend.Variable),
		CircuitPath:    dummyCircuitPath,
	}

	verifierOnlyCircuitData = verifier.DeserializeVerifierOnlyCircuitData(circuitPath + "/verifier_only_circuit_data.json")
	proofWithPis = verifier.DeserializeProofWithPublicInputs(circuitPath + "/proof_with_public_inputs.json")
	witness := Plonky2xVerifierCircuit{
		ProofWithPis:   proofWithPis,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: new(frontend.Variable),
		InputHash:      new(frontend.Variable),
		OutputHash:     new(frontend.Variable),
		CircuitPath:    dummyCircuitPath,
	}
	return test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
}

func CompileVerifierCircuit(dummyCircuitPath string) (constraint.ConstraintSystem, groth16.ProvingKey, groth16.VerifyingKey, error) {
	log := logger.Logger()
	verifierOnlyCircuitData := verifier.DeserializeVerifierOnlyCircuitData(dummyCircuitPath + "/verifier_only_circuit_data.json")
	proofWithPis := verifier.DeserializeProofWithPublicInputs(dummyCircuitPath + "/proof_with_public_inputs.json")
	circuit := Plonky2xVerifierCircuit{
		ProofWithPis:   proofWithPis,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: new(frontend.Variable),
		InputHash:      new(frontend.Variable),
		OutputHash:     new(frontend.Variable),
		CircuitPath:    dummyCircuitPath,
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

	log.Info().Msg("Saving verifying key to" + path + "/vk.bin")
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
