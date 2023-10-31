// The API for operations relating to [64]vars.Bit inside circuits.
package bits64

import (
	"github.com/succinctlabs/succinctx/gnarkx/builder"
	"github.com/succinctlabs/succinctx/gnarkx/vars"
)

type API struct {
	api builder.API
}

// Computes the xor of 64 bit arrays.
func (a *API) Xor64(in ...[64]vars.Bool) [64]vars.Bool {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 64; j++ {
			result[j] = a.api.Xor(result[j], in[i][j])
		}
	}
	return result
}

// Computes the and of 64 bit arrays.
func (a *API) And64(in ...[64]vars.Bool) [64]vars.Bool {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 64; j++ {
			result[j] = a.api.And(result[j], in[i][j])
		}
	}
	return result
}
