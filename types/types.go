package types

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common/hexutil"
)

type Groth16Proof struct {
	A      [2]*big.Int    `json:"a"`
	B      [2][2]*big.Int `json:"b"`
	C      [2]*big.Int    `json:"c"`
	Inputs []*big.Int     `json:"inputs"`
	Output hexutil.Bytes  `json:"output"`
}
