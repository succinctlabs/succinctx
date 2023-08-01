package vars

import "github.com/consensys/gnark/frontend"

// A variable in a circuit representing a u64.
type U64 struct {
	Value frontend.Variable
}

// Creates a new u64 as a variable in a circuit.
func NewU64(i1 uint64) U64 {
	return U64{Value: int(i1)}
}
