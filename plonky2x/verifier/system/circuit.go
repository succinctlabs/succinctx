package system

import (
	"fmt"
	"math/big"

	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)

type VerifierCircuit struct {
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

func (c *VerifierCircuit) Define(api frontend.API) error {
	// initialize the verifier chip
	verifierChip := verifier.NewVerifierChip(api, c.CommonCircuitData)
	// verify the plonky2 proofD
	// _ = verifierChip
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
