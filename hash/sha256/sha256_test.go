package sha256

import (
	"encoding/hex"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
)

type TestSha256Circuit struct {
	In  []frontend.Variable `gnark:"in"`
	Out []frontend.Variable `gnark:"out"`
}

func (circuit *TestSha256Circuit) Define(api frontend.API) error {
	res := Hash(api, circuit.In)
	if len(res) != 256 {
		panic("bad length")
	}
	for i := 0; i < 256; i++ {
		api.AssertIsEqual(res[i], circuit.Out[i])
	}
	return nil
}

var testCurve = ecc.BN254

func TestSha256Witness(t *testing.T) {
	assert := test.NewAssert(t)

	testCase := func(in []byte, output string) {
		out, err := hex.DecodeString(output)
		if err != nil {
			panic(err)
		}
		if len(out) != 256/8 {
			panic("bad output length")
		}

		circuit := TestSha256Circuit{
			In:  toBits(in),
			Out: toBits(out),
		}
		witness := TestSha256Circuit{
			In:  toBits(in),
			Out: toBits(out),
		}
		err = test.IsSolved(&circuit, &witness, testCurve.ScalarField())
		assert.NoError(err)
	}

	testCase([]byte("Succinct Labs"), "7fb4acc57b9765e167a716dee0d19c5dce851cfa140dbce7fff42a3e589ab470")
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
