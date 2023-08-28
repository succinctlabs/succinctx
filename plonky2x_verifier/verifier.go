package plonky2xverifier

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/verifier"
)


type Plonky2xVerifierCircuit struct {
	ProofWithPis        types.ProofWithPublicInputs
	VerifierData types.VerifierOnlyCircuitData

	verifierChip       *verifier.VerifierChip `gnark:"-"`
	CircuitPath string                 `gnark:"-"`
}


func (c *Plonky2xVerifierCircuit) Define(api frontend.API) error {
	commonCircuitData := verifier.DeserializeCommonCircuitData(c.CircuitPath + "/common_circuit_data.json")

	c.verifierChip = verifier.NewVerifierChip(api, commonCircuitData)

	c.verifierChip.Verify(c.ProofWithPis.Proof, c.ProofWithPis.PublicInputs, c.VerifierData, commonCircuitData)

	return nil
}