package succinct

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

// SuccinctCircuit is the interface that circuits using the Succinct SDK must implement.
type SuccinctCircuit interface {
	GetInputBytes() *[]vars.Byte
	GetOutputBytes() *[]vars.Byte
	SetWitness(inputBytes []byte)
	Assign(inputBytes []byte) error
	Define(BaseApi frontend.API) error
}
