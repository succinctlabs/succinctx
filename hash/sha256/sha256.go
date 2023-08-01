package sha256

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-gadgets/hash/bits"
)

// First 32 bits of the fractional parts of the square roots of the first 8 primes.
// Reference: https://en.wikipedia.org/wiki/SHA-2
var H = []uint32{
	0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
}

// First 32 bits of the fractional parts of the cube roots of the first 64 primes.
// Reference: https://en.wikipedia.org/wiki/SHA-2
var K = []uint32{
	0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
	0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
	0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
	0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
	0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
	0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
	0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
	0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
}

func Hash(api frontend.API, in []frontend.Variable) [256]frontend.Variable {
	// The length-encoded message length ("L + 1 + 64").
	const seperatorLength = 1
	const u64BitLength = 64
	encodedMessageLength := len(in) + seperatorLength + u64BitLength

	// The multiple of 512-bit padded message length. Padding length is "K".
	remainderLength := encodedMessageLength % 512
	paddingLength := 0
	if remainderLength == 0 {
		paddingLength = 0
	} else {
		paddingLength = (512 - remainderLength)
	}

	// Initialization of core variables.
	var paddedMessage [256]frontend.Variable
	for i := 0; i < len(in); i++ {
		paddedMessage[i] = frontend.Variable(0)
	}

	// Begin with the original message of length "L".
	for i := 0; i < len(in); i++ {
		paddedMessage[i] = in[i]
	}

	// Append a single '1' bit.
	paddedMessage[len(in)] = frontend.Variable(1)

	// Append L as a 64-bit big-endian integer.
	inputLengthBitsLE := api.ToBinary(frontend.Variable(len(in)), 64)
	inputLengthBitsBE := make([]frontend.Variable, 64)
	for i := 0; i < 64; i++ {
		inputLengthBitsBE[i] = inputLengthBitsLE[63-i]
	}
	for i := 0; i < len(inputLengthBitsBE); i++ {
		paddedMessage[len(in)+i+1+paddingLength] = inputLengthBitsBE[i]
	}

	// At this point, the padded message should be of the following form.
	//      <message of length L> 1 <K zeros> <L as 64 bit integer>
	// Now, we will process the padded message in 512 bit chunks and begin referring to the
	// padded message as "message".
	const sha256ChunkLength = 512
	const sha256WordLength = 32
	const sha256MessageScheduleArrayLength = 64

	message := paddedMessage
	numChunks := len(message) / sha256ChunkLength

	for i := 0; i < numChunks; i++ {
		// The 64-entry message schedule array of 32-bit words.
		var w [sha256MessageScheduleArrayLength][sha256WordLength]frontend.Variable

		// Copy chunk into first 16 words w[0..15] of the message schedule array.
		chunkOffset := i * sha256ChunkLength
		for j := 0; j < 16; j++ {
			wordOffset := j * 32
			for k := 0; k < 32; k++ {
				w[j][k] = message[chunkOffset+wordOffset+k]
			}
		}

		// Extend the first 16 words into the remaining 48 words w[16..63].
		for j := 16; j < sha256MessageScheduleArrayLength; j++ {
			s0 := bits.Xor3(
				api,
				bits.Rotate(w[j-15], 7),
				bits.Rotate(w[j-15], 18),
				bits.Shr(w[j-15], 3),
			)
			s1 := bits.Xor3(
				api,
				bits.Rotate(w[j-2], 17),
				bits.Rotate(w[j-2], 19),
				bits.Shr(w[j-2], 10),
			)
			w[j] = bits.Add(api, w[j-16], s0, w[j-7], s1)
		}
	}

	var digest [256]frontend.Variable
	return digest
}
