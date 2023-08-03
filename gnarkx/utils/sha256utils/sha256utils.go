package sha256utils

import (
	"crypto/sha256"
	"math/big"
)

func reverseBytes(data []byte) []byte {
	length := len(data)
	reversed := make([]byte, length)
	for i := range data {
		reversed[i] = data[length-1-i]
	}
	return reversed
}

// Computes sha256(data) & ((1 << nbBits) - 1)).
func HashAndTruncate(data []byte, nbBits int) *big.Int {
	// Compute sha256(data).
	hasher := sha256.New()
	hasher.Write(data)
	h := hasher.Sum(nil)
	var bytes [32]byte
	copy(bytes[:], h)

	// Convert the hash to a big.Int and truncate it to the lower nbBits.
	value := new(big.Int).SetBytes(reverseBytes(bytes[:]))
	mask := new(big.Int).Lsh(big.NewInt(1), uint(nbBits))
	mask.Sub(mask, big.NewInt(1))
	result := new(big.Int).And(value, mask)
	return result
}
