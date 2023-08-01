package vars

import "github.com/consensys/gnark/frontend"

// The zero bit as a variable in a circuit. If used within APIs, it will be treated as a constant.
var ZERO_BIT = Bit{Value: 0}

// The zero bit as a variable in a circuit. If used within APIs, it will be treated as a constant.
var ONE_BIT = Bit{Value: 1}

// A variable in a circuit representing a bit. Under the hood, the value is a single field element.
type Bit struct {
	Value frontend.Variable
}

// Creates a new bit as a variable in a circuit from a boolean.
func NewBitFromBool(i1 bool) Bit {
	if i1 {
		return ONE_BIT
	}
	return ZERO_BIT
}

// Creates a new bit as a variable in a circuit.
func NewBitFromInt(i1 int) Bit {
	if (i1 != 0) && (i1 != 1) {
		panic("x must be 0 or 1")
	}
	return Bit{Value: i1}
}
