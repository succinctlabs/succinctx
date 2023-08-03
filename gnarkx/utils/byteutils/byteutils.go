package byteutils

func ReverseBytes(data []byte) []byte {
	length := len(data)
	reversed := make([]byte, length)
	for i := range data {
		reversed[i] = data[length-1-i]
	}
	return reversed
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

func ToBytes32FromU64LE(v uint64) [32]byte {
	var res [32]byte
	for i := 0; i < 8; i++ {
		res[i] = byte(v >> (8 * i))
	}
	return res
}

func ToBytes32FromBytesLeftPad(data []byte) [32]byte {
	var res [32]byte
	if len(data) > 32 {
		panic("data too long")
	}
	copy(res[32-len(data):], data)
	return res
}

func ToBytes32FromBytesRightPad(data []byte) [32]byte {
	var res [32]byte
	if len(data) > 32 {
		panic("data too long")
	}
	copy(res[:], data)
	return res
}
