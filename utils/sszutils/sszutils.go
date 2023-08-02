// Methods for operations relating to SSZ, a serialization method used in the Ethereum consensus
// layer, for out of circuit data. To learn more, refer to
// https://ethereum.org/en/developers/docs/data-structures-and-encoding/ssz/.
package sszutils

import "crypto/sha256"

// Hashes a byte slice using sha256.
func Hash(data []byte) [32]byte {
	h := sha256.New()
	h.Write(data)
	res := h.Sum(nil)
	if len(res) != 32 {
		panic("hash length must be 32 bytes")
	}
	var hash [32]byte
	copy(hash[:], res)
	return hash
}

// Verifies a SSZ proof.
func VerifyProof(root [32]byte, leaf [32]byte, proof [][32]byte, gindex int) {
	restoredRoot := RestoreMerkleRoot(leaf, proof, gindex)
	for i := 0; i < 32; i++ {
		if root[i] != restoredRoot[i] {
			panic("root does not match")
		}
	}
}

// Computes the expected root of a SSZ proof given a leaf, proof, and gindex.
func RestoreMerkleRoot(leaf [32]byte, proof [][32]byte, gindex int) [32]byte {
	hash := leaf
	for i := 0; i < len(proof); i++ {
		if gindex%2 == 1 {
			hash = Hash(append(proof[i][:], hash[:]...))
		} else {
			hash = Hash(append(hash[:], proof[i][:]...))
		}
		gindex = gindex / 2
	}
	return hash
}

// Computes the root of a SSZ tree given a list of leaves.
func HashTreeRoot(leaves [][32]byte) [32]byte {
	nbLeaves := len(leaves)
	if nbLeaves&(nbLeaves-1) != 0 {
		panic("nbLeaves must be a power of 2")
	}
	for nbLeaves > 1 {
		for i := 0; i < nbLeaves/2; i++ {
			leaves[i] = Hash(append(leaves[i*2][:], leaves[i*2+1][:]...))
		}
		nbLeaves = nbLeaves / 2
	}
	return leaves[0]
}

// Converts a u64 to a 32 byte array in little endian.
func NewBytes32FromU64LE(v uint64) [32]byte {
	var res [32]byte
	for i := 0; i < 8; i++ {
		res[i] = byte(v >> (8 * i))
	}
	return res
}

// Converts a byte slice to 32 byte array with left padding.
func NewBytes32FromBytesLeftPad(data []byte) [32]byte {
	var res [32]byte
	if len(data) > 32 {
		panic("data too long")
	}
	copy(res[32-len(data):], data)
	return res
}

// Converts a byte slice to 32 byte array with right padding.
func NewBytes32FromBytesRightPad(data []byte) [32]byte {
	var res [32]byte
	if len(data) > 32 {
		panic("data too long")
	}
	copy(res[:], data)
	return res
}
