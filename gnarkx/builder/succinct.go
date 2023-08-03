// The basic APIs (for vars.Variable, vars.Bit, vars.Byte, vars.U64, etc) needed to write circuits
// and tools for reading and writing inputs and outputs from on-chain and off-chain data.
package builder

import (
	"github.com/consensys/gnark/frontend"
)

// This is the API that we recommend developers use for writing circuits. It is a wrapper around
// the gnark frontend API. Additional methods can be accessed by importing other packages such
// as sha256 or ssz.
type API struct {
	api frontend.API
}

// Creates a new succinct.API object.
func NewAPI(api frontend.API) *API {
	return &API{api: api}
}

// Returns the underlying gnark frontend.FrontendAPI object. Most developers should not need to
// use this.
func (a *API) FrontendAPI() frontend.API {
	return a.api
}
