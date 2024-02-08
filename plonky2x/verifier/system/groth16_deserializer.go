package system

import (
	"encoding/json"
	"io"

	"github.com/consensys/gnark-crypto/ecc/bn254/fp"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr/iop"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark/backend/groth16"
	groth16Bn254 "github.com/consensys/gnark/backend/groth16/bn254"
)

// func readV08ProvingKey(pk *groth16Bn254.ProvingKey, r io.Reader) (int64, error) {
// 	dec := bn254.NewDecoder(r)

// 	// Read domain
// 	if err := dec.Decode(&pk.Domain); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.Domain", pk.Domain)
// 	// Read G1 (Alpha, Beta, Delta, A, B, Z, K)
// 	if err := dec.Decode(&pk.G1.Alpha); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G1.Alpha", pk.G1.Alpha)
// 	if err := dec.Decode(&pk.G1.Beta); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G1.Beta", pk.G1.Beta)
// 	if err := dec.Decode(&pk.G1.Delta); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G1.Delta", pk.G1.Delta)
// 	if err := dec.Decode(&pk.G1.A); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	if err := dec.Decode(&pk.G1.B); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G1.B", pk.G1.B)
// 	if err := dec.Decode(&pk.G1.Z); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	if err := dec.Decode(&pk.G1.K); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G1.K", pk.G1.K)

// 	// Read G2 (Beta, Delta, B)
// 	if err := dec.Decode(&pk.G2.Beta); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G2.Beta", pk.G2.Beta)
// 	if err := dec.Decode(&pk.G2.Delta); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G2.Delta", pk.G2.Delta)
// 	if err := dec.Decode(&pk.G2.B); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.G2.B", pk.G2.B)

// 	// Read InfinityA, InfinityB, NbInfinityA, NbInfinityB
// 	if err := dec.Decode(&pk.InfinityA); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.InfinityA", pk.InfinityA)
// 	if err := dec.Decode(&pk.InfinityB); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.InfinityB", pk.InfinityB)
// 	if err := dec.Decode(&pk.NbInfinityA); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.NbInfinityA", pk.NbInfinityA)
// 	if err := dec.Decode(&pk.NbInfinityB); err != nil {
// 		return dec.BytesRead(), err
// 	}
// 	fmt.Println("pk.NbInfinityB", pk.NbInfinityB)

// 	return dec.BytesRead(), nil
// }

func readV08ProvingKey(pk *groth16Bn254.ProvingKey, r io.Reader) (int64, error) {
	pk.Vk = &groth16.VerifyingKey{}
	n, err := pk.Vk.ReadFrom(r)
	if err != nil {
		return n, err
	}

	n2, err, chDomain0 := pk.Domain[0].AsyncReadFrom(r)
	n += n2
	if err != nil {
		return n, err
	}

	n2, err, chDomain1 := pk.Domain[1].AsyncReadFrom(r)
	n += n2
	if err != nil {
		return n, err
	}

	if withSubgroupChecks {
		n2, err = pk.Kzg.ReadFrom(r)
	} else {
		n2, err = pk.Kzg.UnsafeReadFrom(r)
	}
	n += n2
	if err != nil {
		return n, err
	}
	if withSubgroupChecks {
		n2, err = pk.KzgLagrange.ReadFrom(r)
	} else {
		n2, err = pk.KzgLagrange.UnsafeReadFrom(r)
	}
	n += n2
	if err != nil {
		return n, err
	}

	pk.trace.S = make([]int64, 3*pk.Domain[0].Cardinality)

	dec := curve.NewDecoder(r)

	var ql, qr, qm, qo, qk, s1, s2, s3 []fr.Element
	var qcp [][]fr.Element

	// TODO @gbotrel: this is a bit ugly, we should probably refactor this.
	// The order of the variables is important, as it matches the order in which they are
	// encoded in the WriteTo(...) method.

	// Note: instead of calling dec.Decode(...) for each of the above variables,
	// we call AsyncReadFrom when possible which allows to consume bytes from the reader
	// and perform the decoding in parallel

	type v struct {
		data  *fr.Vector
		chErr chan error
	}

	vectors := make([]v, 8)
	vectors[0] = v{data: (*fr.Vector)(&ql)}
	vectors[1] = v{data: (*fr.Vector)(&qr)}
	vectors[2] = v{data: (*fr.Vector)(&qm)}
	vectors[3] = v{data: (*fr.Vector)(&qo)}
	vectors[4] = v{data: (*fr.Vector)(&qk)}
	vectors[5] = v{data: (*fr.Vector)(&s1)}
	vectors[6] = v{data: (*fr.Vector)(&s2)}
	vectors[7] = v{data: (*fr.Vector)(&s3)}

	// read ql, qr, qm, qo, qk
	for i := 0; i < 5; i++ {
		n2, err, ch := vectors[i].data.AsyncReadFrom(r)
		n += n2
		if err != nil {
			return n, err
		}
		vectors[i].chErr = ch
	}

	// read qcp
	if err := dec.Decode(&qcp); err != nil {
		return n + dec.BytesRead(), err
	}

	// read lqk, s1, s2, s3
	for i := 5; i < 8; i++ {
		n2, err, ch := vectors[i].data.AsyncReadFrom(r)
		n += n2
		if err != nil {
			return n, err
		}
		vectors[i].chErr = ch
	}

	// read pk.Trace.S
	if err := dec.Decode(&pk.trace.S); err != nil {
		return n + dec.BytesRead(), err
	}

	// wait for all AsyncReadFrom(...) to complete
	for i := range vectors {
		if err := <-vectors[i].chErr; err != nil {
			return n, err
		}
	}

	canReg := iop.Form{Basis: iop.Canonical, Layout: iop.Regular}
	pk.trace.Ql = iop.NewPolynomial(&ql, canReg)
	pk.trace.Qr = iop.NewPolynomial(&qr, canReg)
	pk.trace.Qm = iop.NewPolynomial(&qm, canReg)
	pk.trace.Qo = iop.NewPolynomial(&qo, canReg)
	pk.trace.Qk = iop.NewPolynomial(&qk, canReg)
	pk.trace.S1 = iop.NewPolynomial(&s1, canReg)
	pk.trace.S2 = iop.NewPolynomial(&s2, canReg)
	pk.trace.S3 = iop.NewPolynomial(&s3, canReg)

	pk.trace.Qcp = make([]*iop.Polynomial, len(qcp))
	for i := range qcp {
		pk.trace.Qcp[i] = iop.NewPolynomial(&qcp[i], canReg)
	}

	// wait for FFT to be precomputed
	<-chDomain0
	<-chDomain1

	return n + dec.BytesRead(), nil

}

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
