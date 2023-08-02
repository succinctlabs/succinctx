package vars

// A variable in a circuit representing a u64.
type U64 struct {
	Value Variable
}

// Creates a new u64 as a variable in a circuit.
func NewU64(i1 uint64) U64 {
	return U64{Value: Variable{Value: i1}}
}
