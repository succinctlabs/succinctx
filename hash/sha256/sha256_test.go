package sha256

import (
	"encoding/hex"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
	"github.com/succinctlabs/gnark-gadgets/bitutils"
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
			In:  bitutils.ToBits(in),
			Out: bitutils.ToBits(out),
		}
		witness := TestSha256Circuit{
			In:  bitutils.ToBits(in),
			Out: bitutils.ToBits(out),
		}
		err = test.IsSolved(&circuit, &witness, ecc.BN254.ScalarField())
		assert.NoError(err)
	}

	testCase([]byte("Succinct Labs"), "7fb4acc57b9765e167a716dee0d19c5dce851cfa140dbce7fff42a3e589ab470")
	testCase([]byte("i love polynomials"), "f9d31346a1b4b014dcdd3d9c700f7c4a017383ac8fb6502257a58596011b598f")
	testCase([]byte("jtguibas"), "11490498ac6480d6fefe1c01e639875cee3b4ec3f96265eb76701f65da99ea8c")
}

func TestSha256Proof(t *testing.T) {
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
			In:  bitutils.ToBits(in),
			Out: bitutils.ToBits(out),
		}
		witness := TestSha256Circuit{
			In:  bitutils.ToBits(in),
			Out: bitutils.ToBits(out),
		}
		assert.ProverSucceeded(&circuit, &witness, test.WithBackends(backend.GROTH16), test.NoFuzzing())
		assert.NoError(err)
	}

	testCase([]byte("Succinct Labs"), "7fb4acc57b9765e167a716dee0d19c5dce851cfa140dbce7fff42a3e589ab470")
}
