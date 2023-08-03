package vars

// A variable in a circuit representing a u64.
type U64 struct {
	Value Variable
}

// Creates a new u64 as a variable in a circuit.
func NewU64() U64 {
	return U64{Value: ZERO}
}

func (u *U64) Set(i1 uint64) {
	u.Value = NewVariableFromInt(int(i1))
}
