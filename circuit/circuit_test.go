package main

import (
	"bytes"
	"fmt"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/consensys/gnark/test"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/succinctlabs/sdk/gnarkx/succinct"
	"github.com/succinctlabs/sdk/gnarkx/utils/byteutils"
	"github.com/succinctlabs/sdk/gnarkx/utils/sszutils"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

// Test the circuit without generating a proof, but by just simulating the output with big.Int's
// under the hood.
func TestIsSolved(t *testing.T) {
	blockRoot := "0x6de59dc86b36b81bdae8cfdf9c9283e06fc78234a62cac274f2bef1fd1cfd209"
	inputBytes := hexutil.MustDecode(blockRoot)

	circuit := succinct.NewCircuitFunction(NewCircuit())
	assignment := succinct.NewCircuitFunction(NewCircuit())
	assignment.SetWitness(inputBytes)

	// Simulates the output of the circuit with big.Int's under the hood.
	err := test.IsSolved(&circuit, &assignment, ecc.BN254.ScalarField())
	if err != nil {
		panic(fmt.Errorf("circuit is not solved: %w", err))
	}

	outputBytes := vars.GetValuesUnsafe(*assignment.Circuit.GetOutputBytes())
	totalAmount := uint64(96534268)
	totalAmountAsBytes := byteutils.ToBytes32FromU64LE(totalAmount)
	if bytes.Equal(outputBytes, totalAmountAsBytes[:]) {
		fmt.Println("output bytes are correct")
		fmt.Printf("total amount: %s\n", hexutil.Encode(totalAmountAsBytes[:]))
	} else {
		fmt.Println()
		panic("output bytes are incorrect")
	}
}

// Test the end-to-end proof generation and verification. Run this test with "-tags=debug" to get
// more details.
func TestCircuit(t *testing.T) {
	blockRoot := "0x6de59dc86b36b81bdae8cfdf9c9283e06fc78234a62cac274f2bef1fd1cfd209"
	inputBytes := hexutil.MustDecode(blockRoot)

	circuit := NewCircuit()
	assignment := NewCircuit()
	assignment.SetWitness(inputBytes)

	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, circuit)
	if err != nil {
		panic(err)
	}

	witness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		panic(err)
	}

	pk, err := groth16.DummySetup(r1cs)
	if err != nil {
		panic(err)
	}

	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		panic(err)
	}

	fmt.Println(proof)
}

func TestGetWithdrawalsData(t *testing.T) {
	blockRoot := "0x6de59dc86b36b81bdae8cfdf9c9283e06fc78234a62cac274f2bef1fd1cfd209"
	wd, err := GetPartialWithdrawalsProofFromAPI(blockRoot)
	if err != nil {
		panic(err)
	}
	fmt.Println(hexutil.Encode(wd.BlockRoot))
	fmt.Println(hexutil.Encode(wd.WithdrawalsRoot))
}

func TestLogicOutOfCircuit(t *testing.T) {
	blockRoot := "0x6de59dc86b36b81bdae8cfdf9c9283e06fc78234a62cac274f2bef1fd1cfd209"
	wd, err := GetPartialWithdrawalsProofFromAPI(blockRoot)
	if err != nil {
		panic(err)
	}

	withdrawalsRootProof := make([][32]byte, len(wd.WithdrawalsRootProof))
	for i, proof := range wd.WithdrawalsRootProof {
		withdrawalsRootProof[i] = [32]byte(proof)
	}
	sszutils.VerifyProof([32]byte(wd.BlockRoot), [32]byte(wd.WithdrawalsRoot), withdrawalsRootProof, 3230)
	totalAmount := uint64(0)
	withdrawalRoots := make([][32]byte, 4)

	WITHDRAWAL_PROOF_BASE_GINDEX := 103360

	for i := 0; i < 4; i++ {
		withdrawalRoots[i] = sszutils.HashTreeRoot(
			[][32]byte{
				byteutils.ToBytes32FromU64LE(wd.PartialWithdrawalIndexes[i]),
				byteutils.ToBytes32FromU64LE(wd.PartialWithdrawalValidatorIndexes[i]),
				byteutils.ToBytes32FromBytesRightPad(wd.PartialWithdrawalAddresses[i]),
				byteutils.ToBytes32FromU64LE(wd.PartialWithdrawalAmounts[i]),
			},
		)
		partialWithdrawalProof := make([][32]byte, len(wd.PartialWithdrawalProofs[i]))
		for j, proof := range wd.PartialWithdrawalProofs[i] {
			partialWithdrawalProof[j] = [32]byte(proof)
		}

		sszutils.VerifyProof(
			[32]byte(wd.WithdrawalsRoot),
			withdrawalRoots[i],
			partialWithdrawalProof,
			WITHDRAWAL_PROOF_BASE_GINDEX+i,
		)
		totalAmount += wd.PartialWithdrawalAmounts[i]
	}

	if totalAmount != 96534268 {
		panic("total amount is incorrect")
	}
}
