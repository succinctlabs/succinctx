package bitutils

import (
	"encoding/hex"
)

func Decode(s string) []byte {
	result, err := hex.DecodeString(s)
	if err != nil {
		panic(err)
	}
	return result
}
