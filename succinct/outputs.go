package succinct

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

type OutputWriter struct {
	api   API
	ptr   int
	bytes []vars.Byte
}

func NewOutputWriter(api API) *OutputWriter {
	return &OutputWriter{
		api:   api,
		ptr:   0,
		bytes: make([]vars.Byte, 0),
	}
}
