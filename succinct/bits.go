package succinct

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-gadgets/vars"
)

func (a *API) ToBinaryLE(i1 frontend.Variable, n int) []vars.Bit {
	values := a.api.ToBinary(i1, n)
	bits := make([]vars.Bit, n)
	for i := 0; i < n; i++ {
		bits[i] = vars.Bit{Value: values[i]}
	}
	return bits
}

func (a *API) ToBinaryBE(i1 frontend.Variable, n int) []vars.Bit {
	values := a.api.ToBinary(i1, n)
	bits := make([]vars.Bit, n)
	for i := 0; i < n; i++ {
		bits[n-i-1] = vars.Bit{Value: values[i]}
	}
	return bits
}

func (a *API) Xor(i1, i2 vars.Bit) vars.Bit {
	return vars.Bit{Value: a.api.Xor(i1.Value, i2.Value)}
}

func (a *API) Xor32(in ...[32]vars.Bit) [32]vars.Bit {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 32; j++ {
			result[j] = a.Xor(result[j], in[i][j])
		}
	}
	return result
}

func (a *API) Xor64(in ...[64]vars.Bit) [64]vars.Bit {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 64; j++ {
			result[j] = a.Xor(result[j], in[i][j])
		}
	}
	return result
}

func (a *API) And(i1, i2 vars.Bit) vars.Bit {
	return vars.Bit{Value: a.api.And(i1.Value, i2.Value)}
}

func (a *API) And32(in ...[32]vars.Bit) [32]vars.Bit {
	if len(in) < 2 {
		panic("invalid number of arguments")
	}
	result := in[0]
	for i := 1; i < len(in); i++ {
		for j := 0; j < 32; j++ {
			result[j] = a.And(result[j], in[i][j])
		}
	}
	return result
}

func (a *API) Rotate32(i1 [32]vars.Bit, offset int) [32]vars.Bit {
	var result [32]vars.Bit
	for i := 0; i < 32; i++ {
		result[(i+offset)%len(i1)] = i1[i]
	}
	return result
}

func (a *API) Shr32(i1 [32]vars.Bit, offset int) [32]vars.Bit {
	var result [32]vars.Bit
	for i := 0; i < 32; i++ {
		if i < offset {
			result[i].Value = 0
		} else {
			result[i] = i1[i-offset]
		}
	}
	return result
}

func (a *API) AddMany32(in ...[32]vars.Bit) [32]vars.Bit {
	if len(in) == 1 {
		return in[0]
	} else {
		return a.Add32(in[0], a.AddMany32(in[1:]...))
	}
}

func (a *API) Add32(i1, i2 [32]vars.Bit) [32]vars.Bit {
	var result [32]vars.Bit
	var carry frontend.Variable = 0
	for i := 31; i >= 0; i-- {
		sum := a.api.Add(i1[i].Value, i2[i].Value, carry)
		sumBin := a.api.ToBinary(sum, 2)
		result[i] = vars.Bit{Value: sumBin[0]}
		carry = sumBin[1]
	}
	return result
}

func (a *API) Not32(i1 [32]vars.Bit) [32]vars.Bit {
	var result [32]vars.Bit
	for i := 0; i < 32; i++ {
		result[i] = vars.Bit{Value: a.api.Sub(1, i1[i].Value)}
	}
	return result
}

func (a *API) ToBitsFromByte(i1 vars.Byte) [8]vars.Bit {
	values := a.api.ToBinary(i1.Value, 8)
	var bits [8]vars.Bit
	for i := 0; i < 8; i++ {
		bits[i] = vars.Bit{Value: values[i]}
	}
	return bits
}

func (a *API) ToByteFromBits(i1 [8]vars.Bit) vars.Byte {
	value := frontend.Variable(0)
	power := frontend.Variable(1)
	for i := 0; i < 8; i++ {
		value = a.api.Add(value, a.api.Mul(power, i1[i].Value))
		power = a.api.Mul(power, 2)
	}
	return vars.Byte{Value: value}
}

func (a *API) FromUint32(value uint32) [32]vars.Bit {
	var result [32]vars.Bit
	for k := 0; k < 32; k++ {
		if (value & (1 << (31 - k))) != 0 {
			result[k] = vars.ONE_BIT
		} else {
			result[k] = vars.ZERO_BIT
		}
	}
	return result
}

func (a *API) PrintBits(in []vars.Bit) {
	variables := make([]frontend.Variable, len(in))
	for i := 0; i < len(in); i++ {
		variables[i] = in[i].Value
	}
	a.api.Println(variables...)
}

func (a *API) SelectByte(selector vars.Bit, i1 vars.Byte, i2 vars.Byte) vars.Byte {
	return vars.Byte{Value: a.api.Select(selector.Value, i1.Value, i2.Value)}
}

func (a *API) SelectBytes32(selector vars.Bit, i1 [32]vars.Byte, i2 [32]vars.Byte) [32]vars.Byte {
	var result [32]vars.Byte
	for i := 0; i < 32; i++ {
		result[i] = a.SelectByte(selector, i1[i], i2[i])
	}
	return result
}
