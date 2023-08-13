package builder

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

func (a *API) PrintVarBytes(vars []vars.Byte) {
	var fvars []frontend.Variable
	for _, v := range vars {
		fvars = append(fvars, v.Value.Value)
	}
	a.api.Println(fvars)
}

func (a *API) PrintU64(u64 vars.U64) {
	a.api.Println(u64.Value)
}
