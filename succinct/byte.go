package succinct

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

func (a *API) ToByteFromBits(i1 [8]vars.Bool) vars.Byte {
	value := vars.ZERO
	power := vars.ONE
	for i := 0; i < 8; i++ {
		value = a.Add(value, a.Mul(power, i1[i].Value))
		power = a.Mul(power, vars.TWO)
	}
	return vars.Byte{Value: value}
}

func (a *API) SelectByte(selector vars.Bool, i1 vars.Byte, i2 vars.Byte) vars.Byte {
	return vars.Byte{Value: a.Select(selector, i1.Value, i2.Value)}
}

func (a *API) SelectBytes32(selector vars.Bool, i1 [32]vars.Byte, i2 [32]vars.Byte) [32]vars.Byte {
	var result [32]vars.Byte
	for i := 0; i < 32; i++ {
		result[i] = a.SelectByte(selector, i1[i], i2[i])
	}
	return result
}

func (a *API) AssertIsEqualByte(i1, i2 vars.Byte) {
	a.AssertIsEqual(i1.Value, i2.Value)
}
