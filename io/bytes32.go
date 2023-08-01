package io

import (
	"github.com/consensys/gnark/frontend"
)

type Bytes32Var = [256]frontend.Variable

func Bytes32VarFromBytes(bytes []byte) Bytes32Var {
	if len(bytes) != 32 {
		panic("H256FromBytes: must be given 32 bytes")
	}
	var vars [256]frontend.Variable
	for i, b := range bytes {
		for j := 0; j < 8; j++ {
			vars[i*8+j] = (b >> uint(j)) & 1
		}
	}
	return vars
}

func NewBytes32Var() Bytes32Var {
	var bytes32 [256]frontend.Variable
	return bytes32
}

func NewBytes32VarArray(n int) []Bytes32Var {
	var arr []Bytes32Var
	for i := 0; i < n; i++ {
		arr = append(arr, NewBytes32Var())
	}
	return arr
}
