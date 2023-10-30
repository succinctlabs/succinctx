// The API for operations relating to [32]vars.Bit inside circuits.
package bits32

import (
	"github.com/succinctlabs/succinctx/gnarkx/builder"
	"github.com/succinctlabs/succinctx/gnarkx/vars"
)

// An API used for operations related to [32]vars.Bit.
type API struct {
	api builder.API
}

// Creates a new bits32.API.
func NewAPI(api builder.API) API {
	return API{api: api}
}

// Computes the xor of 32 bit arrays.
func (a *API) Xor(in ...[32]vars.Bool) [32]vars.Bool {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 32; j++ {
			result[j] = a.api.Xor(result[j], in[i][j])
		}
	}
	return result
}

// Computes the and of 32 bit arrays.
func (a *API) And(in ...[32]vars.Bool) [32]vars.Bool {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 32; j++ {
			result[j] = a.api.And(result[j], in[i][j])
		}
	}
	return result
}

// Computes the not of a 32 bit array.
func (a *API) Not(i1 [32]vars.Bool) [32]vars.Bool {
	var result [32]vars.Bool
	for i := 0; i < 32; i++ {
		result[i] = a.api.Not(i1[i])
	}
	return result
}

// Computes the binary sum of two 32 bit arrays. Equivalently, think of this as addition modulo
// 2^32.
func (a *API) Add(in ...[32]vars.Bool) [32]vars.Bool {
	if len(in) == 1 {
		return in[0]
	} else {
		return a.add(in[0], a.Add(in[1:]...))
	}
}

func (a *API) add(i1, i2 [32]vars.Bool) [32]vars.Bool {
	var result [32]vars.Bool
	carry := vars.ZERO
	for i := 31; i >= 0; i-- {
		sum := a.api.Add(i1[i].Value, i2[i].Value, carry)
		sumBin := a.api.ToBinaryLE(sum, 2)
		result[i] = sumBin[0]
		carry = sumBin[1].Value
	}
	return result
}

// Rotates a 32-length bit array by a given offset to the right.
func (a *API) Rotate(i1 [32]vars.Bool, offset int) [32]vars.Bool {
	var result [32]vars.Bool
	for i := 0; i < 32; i++ {
		result[(i+offset)%len(i1)] = i1[i]
	}
	return result
}

// Shifts a 32-length bit array by a given offset to the right.
func (a *API) Shr(i1 [32]vars.Bool, offset int) [32]vars.Bool {
	var result [32]vars.Bool
	for i := 0; i < 32; i++ {
		if i < offset {
			result[i] = vars.FALSE
		} else {
			result[i] = i1[i-offset]
		}
	}
	return result
}
