package vars

import "github.com/consensys/gnark/frontend"

type Byte struct {
	Value frontend.Variable
}

func NewByte(x int) Byte {
	return Byte{Value: x}
}

func NewBytes32(b []byte) [32]Byte {
	var result [32]Byte
	for i := 0; i < 32; i++ {
		result[i] = Byte{Value: int(b[i])}
	}
	return result
}

func NewBytes32Array(b [][]byte) [][32]Byte {
	var result [][32]Byte
	for i := 0; i < len(b); i++ {
		result = append(result, NewBytes32(b[i]))
	}
	return result
}

func (b Byte) ToBits(api frontend.API) [8]Bit {
	values := api.ToBinary(b.Value, 8)
	var bits [8]Bit
	for i := 0; i < 8; i++ {
		bits[i] = Bit{Value: values[i]}
	}
	return bits
}
