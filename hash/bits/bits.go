package bits

import "github.com/consensys/gnark/frontend"

func FromUint32(value uint32) [32]frontend.Variable {
	var result [32]frontend.Variable
	for k := 0; k < 32; k++ {
		if (value & (1 << (31 - k))) != 0 {
			result[k] = 1
		} else {
			result[k] = 0
		}
	}
	return result
}

func Xor2(api frontend.API, a, b [32]frontend.Variable) [32]frontend.Variable {
	var result [32]frontend.Variable
	for i := 0; i < 32; i++ {
		result[i] = api.Xor(a[i], b[i])
	}
	return result
}

func Xor3(api frontend.API, a, b, c [32]frontend.Variable) [32]frontend.Variable {
	api.Println(a[0], b[0], c[0])
	var result [32]frontend.Variable
	for i := 0; i < 32; i++ {
		result[i] = api.Xor(a[i], api.Xor(b[i], c[i]))
	}
	return result
}

func Rotate(n [32]frontend.Variable, bits int) [32]frontend.Variable {
	var result [32]frontend.Variable
	for i := 0; i < len(n); i++ {
		result[(i+bits)%len(n)] = n[i]
	}
	return result
}

func Shr(n [32]frontend.Variable, bits int) [32]frontend.Variable {
	var result [32]frontend.Variable
	for i := 0; i < 32; i++ {
		if i < bits {
			result[i] = 0
		} else {
			result[i] = n[i-bits]
		}
	}
	return result
}

func Add(api frontend.API, args ...[32]frontend.Variable) [32]frontend.Variable {
	if len(args) == 1 {
		return args[0]
	} else {
		return Add2(api, args[0], Add(api, args[1:]...))
	}
}

func Add2(api frontend.API, a, b [32]frontend.Variable) [32]frontend.Variable {
	api.Println(a[0], b[0])
	var result [32]frontend.Variable
	var carry frontend.Variable = 0
	for i := 31; i >= 0; i-- {
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

func And2(api frontend.API, a, b [32]frontend.Variable) [32]frontend.Variable {
	var result [32]frontend.Variable
	for i := 0; i < 32; i++ {
		result[i] = api.And(a[i], b[i])
	}
	return result
}

func Not(api frontend.API, n [32]frontend.Variable) [32]frontend.Variable {
	var result [32]frontend.Variable
	for i := 0; i < 32; i++ {
		result[i] = api.Sub(1, n[i])
	}
	return result
}
