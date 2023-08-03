package succinct

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

// Computes the or of two bits or i1 | i2.
func (a *API) Or(i1, i2 vars.Bool) vars.Bool {
	return vars.Bool{Value: a.Add(i1.Value, i2.Value)}
}

// Computes the and of two bits or i1 & i2.
func (a *API) And(i1, i2 vars.Bool) vars.Bool {
	return vars.Bool{Value: a.Mul(i1.Value, i2.Value)}
}

// Computes the xor of two bits or i1 ^ i2.
func (a *API) Xor(i1, i2 vars.Bool) vars.Bool {
	return vars.Bool{Value: vars.Variable{Value: a.api.Xor(i1.Value.Value, i2.Value.Value)}}
}

// Computes the not of a bit or !i1.
func (a *API) Not(i1 vars.Bool) vars.Bool {
	return vars.Bool{Value: a.Sub(vars.ONE, i1.Value)}
}

// Decomposes a variable in the circuit into a number of bits with little-endian ordering. This
// function can also be used for "range-checking", in other words checking that some value
// is less than 2**n.
func (a *API) ToBinaryLE(i1 vars.Variable, nbBits int) []vars.Bool {
	values := a.api.ToBinary(i1.Value, nbBits)
	bits := make([]vars.Bool, nbBits)
	for i := 0; i < nbBits; i++ {
		bits[i] = vars.Bool{Value: vars.Variable{Value: values[i]}}
	}
	return bits
}

// Decomposes a variable in the circuit into a number of bits with big-endian ordering. This
// function can also be used for "range-checking", in other words checking that some value
// is less than 2**n.
func (a *API) ToBinaryBE(i1 vars.Variable, nbBits int) []vars.Bool {
	values := a.api.ToBinary(i1.Value, nbBits)
	bits := make([]vars.Bool, nbBits)
	for i := 0; i < nbBits; i++ {
		bits[nbBits-i-1] = vars.Bool{Value: vars.Variable{Value: values[i]}}
	}
	return bits
}

// Asserts that two booleans are equal.
func (a *API) AssertIsEqualBool(i1, i2 vars.Bool) {
	a.AssertIsEqual(i1.Value, i2.Value)
}

// Converts a byte to bits with little-endian ordering.
func (a *API) ToBitsFromByte(i1 vars.Byte) [8]vars.Bool {
	values := a.ToBinaryLE(i1.Value, 8)
	var bits [8]vars.Bool
	for i := 0; i < 8; i++ {
		bits[i] = values[i]
	}
	return bits
}
