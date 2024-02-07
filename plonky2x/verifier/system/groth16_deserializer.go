package system

import (
	"encoding/json"
	"io"

	"github.com/consensys/gnark-crypto/ecc/bn254/fp"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	groth16Bn254 "github.com/consensys/gnark/backend/groth16/bn254"
)

func readV08VerifyingKey(vk *groth16Bn254.VerifyingKey, r io.Reader) (int64, error) {
	dec := bn254.NewDecoder(r)
	if err := dec.Decode(&vk.G1.Alpha); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G1.Beta); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G2.Beta); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G2.Gamma); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G1.Delta); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G2.Delta); err != nil {
		return dec.BytesRead(), err
	}

	// uint32(len(Kvk)),[Kvk]1
	if err := dec.Decode(&vk.G1.K); err != nil {
		return dec.BytesRead(), err
	}

	if err := vk.Precompute(); err != nil {
		return dec.BytesRead(), err
	}

	return dec.BytesRead(), nil
}

// G1 projective point coordinates
type g1ProjJson [3]string

// G2 projective point coordinates
type g2ProjJson [3][2]string

type vkJson struct {
	AlphaG1 g1ProjJson `json:"vk_alpha_1"`
	BetaG2  g2ProjJson `json:"vk_beta_2"`
	GammaG2 g2ProjJson `json:"vk_gamma_2"`
	DeltaG2 g2ProjJson `json:"vk_delta_2"`
	// length dependent on circuit public inputs
	G1K []g1ProjJson `json:"IC"`
}

type g2Proj struct {
	X, Y, Z bn254.E2
}

func (g *g2Proj) fromJson(j *g2ProjJson) *g2Proj {
	g.X.SetString(j[0][0], j[0][1])
	g.Y.SetString(j[1][0], j[1][1])
	g.Z.SetString(j[2][0], j[2][1])
	return g
}

func (g *g2Proj) toAffine() bn254.G2Affine {
	res := bn254.G2Affine{}
	if !g.Z.IsZero() {
		res.X.Div(&g.X, &g.Z)
		res.Y.Div(&g.Y, &g.Z)
	}
	return res
}

type g1Proj struct {
	X, Y, Z fp.Element
}

func (g *g1Proj) fromJson(j *g1ProjJson) *g1Proj {
	g.X.SetString(j[0])
	g.Y.SetString(j[1])
	g.Z.SetString(j[2])
	return g
}

func (g *g1Proj) toAffine() bn254.G1Affine {
	res := bn254.G1Affine{}
	if !g.Z.IsZero() {
		res.X.Div(&g.X, &g.Z)
		res.Y.Div(&g.Y, &g.Z)
	}
	return res
}

func readJsonVerifyingKey(vk *groth16Bn254.VerifyingKey, r io.Reader) error {
	data, err := io.ReadAll(r)

	if err != nil {
		return err
	}

	var vkJson vkJson

	err = json.Unmarshal(data, &vkJson)

	if err != nil {
		return err
	}

	vk.G1.Alpha = new(g1Proj).fromJson(&vkJson.AlphaG1).toAffine()

	vk.G2.Beta = new(g2Proj).fromJson(&vkJson.BetaG2).toAffine()
	vk.G2.Gamma = new(g2Proj).fromJson(&vkJson.GammaG2).toAffine()
	vk.G2.Delta = new(g2Proj).fromJson(&vkJson.DeltaG2).toAffine()

	vk.G1.K = make([]bn254.G1Affine, len(vkJson.G1K))
	for i := 0; i < len(vkJson.G1K); i++ {
		vk.G1.K[i] = new(g1Proj).fromJson(&vkJson.G1K[i]).toAffine()
	}

	vk.Precompute()

	return nil
}
