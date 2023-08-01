package sha512

import (
	"encoding/hex"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
)

type TestSha512Circuit struct {
	In  []frontend.Variable `gnark:"in"`
	Out []frontend.Variable `gnark:"out"`
}

func (circuit *TestSha512Circuit) Define(api frontend.API) error {
	res := Sha512(api, circuit.In)
	if len(res) != 512 {
		panic("bad length")
	}
	for i := 0; i < 512; i++ {
		api.AssertIsEqual(res[i], circuit.Out[i])
	}
	return nil
}

var testCurve = ecc.BN254

func TestSha512Witness(t *testing.T) {
	assert := test.NewAssert(t)

	testCase := func(in []byte, output string) {
		out, err := hex.DecodeString(output)
		if err != nil {
			panic(err)
		}
		if len(out) != 512/8 {
			panic("bad output length")
		}

		circuit := TestSha512Circuit{
			In:  toBits(in),
			Out: toBits(out),
		}
		witness := TestSha512Circuit{
			In:  toBits(in),
			Out: toBits(out),
		}
		err = test.IsSolved(&circuit, &witness, testCurve.ScalarField())
		assert.NoError(err)
	}

	testCase([]byte(""), "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e")
	testCase([]byte("Succinct Labs"), "503ace098aa03f6feec1b5df0a38aee923f744a775508bc81f2b94ad139be297c2e8cd8c44af527b5d3f017a7fc929892c896604047e52e3f518924f52bff0dc")
	testCase(decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273"), "4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39")
}

func toBits(arr []byte) []frontend.Variable {
	result := make([]frontend.Variable, len(arr)*8)
	for i, v := range arr {
		for j := 0; j < 8; j++ {
			if (v & (1 << (7 - j))) != 0 {
				result[i*8+j] = 1
			} else {
				result[i*8+j] = 0
			}
		}
	}
	return result
}

func decode(s string) []byte {
	result, err := hex.DecodeString(s)
	if err != nil {
		panic(err)
	}
	return result
}
