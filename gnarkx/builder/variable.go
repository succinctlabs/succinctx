package builder

import (
	"github.com/consensys/gnark/frontend"
	"github.com/succinctlabs/sdk/gnarkx/vars"
)

// Add returns res = i1+i2+...in.
func (a *API) Add(i1 vars.Variable, i2 vars.Variable, in ...vars.Variable) vars.Variable {
	els := make([]frontend.Variable, len(in))
	for i := 0; i < len(in); i++ {
		els[i] = in[i].Value
	}
	return vars.Variable{Value: a.api.Add(i1.Value, i2.Value, els...)}
}

// Sub returns res = i1 - i2 - ...in.
func (a *API) Sub(i1 vars.Variable, i2 vars.Variable, in ...vars.Variable) vars.Variable {
	els := make([]frontend.Variable, len(in))
	for i := 0; i < len(in); i++ {
		els[i] = in[i].Value
	}
	return vars.Variable{Value: a.api.Sub(i1.Value, i2.Value, els...)}
}

// Mul returns res = i1 * i2 * ... in.
func (a *API) Mul(i1 vars.Variable, i2 vars.Variable, in ...vars.Variable) vars.Variable {
	els := make([]frontend.Variable, len(in))
	for i := 0; i < len(in); i++ {
		els[i] = in[i].Value
	}
	return vars.Variable{Value: a.api.Mul(i1.Value, i2.Value, els...)}
}

// Div returns i1 / i2.
func (a *API) Div(i1 vars.Variable, i2 vars.Variable) vars.Variable {
	return vars.Variable{Value: a.api.Div(i1.Value, i2.Value)}
}

// Neg returns -i.
func (a *API) Neg(i1 vars.Variable) vars.Variable {
	return vars.Variable{Value: a.api.Neg(i1.Value)}
}

// Inverse returns 1/i.
func (a *API) Inverse(i1 vars.Variable) vars.Variable {
	return vars.Variable{Value: a.api.Inverse(i1.Value)}
}

// Select if b is true, yields i1 else yields i2.
func (a *API) Select(selector vars.Bool, i1 vars.Variable, i2 vars.Variable) vars.Variable {
	return vars.Variable{Value: a.api.Select(selector.Value.Value, i1.Value, i2.Value)}
}

// Lookup2 performs a 2-bit lookup between i1, i2, i3, i4 based on bits b0
// and b1. Returns i0 if b0=b1=0, i1 if b0=1 and b1=0, i2 if b0=0 and b1=1
// and i3 if b0=b1=1.
func (a *API) Lookup2(b1, b2 vars.Variable, i1, i2, i3, i4 vars.Variable) vars.Variable {
	return vars.Variable{Value: a.api.Lookup2(b1.Value, b2.Value, i1.Value, i2.Value, i3.Value, i4.Value)}
}

// IsZero returns 1 if a is zero, 0 otherwise.
func (a *API) IsZero(i1 vars.Variable) vars.Bool {
	return vars.Bool{Value: vars.Variable{Value: a.api.IsZero(i1.Value)}}
}

// Cmp returns 1 if i1>i2, 0 if i1=i2, -1 if i1<i2
func (a *API) Cmp(i1, i2 vars.Variable) vars.Variable {
	return vars.Variable{Value: a.api.Cmp(i1.Value, i2.Value)}
}

// AssertIsEqual fails if i1 != i2
func (a *API) AssertIsEqual(i1, i2 vars.Variable) {
	a.api.AssertIsEqual(i1.Value, i2.Value)
}

// AssertIsDifferent fails if i1 == i2.
func (a *API) AssertIsDifferent(i1, i2 vars.Variable) {
	a.api.AssertIsDifferent(i1.Value, i2.Value)
}

// AssertIsBoolean fails if v != 0 ∥ v != 1.
func (a *API) AssertIsBoolean(i1 vars.Variable) {
	a.api.AssertIsBoolean(i1.Value)
}

// AssertIsLessOrEqual fails if i1 > i2.
func (a *API) AssertIsLessOrEqual(i1, i2 vars.Variable) {
	a.api.AssertIsLessOrEqual(i1.Value, i2.Value)
}
