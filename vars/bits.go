package vars

import "github.com/consensys/gnark/frontend"

type Bit struct {
	Value frontend.Variable
}

func NewBit(x int) Bit {
	if (x != 0) && (x != 1) {
		panic("NewBit: must be given 0 or 1")
	}
	return Bit{Value: x}
}
