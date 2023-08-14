package vars

import (
	"encoding/binary"
	"math/big"
)

// The zero byte as a variable in a circuit. If used within APIs, it will be treated as a constant.
var ZERO_BYTE = Byte{Value: ZERO}

// A variable in a circuit representing a byte. Under the hood, the value is a single field element.
type Byte struct {
	Value Variable
}

// Initializes an empty Byte
func NewByte() Byte {
	return Byte{Value: ZERO}
}

// Sets a byte from a byte value.
func (b *Byte) Set(i1 byte) {
	b.Value = NewVariableFromInt(int(i1))
}

func (b *Byte) GetValueUnsafe() byte {
	rawValue := b.Value.Value
	if intValue, ok := rawValue.(int); ok {
		// If so, check if int is > 255 and panic if it is
		if intValue > 255 {
			panic("Value is greater than 255")
		}
		// Convert int to type byte
		convertedValue := byte(intValue)
		return convertedValue
	} else if bigIntValue, ok := rawValue.(*big.Int); ok {
		if bigIntValue.Cmp(big.NewInt(255)) > 0 {
			panic("Value is greater than 255")
		}
		convertedValue := byte(bigIntValue.Int64())
		return convertedValue
	} else {
		panic("Value is not of type big.Int")
	}
}

// Creates a new array of bytes as a variable in a circuit.
func NewBytes(n int) []Byte {
	var bytes []Byte
	for i := 0; i < n; i++ {
		bytes = append(bytes, NewByte())
	}
	return bytes
}

func SetBytes(b *[]Byte, i1 []byte) {
	for i := 0; i < len(i1); i++ {
		(*b)[i].Set(i1[i])
	}
}

func NewBytesFrom(bytes []byte) []Byte {
	result := NewBytes(len(bytes))
	SetBytes(&result, bytes)
	return result
}

func GetValuesUnsafe(b []Byte) []byte {
	var bytes []byte
	for _, v := range b {
		bytes = append(bytes, v.GetValueUnsafe())
	}
	return bytes
}

func NewBytesArray(n int, m int) [][]Byte {
	var bytes [][]Byte
	for i := 0; i < n; i++ {
		bytes = append(bytes, NewBytes(m))
	}
	return bytes
}

func SetBytesArray(b *[][]Byte, i1 [][]byte) {
	for i := 0; i < len(i1); i++ {
		SetBytes(&(*b)[i], i1[i])
	}
}

func NewBytes32() [32]Byte {
	var result [32]Byte
	for i := 0; i < 32; i++ {
		result[i] = ZERO_BYTE
	}
	return result
}

// Creates a new bytes32 as a variable in a circuit.
func SetBytes32(b *[32]Byte, i1 [32]byte) {
	for i := 0; i < 32; i++ {
		b[i].Set(i1[i])
	}
}

// ReverseBytes32 returns a bytes32 with the bytes reversed.
func ReverseBytes32(b [32]Byte) [32]Byte {
	var result [32]Byte
	for i := 0; i < 32; i++ {
		result[i] = b[32-i-1]
	}
	return result
}

// Creates a new bytes32 as a variable in a circuit from a u64. The u64 will placed in the first
// 8 bytes of the bytes32 (aka "little endian").
func SetBytes32FromU64LE(b *[32]Byte, i1 uint64) {
	var cast [32]byte
	binary.LittleEndian.PutUint64(cast[:], i1)
	SetBytes32(b, cast)
}

func SetBytes32WithLeftPad(b *[32]Byte, i1 []byte) {
	if len(i1) > 32 {
		panic("length of i1 is less than 20")
	}

	var padded [32]byte
	startOffset := 32 - len(i1) - 1
	for i := 0; i < len(i1); i++ {
		padded[startOffset+i] = i1[i]
	}
	SetBytes32(b, padded)
}

func SetBytes32WithRightPad(b *[32]Byte, data []byte) {
	if len(data) > 32 {
		panic("length of data is greater than 32")
	}
	var padded [32]byte
	copy(padded[:], data)
	SetBytes32(b, padded)
}

// Creates a new array of bytes32 as a variable in a circuit.
func NewBytes32Array(n int) [][32]Byte {
	var result [][32]Byte
	for i := 0; i < n; i++ {
		result = append(result, NewBytes32())
	}
	return result
}

func SetBytes32Array(b *[][32]Byte, i1 [][32]byte) {
	for i := 0; i < len(i1); i++ {
		SetBytes32(&(*b)[i], i1[i])
	}
}
