package succinct

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"testing"

	"github.com/consensys/gnark/frontend"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/stretchr/testify/assert"
	"github.com/succinctlabs/sdk/gnarkx/builder"
	"github.com/succinctlabs/sdk/gnarkx/utils/byteutils"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

type TestCircuit struct {
	InputBytes  []vars.Byte
	OutputBytes []vars.Byte
}

var _ Circuit = (*TestCircuit)(nil)

func NewTestCircuit() *TestCircuit {
	return &TestCircuit{
		InputBytes:  vars.NewBytes(32),
		OutputBytes: vars.NewBytes(32),
	}
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

	a := inputReader.ReadBytes32()

	// a := inputReader.ReadUint64()
	// b := inputReader.ReadUint64()

	// api.PrintU64(a)
	// api.PrintU64(b)

	outputWriter := builder.NewOutputWriter(*api)

	outputWriter.WriteBytes32(a)
	outputWriter.Close(c.OutputBytes)

	// sum := api.AddU64(a, b)

	// fmt.Println("wtf", sum)

	// baseAPI.Println("sum", sum)

	// outputWriter.WriteU64(sum)

	// outputWriter.Close(c.OutputBytes)

	return nil
}

func (c *TestCircuit) SetWitness(inputBytes []byte) {
	vars.SetBytes(&c.InputBytes, inputBytes)
	bytes32, err := abi.NewType("bytes32", "", nil)
	if err != nil {
		panic(err)
	}
	args := abi.Arguments{
		{
			Type: bytes32,
		},
	}
	inputs, err := args.Unpack(inputBytes)
	if err != nil {
		panic(err)
	}

	a := inputs[0].([32]byte)
	outputBytes := a

	fmt.Println(outputBytes)
	fmt.Println(hex.EncodeToString(outputBytes[:]))

	vars.SetBytes(&c.OutputBytes, outputBytes[:])
	// uint64Type, err := abi.NewType("uint64", "", nil)
	// if err != nil {
	// 	panic(err)
	// }
	// args := abi.Arguments{
	// 	{
	// 		Type: uint64Type,
	// 	},
	// 	{
	// 		Type: uint64Type,
	// 	},
	// }
	// inputs, err := args.Unpack(inputBytes)
	// if err != nil {
	// 	panic(err)
	// }

	// a := inputs[0].(uint64)
	// b := inputs[1].(uint64)

	// output := a + b

	// outputArgs := abi.Arguments{
	// 	{
	// 		Type: uint64Type,
	// 	},
	// }
	// outputBytes, err := outputArgs.Pack(output)

	// fmt.Println(outputBytes)
	// fmt.Println(hex.EncodeToString(outputBytes))

	// vars.SetBytes(&c.OutputBytes, outputBytes)

	// func (c *Circuit) SetWitness(inputBytes []byte) {
	// 	// Set the input bytes.
	// 	vars.SetBytes(&c.InputBytes, inputBytes)

	// 	// Get the block root from the input bytes.
	// 	blockRoot := hexutil.Encode(inputBytes)

	// 	// Read from the JSON fixture to get the merkle proofs.
	// 	wd, err := GetPartialWithdrawalsProofFromAPI(blockRoot)
	// 	if err != nil {
	// 		panic(fmt.Errorf("failed to get withdrawals data: %v", err))
	// 	}

	// 	// Set the withdrawals root and withdrawals root proof.)
	// 	vars.SetBytes32(&c.WithdrawalsRoot, byteutils.ToBytes32FromBytes(wd.WithdrawalsRoot))
	// 	for i, proof := range wd.WithdrawalsRootProof {)
	// 		vars.SetBytes32(
	// 			&c.WithdrawalsRootProof[i],
	// 			byteutils.ToBytes32FromBytes(proof),)
	// 		)
	// 	}

	// 	// Set the partial withdrawals and partial withdrawal proofs.
	// 	totalAmount := uint64(0)
	// 	for i := 0; i < 4; i++ {
	// 		pw := &c.PartialWithdrawals[i]
	// 		vars.SetBytes32FromU64LE(&pw.Index, wd.PartialWithdrawalIndexes[i])
	// 		vars.SetBytes32FromU64LE(&pw.ValidatorIndex, wd.PartialWithdrawalValidatorIndexes[i])
	// 		vars.SetBytes32WithRightPad(&pw.Address, wd.PartialWithdrawalAddresses[i])
	// 		pw.Amount.Set(wd.PartialWithdrawalAmounts[i])
	// 		for j, proof := range wd.PartialWithdrawalProofs[i] {)
	// 			vars.SetBytes32(&c.PartialWithdrawalProofs[i][j], byteutils.ToBytes32FromBytes(proof)))
	// 		}
	// 		totalAmount += wd.PartialWithdrawalAmounts[i]
	// 	}

	// 	// Set the total amount.
	// 	totalAmountAsBytes := byteutils.ToBytes32FromU64LE(totalAmount)
	// 	vars.SetBytes(&c.OutputBytes, totalAmountAsBytes[:])
	// }
}

func TestSimpleCircuit(t *testing.T) {
	c := NewCircuitFunction(NewTestCircuit())

	build, err := c.Build()
	assert.NoError(t, err)

	// input, err := hex.DecodeString("000000000000000000000000000000000000000000000000000000000000004500000000000000000000000000000000000000000000000000000000000001a4")
	input, err := hex.DecodeString("4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a")
	assert.NoError(t, err)

	proof, err := c.Prove(input, build)
	assert.NoError(t, err)

	expectedInputHash, err := hex.DecodeString("9c12cfdc04c74584d787ac3d23772132c18524bc7ab28dec4219b8fc5b425f70")
	// expectedOutputHash, err := hex.DecodeString("9c12cfdc04c74584d787ac3d23772132c18524bc7ab28dec4219b8fc5b425f70")
	// assert.True(t, bytes.Equal(proof.OutputHash.Bytes(), sha256utils.HashAndTruncate(expectedOutputHash, 253).Bytes()))
	fmt.Println("hash", hex.EncodeToString(proof.InputHash.Bytes()))
	truncated := byteutils.TruncateBytes32([32]byte(expectedInputHash), 253)
	assert.True(t, bytes.Equal(proof.InputHash.Bytes(), truncated[:]))
	fmt.Println("hash", hex.EncodeToString(truncated[:]))
}
