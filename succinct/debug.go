package succinct

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-gadgets/vars"
)

func (a *API) PrintVarBytes(vars []vars.Byte) {
	var fvars []frontend.Variable
	for _, v := range vars {
		fvars = append(fvars, v.Value.Value)
	}
	a.api.Println(fvars)
}
