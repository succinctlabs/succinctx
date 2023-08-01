package bitutils

import (
	"encoding/hex"

	"github.com/succinctlabs/gnark-gadgets/vars"
)

func ToBits(arr []byte) []vars.Bit {
	result := make([]vars.Bit, len(arr)*8)
	for i, v := range arr {
		for j := 0; j < 8; j++ {
			if (v & (1 << (7 - j))) != 0 {
				result[i*8+j] = vars.ONE_BIT
			} else {
				result[i*8+j] = vars.ZERO_BIT
			}
		}
	}
	return result
}

func Decode(s string) []byte {
	result, err := hex.DecodeString(s)
	if err != nil {
		panic(err)
	}
	return result
}
