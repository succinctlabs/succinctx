package bitutils

import (
	"encoding/hex"

	"github.com/consensys/gnark/frontend"
)

func ToBits(arr []byte) []frontend.Variable {
	result := make([]frontend.Variable, len(arr)*8)
	for i, v := range arr {
		for j := 0; j < 8; j++ {
			if (v & (1 << (7 - j))) != 0 {
				result[i*8+j] = 1
			} else {
				result[i*8+j] = 0
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
