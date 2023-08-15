package main

import (
	"encoding/hex"
	"strconv"
)

func decodeHex(s string) ([]byte, error) {
	return hex.DecodeString(s[2:])
}

func decodeHexSlice(strings []string) [][]byte {
	bytes := make([][]byte, len(strings))
	for i, s := range strings {
		bytes[i], _ = decodeHex(s)
	}
	return bytes
}

func stringToUint64Slice(strings []string) []uint64 {
	numbers := make([]uint64, len(strings))
	for i, s := range strings {
		numbers[i], _ = strconv.ParseUint(s, 10, 64)
	}
	return numbers
}
