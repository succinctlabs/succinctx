package ssz_test

import (
	"fmt"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/succinctlabs/gnark-gadgets/ssz"
	"github.com/succinctlabs/gnark-gadgets/succinct"
	"github.com/succinctlabs/gnark-gadgets/vars"
)

type TestData struct {
	depth  int
	gindex int
	root   []byte
	leaf   []byte
	proof  [][]byte
}

// Test case from https://github.com/succinctlabs/telepathy-contracts/blob/main/test/libraries/SimpleSerialize.t.sol#L37
func GetTestData() TestData {
	var depth = 6
	gindex := 105
	root := hexutil.MustDecode("0xe81a65c5c0f2a36e40b6872fcfdd62dbb67d47f3d49a6b978c0d4440341e723f")
	leaf := hexutil.MustDecode("0xd85d3181f1178b07e89691aa2bfcd4d88837f011fcda3326b4ce9a68ec6d9e44")
	branch := make([][]byte, depth)
	branch[0] =
		hexutil.MustDecode("0xe424020000000000000000000000000000000000000000000000000000000000")
	branch[1] = hexutil.MustDecode("0x75410a8f37f9506fb3f972cce6ece955e381e51037e432ce4ca47479c9cd9158")
	branch[2] =
		hexutil.MustDecode("0xe6af38835c0ac3c2b0d561dfaec168171d7d77c1c2e8e74ff9b1891cf43faf8d")
	branch[3] =
		hexutil.MustDecode("0x3e4fb2d12bd835bc6ee23b5ec65a43f4493e32f5ef45d46bd2c38830b17672bb")
	branch[4] =
		hexutil.MustDecode("0x880548f4df2d4003f7be2fbbde112eb46b8f756b5e33202e04863000e4383f3b")
	branch[5] = hexutil.MustDecode("0x88475251bcec25245a44bddd92b2c36db6c9c48bc6d91b5d0da78af3229ff783")
	return TestData{depth, gindex, root, leaf, branch}
}

type TestSimpleSerializeCircuit struct {
	Root   [32]vars.Byte
	Leaf   [32]vars.Byte
	Proof  [][32]vars.Byte
	GIndex int
	Depth  int
}

func (circuit *TestSimpleSerializeCircuit) Define(api frontend.API) error {
	leaf := circuit.Leaf
	proof := circuit.Proof
	root := circuit.Root
	gindex := circuit.GIndex
	depth := circuit.Depth

	succinctAPI := succinct.NewAPI(api)
	sszAPI := ssz.NewAPI(succinctAPI)
	sszAPI.VerifyProof(root, leaf, proof, gindex, depth)
	return nil
}

func TestCircuit(t *testing.T) {
	testData := GetTestData()
	fmt.Printf("%#v", testData)
	circuit := &TestSimpleSerializeCircuit{
		Leaf:   vars.NewBytes32(testData.leaf),
		Proof:  vars.NewBytes32Array(testData.proof),
		Root:   vars.NewBytes32(testData.root),
		GIndex: testData.gindex,
		Depth:  testData.depth,
	}
	assignment := &TestSimpleSerializeCircuit{
		Leaf:   vars.NewBytes32(testData.leaf),
		Proof:  vars.NewBytes32Array(testData.proof),
		Root:   vars.NewBytes32(testData.root),
		GIndex: testData.gindex,
		Depth:  testData.depth,
	}
	err := test.IsSolved(circuit, assignment, ecc.BN254.ScalarField())
	if err != nil {
		t.Errorf("assignment should be valid")
	}

	badAssignment := &TestSimpleSerializeCircuit{
		Leaf:   vars.NewBytes32(testData.leaf),
		Proof:  vars.NewBytes32Array(testData.proof),
		Root:   vars.NewBytes32(make([]byte, 32)),
		GIndex: testData.gindex,
		Depth:  testData.depth,
	}
	err = test.IsSolved(circuit, badAssignment, ecc.BN254.ScalarField())
	if err == nil {
		t.Errorf("badAssignment should be invalid")
	}
}
