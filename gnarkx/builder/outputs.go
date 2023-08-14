package builder

import (
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

// OutputWriter is used for writing outputs from a circuit that need to be read on-chain. In
// particular, the struct is used for writing to a a list of output bytes which is then hashed
// to produce a commitment to the outputs of the circuit.
type OutputWriter struct {
	api   API
	ptr   int
	bytes []vars.Byte
}

// Creates a new OutputWriter.
func NewOutputWriter(api API) *OutputWriter {
	return &OutputWriter{
		api:   api,
		ptr:   0,
		bytes: make([]vars.Byte, 0),
	}
}

// Writes a single u64 to the output stream.
func (w *OutputWriter) WriteU64(i1 vars.U64) {
	bytes := w.api.ToBytes32FromU64LE(i1)
	for i := 0; i < 8; i++ {
		w.bytes = append(w.bytes, bytes[8-i-1])
	}
}

func (w *OutputWriter) WriteBytes32(bytes [32]vars.Byte) {
	for i := 0; i < 32; i++ {
		w.bytes = append(w.bytes, bytes[i])
	}
}

func (w *OutputWriter) Close(expectedBytes []vars.Byte) {
	if len(w.bytes) != len(expectedBytes) {
		panic("unexpected number of output bytes")
	}
	for i := 0; i < len(w.bytes); i++ {
		w.api.AssertIsEqualByte(w.bytes[i], expectedBytes[i])
	}
}
