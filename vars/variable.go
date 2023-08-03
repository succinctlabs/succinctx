package vars

import "github.com/consensys/gnark/frontend"

// The zero value as a variable in a circuit.
var ZERO = Variable{Value: 0}

// The one value as a variable in a circuit.
var ONE = Variable{Value: 1}

// The two value as a variable in a circuit.
var TWO = Variable{Value: 2}

// The three value as a variable in a circuit.
var THREE = Variable{Value: 3}

// The three value as a variable in a circuit.
var FOUR = Variable{Value: 4}

// This is the native type of the circuit. You can think of the native type as an integer modulus
// some prime. When using variables, you must be mindful of overflows. In most cases, we suggest
// using the vars.U64 type instead.
type Variable struct {
	Value frontend.Variable
}

func NewVariableFromInt(i1 int) Variable {
	return Variable{Value: frontend.Variable(i1)}
}

// Creates a new variable in a circuit from base10 string integer.
func NewVariableFromString(s string) Variable {
	return Variable{Value: frontend.Variable(s)}
}
