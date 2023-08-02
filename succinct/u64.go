package succinct

import (
	"math"

	"github.com/succinctlabs/gnark-gadgets/vars"
)

// Computes a_1 + ... + a_n where a_i \in [0, 2^64). This function automatically carries the result
// such that the result is in [0, 2^64). The cost of the carry can be amortized over multiple calls
// to this function by accumulating more terms into a single add.
func (a *API) AddU64(in ...vars.U64) vars.U64 {
	// Find the maximum number of bits needed to represent the sum of all inputs.
	nbTerms := len(in)
	nbMaxBits := 64 + math.Log2(float64(nbTerms)) + 2

	// Compute the sum of all inputs over the field.
	acc := vars.ZERO
	for i := 0; i < len(in); i++ {
		acc = a.Add(acc, in[i].Value)
	}

	// Convert the sum to binary with the calculated number of maximum bits.
	bits := a.ToBinaryLE(acc, int(nbMaxBits))

	// Compute acc % 2^64.
	reduced := vars.ZERO
	power := vars.ONE
	for i := 0; i < 64; i++ {
		reduced = a.Add(reduced, a.Mul(bits[i].Value, power))
		power = a.Mul(power, vars.TWO)
	}
	return vars.U64{Value: reduced}
}

// Converts a U64 to a Bytes32 in little-endian format. In particular, the u64 is decomposed into
// bytes b1, ..., b8 such that 256^0 * b1 + ... + 256^7 * b8 is the native value. The bytes32
// returned is in the form [b1, ..., b8, 0, ..., 0].
func (a *API) ToBytes32FromU64LE(i1 vars.U64) [32]vars.Byte {
	bits := a.ToBinaryLE(i1.Value, 64)
	var bytes [32]vars.Byte
	for i := 0; i < 8; i++ {
		var byteBits [8]vars.Bool
		for j := 0; j < 8; j++ {
			byteBits[j] = bits[i*8+j]
		}
		bytes[i] = a.ToByteFromBits(byteBits)
	}
	return bytes
}
