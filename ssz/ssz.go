package ssz

import (
	"github.com/succinctlabs/gnark-gadgets/hash/sha256"
	"github.com/succinctlabs/gnark-gadgets/succinct"
	"github.com/succinctlabs/gnark-gadgets/vars"
)

type SimpleSerializeAPI struct {
	api succinct.API
}

func NewAPI(api *succinct.API) *SimpleSerializeAPI {
	return &SimpleSerializeAPI{api: *api}
}

func (a *SimpleSerializeAPI) VerifyProof(
	root [32]vars.Byte,
	leaf [32]vars.Byte,
	proof [][32]vars.Byte,
	gindex int,
	depth int,
) {
	restoredRoot := a.RestoreMerkleRoot(leaf, proof, gindex, depth)
	for i := 0; i < 32; i++ {
		a.api.API().AssertIsEqual(root[i].Value, restoredRoot[i].Value)
	}
}

func (a *SimpleSerializeAPI) RestoreMerkleRoot(
	leaf [32]vars.Byte,
	proof [][32]vars.Byte,
	gindex int,
	depth int,
) [32]vars.Byte {
	if len(proof) != depth {
		panic("ssz proof length must equal provided depth")
	}
	hash := leaf
	for i := 0; i < depth; i++ {
		if gindex%2 == 1 {
			hash = sha256.Hash(a.api, append(proof[i][:], hash[:]...))
		} else {
			hash = sha256.Hash(a.api, append(hash[:], proof[i][:]...))
		}
		gindex = gindex / 2
	}
	return hash
}

func (a *SimpleSerializeAPI) HashTreeRoot(
	leaves [][32]vars.Byte,
	nbLeaves int,
) [32]vars.Byte {
	if nbLeaves&(nbLeaves-1) != 0 {
		panic("nbLeaves must be a power of 2")
	}
	for nbLeaves > 1 {
		for i := 0; i < nbLeaves/2; i++ {
			leaves[i] = sha256.Hash(a.api, append(leaves[i*2][:], leaves[i*2+1][:]...))
		}
		nbLeaves = nbLeaves / 2
	}
	return leaves[0]
}
