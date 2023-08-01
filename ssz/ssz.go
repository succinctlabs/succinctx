package ssz

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-gadgets/hash/sha256"
	"github.com/succinctlabs/gnark-gadgets/vars"
)

func VerifyProof(api frontend.API, leaf vars.Bytes32, proof []vars.Bytes32, root vars.Bytes32, gindex int, depth int) {
	restoredRoot := RestoreMerkleRoot(api, leaf, proof, gindex, depth)
	for i := 0; i < 256; i++ {
		api.AssertIsEqual(root[i], restoredRoot[i])
	}
}

func HashTreeRoot(api frontend.API, leaves []vars.Bytes32, NumLeaves int) vars.Bytes32 {
	// check NumLeaves is a power of 2
	if NumLeaves&(NumLeaves-1) != 0 {
		panic("NumLeaves must be a power of 2")
	}
	for NumLeaves > 1 {
		for i := 0; i < NumLeaves/2; i++ {
			leaves[i] = sha256.HashBytes(api, append(leaves[i*2][:], leaves[i*2+1][:]...))
		}
		NumLeaves = NumLeaves / 2
	}
	return leaves[0]
}

func RestoreMerkleRoot(api frontend.API, leaf vars.Bytes32, proof []vars.Bytes32, gindex int, depth int) vars.Bytes32 {
	if len(proof) != depth {
		panic("SSZ proof length must equal provided depth")
	}
	hash := leaf
	for i := 0; i < depth; i++ {
		if gindex%2 == 1 {
			hash = sha256.HashBytes(api, append(proof[i][:], hash[:]...))
		} else {
			hash = sha256.HashBytes(api, append(hash[:], proof[i][:]...))
		}
		gindex = gindex / 2
	}
	return hash
}
