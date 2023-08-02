package vars

import (
	"encoding/binary"
)

// The zero byte as a variable in a circuit. If used within APIs, it will be treated as a constant.
var ZERO_BYTE = Byte{Value: ZERO}

// A variable in a circuit representing a byte. Under the hood, the value is a single field element.
type Byte struct {
	Value Variable
}

// Creates a new byte as a variable in a circuit.
func NewByte(i1 byte) Byte {
	return Byte{Value: NewVariableFromInt(int(i1))}
}

// Creates a new array of bytes as a variable in a circuit.
func NewBytes(i1 []byte) []Byte {
	var result []Byte
	for i := 0; i < len(i1); i++ {
		result = append(result, NewByte(i1[i]))
	}
	return result
}

// Creates a new array of bytes32 as a variable in a circuit.
func NewBytesArray(i1 [][]byte) [][]Byte {
	var result [][]Byte
	for i := 0; i < len(i1); i++ {
		result = append(result, NewBytes(i1[i]))
	}
	return result
}

// Creates a new bytes32 as a variable in a circuit.
func NewBytes32(i1 [32]byte) [32]Byte {
	var result [32]Byte
	for i := 0; i < 32; i++ {
		result[i] = NewByte(i1[i])
	}
	return result
}

// Creates a new array of bytes32 as a variable in a circuit.
func NewBytes32Array(i1 [][32]byte) [][32]Byte {
	var result [][32]Byte
	for i := 0; i < len(i1); i++ {
		result = append(result, NewBytes32(i1[i]))
	}
	return result
}

// Creates a new bytes32 as a variable in a circuit from a u64. The u64 will placed in the first
// 8 bytes of the bytes32 (aka "little endian").
func NewBytes32FromU64LE(i1 uint64) [32]Byte {
	var b [32]byte
	binary.LittleEndian.PutUint64(b[:], i1)
	return NewBytes32(b)
}

// Creates a new bytes32 as a variable in a circuit from a u64. The u64 will placed in the
func NewBytes32FromBytesLeftPad(i1 []byte) [32]Byte {
	if len(i1) < 20 {
		panic("length of i1 is less than 20")
	}

	var b [32]byte
	startOffset := 32 - len(i1) - 1
	for i := 0; i < len(i1); i++ {
		b[startOffset+i] = i1[i]
	}
	return NewBytes32(b)
}

func NewBytes32FromBytesRightPad(data []byte) [32]Byte {
	if len(data) > 32 {
		panic("length of data is greater than 32")
	}
	return NewBytes32([32]byte(data))
}
