package sha512

/* Based on https://gist.github.com/illia-v/7883be942da5d416521375004cecb68f */

import (
	"github.com/consensys/gnark/frontend"
)

func Sha512(api frontend.API, in []frontend.Variable) [512]frontend.Variable {
	_not := func(x [64]frontend.Variable) [64]frontend.Variable {
		return not(api, x)
	}
	_and := func(xs ...[64]frontend.Variable) [64]frontend.Variable {
		return and(api, xs...)
	}
	_add := func(xs ...[64]frontend.Variable) [64]frontend.Variable {
		return add(api, xs...)
	}
	_xor := func(xs ...[64]frontend.Variable) [64]frontend.Variable {
		return xor(api, xs...)
	}
	zip_add := func(a, b Array8_64) Array8_64 {
		a0, a1, a2, a3, a4, a5, a6, a7 := unpack8(a)
		b0, b1, b2, b3, b4, b5, b6, b7 := unpack8(b)
		return Array8_64{
			_add(a0, b0),
			_add(a1, b1),
			_add(a2, b2),
			_add(a3, b3),
			_add(a4, b4),
			_add(a5, b5),
			_add(a6, b6),
			_add(a7, b7),
		}
	}
	initial_hash := []uint64{
		0x6a09e667f3bcc908,
		0xbb67ae8584caa73b,
		0x3c6ef372fe94f82b,
		0xa54ff53a5f1d36f1,
		0x510e527fade682d1,
		0x9b05688c2b3e6c1f,
		0x1f83d9abfb41bd6b,
		0x5be0cd19137e2179,
	}
	round_constants := []uint64{
		0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f,
		0xe9b5dba58189dbbc, 0x3956c25bf348b538, 0x59f111f1b605d019,
		0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242,
		0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
		0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235,
		0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3,
		0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 0x2de92c6f592b0275,
		0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
		0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f,
		0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725,
		0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc,
		0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
		0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6,
		0x92722c851482353b, 0xa2bfe8a14cf10364, 0xa81a664bbc423001,
		0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218,
		0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
		0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99,
		0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb,
		0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc,
		0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
		0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915,
		0xc67178f2e372532b, 0xca273eceea26619c, 0xd186b8c721c0c207,
		0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba,
		0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
		0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc,
		0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a,
		0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
	}
	for _, v := range in {
		api.AssertIsBoolean(v)
	}
	mdi := divChecked(len(in), 8) % 128
	var padding_len int
	if mdi < 112 {
		padding_len = 119 - mdi
	} else {
		padding_len = 247 - mdi
	}
	message_length_bits := uint64ToBits(uint64(len(in)))
	in = append(in, 1)
	for i := 0; i < 7; i++ {
		in = append(in, 0)
	}
	for i := 0; i < padding_len*8; i++ {
		in = append(in, 0)
	}
	for i := 0; i < 64; i++ {
		in = append(in, message_length_bits[i])
	}

	sha512_hash := Array8_64{
		uint64ToBits(initial_hash[0]),
		uint64ToBits(initial_hash[1]),
		uint64ToBits(initial_hash[2]),
		uint64ToBits(initial_hash[3]),
		uint64ToBits(initial_hash[4]),
		uint64ToBits(initial_hash[5]),
		uint64ToBits(initial_hash[6]),
		uint64ToBits(initial_hash[7]),
	}
	for chunk_start := 0; chunk_start < divChecked(len(in), 8); chunk_start += 128 {
		chunk := in[chunk_start*8 : (chunk_start+128)*8]
		if len(chunk) != 1024 {
			panic("bad length")
		}
		u := make([]frontend.Variable, 80*64)
		for i, _ := range u {
			u[i] = 0
		}
		copy(u, chunk)

		w := reshape(u)

		for i := 16; i < 80; i++ {
			s0 := _xor(
				_right_rotate(w[i-15], 1),
				_right_rotate(w[i-15], 8),
				_shr(w[i-15], 7),
			)
			s1 := _xor(
				_right_rotate(w[i-2], 19),
				_right_rotate(w[i-2], 61),
				_shr(w[i-2], 6),
			)
			w[i] = _add(w[i-16], s0, w[i-7], s1)
		}
		a, b, c, d, e, f, g, h := unpack8(sha512_hash)
		for i := 0; i < 80; i++ {
			sum1 := _xor(
				_right_rotate(e, 14),
				_right_rotate(e, 18),
				_right_rotate(e, 41),
			)
			ch := _xor(_and(e, f), _and(_not(e), g))
			temp1 := _add(h, sum1, ch, uint64ToBits(round_constants[i]), w[i])
			sum0 := _xor(
				_right_rotate(a, 28),
				_right_rotate(a, 34),
				_right_rotate(a, 39),
			)
			maj := _xor(_and(a, b), _and(a, c), _and(b, c))
			temp2 := _add(sum0, maj)

			h = g
			g = f
			f = e
			e = _add(d, temp1)
			d = c
			c = b
			b = a
			a = _add(temp1, temp2)
		}
		sha512_hash = zip_add(sha512_hash, Array8_64{a, b, c, d, e, f, g, h})
	}
	return flatten8(sha512_hash)
}

func _right_rotate(n [64]frontend.Variable, bits int) [64]frontend.Variable {
	var result [64]frontend.Variable
	for i := 0; i < len(n); i++ {
		result[(i+bits)%len(n)] = n[i]
	}
	return result
}

func reshape(u []frontend.Variable) [][64]frontend.Variable {
	l := divChecked(len(u), 64)
	result := make([][64]frontend.Variable, l)
	for i := 0; i < l; i++ {
		var arr [64]frontend.Variable
		for k := 0; k < 64; k++ {
			arr[k] = u[i*64+k]
		}
		result[i] = arr
	}
	return result
}

type Array8_64 [8][64]frontend.Variable

func unpack8(x Array8_64) ([64]frontend.Variable, [64]frontend.Variable, [64]frontend.Variable, [64]frontend.Variable, [64]frontend.Variable, [64]frontend.Variable, [64]frontend.Variable, [64]frontend.Variable) {
	return x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]
}

func flatten8(x Array8_64) [512]frontend.Variable {
	var result [512]frontend.Variable
	k := 0
	for i := 0; i < 8; i++ {
		for j := 0; j < 64; j++ {
			result[k] = x[i][j]
			k++
		}
	}
	return result
}

func uint64ToBits(value uint64) [64]frontend.Variable {
	var result [64]frontend.Variable
	for k := 0; k < 64; k++ {
		if (value & (1 << (63 - k))) != 0 {
			result[k] = 1
		} else {
			result[k] = 0
		}
	}
	return result
}

func xor(api frontend.API, args ...[64]frontend.Variable) [64]frontend.Variable {
	if len(args) == 1 {
		return args[0]
	} else {
		return xor2(api, args[0], xor(api, args[1:]...))
	}
}
func add(api frontend.API, args ...[64]frontend.Variable) [64]frontend.Variable {
	if len(args) == 1 {
		return args[0]
	} else {
		return add2(api, args[0], add(api, args[1:]...))
	}
}
func and(api frontend.API, args ...[64]frontend.Variable) [64]frontend.Variable {
	if len(args) == 1 {
		return args[0]
	} else {
		return and2(api, args[0], and(api, args[1:]...))
	}
}

func xor2(api frontend.API, a, b [64]frontend.Variable) [64]frontend.Variable {
	var result [64]frontend.Variable
	for i := 0; i < 64; i++ {
		result[i] = api.Xor(a[i], b[i])
	}
	return result
}

func and2(api frontend.API, a, b [64]frontend.Variable) [64]frontend.Variable {
	var result [64]frontend.Variable
	for i := 0; i < 64; i++ {
		result[i] = api.And(a[i], b[i])
	}
	return result
}

func add2(api frontend.API, a, b [64]frontend.Variable) [64]frontend.Variable {
	var result [64]frontend.Variable
	var carry frontend.Variable = 0
	for i := 63; i >= 0; i-- {
		sum := api.Add(a[i], b[i], carry)
		sumBin := api.ToBinary(sum, 2)
		if len(sumBin) != 2 {
			panic("bad length")
		}
		result[i] = sumBin[0]
		carry = sumBin[1]
	}
	return result
}

func _shr(n [64]frontend.Variable, bits int) [64]frontend.Variable {
	var result [64]frontend.Variable
	for i := 0; i < 64; i++ {
		if i < bits {
			result[i] = 0
		} else {
			result[i] = n[i-bits]
		}
	}
	return result
}

func not(api frontend.API, n [64]frontend.Variable) [64]frontend.Variable {
	var result [64]frontend.Variable
	for i := 0; i < 64; i++ {
		result[i] = api.Sub(1, n[i])
	}
	return result
}

func divChecked(a, b int) int {
	if a%b != 0 {
		panic("divChecked: does not divide evenly")
	}
	return a / b
}
