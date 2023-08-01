package byteutils

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

func ToBytes(arr []byte) []vars.Byte {
	result := make([]vars.Byte, len(arr))
	for i, v := range arr {
		result[i] = vars.NewByte(int(v))
	}
	return result
}
