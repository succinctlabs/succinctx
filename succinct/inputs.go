package succinct

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

type InputReader struct {
	api   API
	ptr   int
	bytes []vars.Byte
}

func NewInputReader(api API, bytes []vars.Byte) *InputReader {
	return &InputReader{
		api:   api,
		ptr:   0,
		bytes: bytes,
	}
}

func (r *InputReader) readByte() vars.Byte {
	out := r.bytes[r.ptr]
	r.ptr++
	return out
}

func (r *InputReader) ReadBytes32() [32]vars.Byte {
	var out [32]vars.Byte
	for i := 0; i < 32; i++ {
		out[i] = r.readByte()
	}
	return out
}
