package byteutils

import (
	"github.com/succinctlabs/gnark-gadgets/vars"
)

func ToBytes(arr []byte) []vars.Byte {
	result := make([]vars.Byte, len(arr))
	for i, v := range arr {
		result[i] = vars.NewByte(v)
	}
	return result
}

func ToBytes32FromBytes(data []byte) [32]byte {
	var fixedSizeArray [32]byte

	// Check if the length of the data is less than or equal to 32
	if len(data) <= len(fixedSizeArray) {
		copy(fixedSizeArray[:], data)
	} else {
		// If the data is larger than 32, copy the first 32 bytes.
		copy(fixedSizeArray[:], data[:32])
	}

	return fixedSizeArray
}
