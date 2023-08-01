package vars

import (
	"math/big"

	"github.com/consensys/gnark/frontend"
)

// A variable in a circuit representing a u128.
type U128 struct {
	Value frontend.Variable
}

// Creates a new u128 as a variable in a circuit.
func NewU128(i1 *big.Int) U128 {
	return U128{Value: frontend.Variable(i1)}
}
