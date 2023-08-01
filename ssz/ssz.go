package ssz

import (
	"errors"

	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/gnark-gadgets/hash/sha256"
	"github.com/succinctlabs/gnark-gadgets/io"
)

type SSZProofCircuit struct {
	Leaf   io.Bytes32Var
	Proof  []io.Bytes32Var
	Root   io.Bytes32Var
	GIndex int
	Depth  int
}

func (sszProof *SSZProofCircuit) Define(api frontend.API) error {
	depth := sszProof.Depth
	if len(sszProof.Proof) != depth {
		return errors.New("SSZ proof length must equal provided depth")
	}

	gindex := sszProof.GIndex

	hash := sszProof.Leaf
	for i := 0; i < depth; i++ {
		if gindex%2 == 1 {
			hash = sha256.Hash(api, append(sszProof.Proof[i][:], hash[:]...))
		} else {
			hash = sha256.Hash(api, append(hash[:], sszProof.Proof[i][:]...))
		}
		gindex = gindex / 2
	}

	for i := 0; i < 256; i++ {
		api.AssertIsEqual(sszProof.Root[i], hash[i])
	}

	return nil
}
