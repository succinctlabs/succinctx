package succinct

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

// Circuit is the interface a circuit interacting with the Succinct Hub must implement.
// These methods are used for loading witnesses into the circuit, defining constraints, and
// reading and writing data to Ethereum.
type Circuit interface {
	GetInputBytes() *[]vars.Byte
	GetOutputBytes() *[]vars.Byte
	SetWitness(inputBytes []byte)
	Assign(inputBytes []byte) error
	Define(BaseApi frontend.API) error
}
