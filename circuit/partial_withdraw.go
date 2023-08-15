package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/succinctlabs/sdk/gnarkx/utils/sszutils"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

// A variable in a circuit representing a partial withdrawal.
type PartialWithdrawalVariable struct {
	Index          [32]vars.Byte
	ValidatorIndex [32]vars.Byte
	Address        [32]vars.Byte
	Amount         vars.U64
}

// The response from the API call to get the partial withdrawals proof.
type PartialWithdrawalsProofResponse struct {
	Success bool `json:"success"`
	Result  struct {
		BlockRoot                         string     `json:"blockRoot"`
		Slot                              int        `json:"slot"`
		WithdrawalsRoot                   string     `json:"withdrawalsRoot"`
		WithdrawalsRootProof              []string   `json:"withdrawalsRootProof"`
		WithdrwalsRootGIndex              string     `json:"withdrwalsRootGIndex"`
		PartialWithdrawalIndexes          []string   `json:"partialWithdrawalIndexes"`
		PartialWithdrawalValidatorIndexes []string   `json:"partialWithdrawalValidatorIndexes"`
		PartialWithdrawalAddresses        []string   `json:"partialWithdrawalAddresses"`
		PartialWithdrawalAmounts          []string   `json:"partialWithdrawalAmounts"`
		PartialWithdrawalProofs           [][]string `json:"partialWithdrawalProofs"`
	} `json:"result"`
}

// The parsed data from the API call to get the partial withdrawals proof.
type PartialWithdrawalsProofData struct {
	BlockRoot                         []byte     `json:"blockRoot"`
	Slot                              int        `json:"slot"`
	WithdrawalsRoot                   []byte     `json:"withdrawalsRoot"`
	WithdrawalsRootProof              [][]byte   `json:"withdrawalsRootProof"`
	PartialWithdrawalIndexes          []uint64   `json:"partialWithdrawalIndexes"`
	PartialWithdrawalValidatorIndexes []uint64   `json:"partialWithdrawalValidatorIndexes"`
	PartialWithdrawalAddresses        [][]byte   `json:"partialWithdrawalAddresses"`
	PartialWithdrawalAmounts          []uint64   `json:"partialWithdrawalAmounts"`
	PartialWithdrawalProofs           [][][]byte `json:"partialWithdrawalProofs"`
}

// Gets the partial withdrawals proof from the API.
func GetPartialWithdrawalsProofFromAPI(blockRoot string) (*PartialWithdrawalsProofData, error) {
	endpoint := "https://platform-beaconapi.vercel.app/api/partialWithdrawalsProof/" + blockRoot

	resp, err := http.Get(endpoint)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var data PartialWithdrawalsProofResponse
	err = json.Unmarshal(body, &data)
	if err != nil {
		return nil, err
	}

	var w PartialWithdrawalsProofData
	w.BlockRoot, _ = decodeHex(data.Result.BlockRoot)
	w.WithdrawalsRoot, _ = decodeHex(data.Result.WithdrawalsRoot)
	w.WithdrawalsRootProof = decodeHexSlice(data.Result.WithdrawalsRootProof)
	w.PartialWithdrawalAddresses = decodeHexSlice(data.Result.PartialWithdrawalAddresses)
	w.PartialWithdrawalIndexes = stringToUint64Slice(data.Result.PartialWithdrawalIndexes)
	w.PartialWithdrawalValidatorIndexes = stringToUint64Slice(data.Result.PartialWithdrawalValidatorIndexes)
	w.PartialWithdrawalAmounts = stringToUint64Slice(data.Result.PartialWithdrawalAmounts)
	w.PartialWithdrawalProofs = make([][][]byte, len(data.Result.PartialWithdrawalProofs))
	for i, proofs := range data.Result.PartialWithdrawalProofs {
		w.PartialWithdrawalProofs[i] = decodeHexSlice(proofs)
	}

	var blockRoot2 [32]byte
	copy(blockRoot2[:], w.BlockRoot)

	var withdrawalsRoot [32]byte
	copy(withdrawalsRoot[:], w.WithdrawalsRoot)

	var withdrawalsRootProof [][32]byte
	for _, proof := range w.WithdrawalsRootProof {
		var p [32]byte
		copy(p[:], proof)
		withdrawalsRootProof = append(withdrawalsRootProof, p)
	}

	sszutils.VerifyProof(
		blockRoot2,
		withdrawalsRoot,
		withdrawalsRootProof,
		3230,
	)
	return &w, nil
}
