package succinct

import (
	"bytes"
	"encoding/hex"
	"math/big"
	"testing"

	"github.com/consensys/gnark/frontend"
	"github.com/stretchr/testify/assert"
	"github.com/succinctlabs/succinctx/gnarkx/builder"
	"github.com/succinctlabs/succinctx/gnarkx/utils/byteutils"
	"github.com/succinctlabs/succinctx/gnarkx/vars"
)

type TestCircuit struct {
	InputBytes  []vars.Byte
	OutputBytes []vars.Byte
}

var _ Circuit = (*TestCircuit)(nil)

func NewTestCircuit() *TestCircuit {
	return &TestCircuit{
		InputBytes:  vars.NewBytes(16),
		OutputBytes: vars.NewBytes(8),
	}
}

func (c *TestCircuit) Assign(in []byte) error {
	return nil
}

func (c *TestCircuit) GetInputBytes() *[]vars.Byte {
	return &c.InputBytes
}

func (c *TestCircuit) GetOutputBytes() *[]vars.Byte {
	return &c.OutputBytes
}

func (c *TestCircuit) Define(baseAPI frontend.API) error {
	api := builder.NewAPI(baseAPI)
	inputReader := builder.NewInputReader(*api, c.InputBytes)

	a := inputReader.ReadUint64()
	b := inputReader.ReadUint64()

	outputWriter := builder.NewOutputWriter(*api)

	sum := api.AddU64(a, b)

	outputWriter.WriteU64(sum)

	outputWriter.Close(c.OutputBytes)

	return nil
}

func (c *TestCircuit) SetWitness(inputBytes []byte) {
	vars.SetBytes(&c.InputBytes, inputBytes)

	a := new(big.Int).SetBytes(inputBytes[:8])
	b := new(big.Int).SetBytes(inputBytes[8:])

	sum := new(big.Int).Add(a, b)
	outputBytes := make([]byte, 8)
	sum.FillBytes(outputBytes)

	vars.SetBytes(&c.OutputBytes, outputBytes[:])
}

func TestSimpleCircuit(t *testing.T) {
	c := NewCircuitFunction(NewTestCircuit())

	build, err := c.Build()
	assert.NoError(t, err)

	// 420, 69
	input, err := hex.DecodeString("00000000000001a40000000000000045")
	assert.NoError(t, err)

	proof, err := c.Prove(input, build)
	assert.NoError(t, err)

	// sha256(input)
	expectedInputHash, err := hex.DecodeString("ae964f1e8905240278a7429d6573ba715baf3f4134693c94533ba8a7e57b636e")
	if err != nil {
		t.Fatal(err)
	}
	// uint64(489)
	expectedOutput, err := hex.DecodeString("00000000000001e9")
	if err != nil {
		t.Fatal(err)
	}
	// sha256(uint64(489))
	expectedOutputHash, err := hex.DecodeString("080f024e0afaa2f4ed40a8bb08976d6c1bf771342a7ddec9de94a97c0598846a")
	if err != nil {
		t.Fatal(err)
	}

	// Truncate hashes to rightmost 253 bits as we would in Solidity
	truncatedInputHash := byteutils.TruncateBytes32([32]byte(expectedInputHash), 253)
	truncatedOutputHash := byteutils.TruncateBytes32([32]byte(expectedOutputHash), 253)

	resultInput := proof.Input
	inputHash := c.InputHash.Value.(*big.Int).Bytes()
	output := proof.Output
	outputHash := c.OutputHash.Value.(*big.Int).Bytes()

	assert.True(t, bytes.Equal(input, resultInput))
	assert.True(t, bytes.Equal(inputHash, truncatedInputHash[:]))
	assert.True(t, bytes.Equal(output, expectedOutput))
	assert.True(t, bytes.Equal(outputHash, truncatedOutputHash[:]))
}
