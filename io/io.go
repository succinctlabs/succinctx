package io

import (
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/hash/sha2"
	"github.com/consensys/gnark/std/math/uints"
)

// type ComponentIO struct {
// 	InputHash  frontend.Variable `gnark:",public"`
// 	OutputHash frontend.Variable `gnark:",public"`
// }

// func (io *SuccinctIO) Constrain(api frontend.API, ) error {

// 	var inputBytes []frontend.Variable

// 	// 1) check input hash matches
// 	sha(inputBytes) =

// 	// 2) decode input bytes
// 	ptr := 0
// 	firstUint256 := readUint256(inputBytes, ptr)

// }

type Reader struct {
	bytes []frontend.Variable
	api   frontend.API
	ptr   int
}

func NewReader(api frontend.API, bytes []frontend.Variable) *Reader {
	return &Reader{
		bytes: bytes,
		api:   api,
		ptr:   0,
	}
}

func (d *Reader) nextByte() frontend.Variable {
	out := d.bytes[d.ptr]
	d.ptr++
	return out
}

func (d *Reader) ReadBytes(numBytes int) []frontend.Variable {
	bytes := make([]frontend.Variable, numBytes)
	for i := 0; i < numBytes; i++ {
		bytes[i] = d.nextByte()
	}
	return bytes
}

func (d *Reader) ReadUint(numBytes int) frontend.Variable {
	if numBytes == 32 {
		panic("Use ReadUint256")
	}
	bytes := d.ReadBytes(numBytes)

	for i := 0; i < numBytes; i++ {
		if i < numBytes-1 {
			bytes[i] = d.api.Mul(bytes[i], 256^(numBytes-i-1))
		}
	}

	if numBytes == 1 {
		return bytes[0]
	}
	if numBytes == 2 {
		return d.api.Add(bytes[0], bytes[1])
	}
	return d.api.Add(bytes[0], bytes[1], bytes[2:]...)
}

func (d *Reader) ReadUint256() [2]frontend.Variable {
	upper := d.ReadUint(16)
	lower := d.ReadUint(16)
	return [2]frontend.Variable{upper, lower}
}

// func

// frontend.Circuit
// Populate(input []byte) ([]byte, error)

type Writer struct {
	bytes []frontend.Variable
	api   frontend.API
}

func NewWriter(api frontend.API) *Writer {
	return &Writer{
		bytes: []frontend.Variable{},
		api:   api,
	}
}

func (w *Writer) WriteUint(num frontend.Variable) {
	bits := w.api.ToBinary(num)
	for _, bit := range bits {
		w.bytes = append(w.bytes, bit)
	}
}

func (w *Writer) WriteUintBits(numBits int, num frontend.Variable) {
	// bits := w.api.ToBinary(num, numBits)
	// for i := 0; i < numBits; i++ {
	// 	w.bytes = append(w.bytes, bits[len(bits)-i-1])
	// }
	// for i := 0; i < numBits/8; i++ {
	// 	w.api.
}

func (w *Writer) WriteUint256(num [2]frontend.Variable) {
	w.WriteUintBits(128, num[0])
	w.WriteUintBits(128, num[1])
}

func (w *Writer) Bytes() []frontend.Variable {
	return w.bytes
}

func (w *Writer) WriteBytes(bytes []frontend.Variable) {
	w.bytes = append(w.bytes, bytes...)
}

func (w *Writer) WriteByte(byte frontend.Variable) {
	w.bytes = append(w.bytes, byte)
}

func (w *Writer) Hash() []uints.U8 {
	hasher, error := sha2.New(w.api)
	if error != nil {
		panic(error)
	}
	outputUints := make([]uints.U8, len(w.bytes))
	for i, outputByte := range w.bytes {
		outputUints[i] = uints.U8{
			Val: outputByte,
		}
	}
	hasher.Write(outputUints)
	return hasher.Sum()
}
