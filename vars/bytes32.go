package vars

import (
	"encoding/binary"

	"github.com/consensys/gnark/frontend"
)

type Bytes32 = [32]frontend.Variable
type Bytes = []frontend.Variable

func NewBytes(bytes []byte) Bytes {
	vars := make([]frontend.Variable, len(bytes))
	for i, b := range bytes {
		vars[i] = frontend.Variable(b)
	}
	return vars
}

func NewBytes32(bytes []byte) Bytes32 {
	if len(bytes) != 32 {
		panic("NewBytes32 must be given 32 bytes")
	}
	var vars [32]frontend.Variable
	for i, b := range bytes {
		vars[i] = frontend.Variable(b)
	}
	return vars
}

func NewBytes32FromUint64LE(value uint64) Bytes32 {
	var bytes [32]byte
	binary.LittleEndian.PutUint64(bytes[:], value)
	return NewBytes32(bytes[:])
}

func NewBytes32PadLeft(bytes []byte, n int) Bytes32 {
	if len(bytes) != n {
		panic("NewBytes32FromLE must be given n bytes")
	}
	if n >= 32 {
		panic("NewBytes32FromLE must be given n <= 32 bytes")
	}
	var vars [32]frontend.Variable
	for i := 0; i < 32-n; i++ {
		vars[i] = frontend.Variable(0)
	}
	for i, b := range bytes {
		vars[i+32-n] = frontend.Variable(b)
	}
	return vars
}

func NewBytes32Array(bytes [][]byte) []Bytes32 {
	var vars []Bytes32
	for _, b := range bytes {
		vars = append(vars, NewBytes32(b))
	}
	return vars
}
