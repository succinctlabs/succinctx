package vars

// The zero bit as a variable in a circuit. If used within APIs, it will be treated as a constant.
var FALSE = Bool{Value: ZERO}

// The zero bit as a variable in a circuit. If used within APIs, it will be treated as a constant.
var TRUE = Bool{Value: ONE}

// A variable in a circuit representing a bit. Under the hood, the value is a single field element.
type Bool struct {
	Value Variable
}

// Creates a new bit as a variable in a circuit from a boolean.
func NewBool(i1 bool) Bool {
	if i1 {
		return TRUE
	}
	return FALSE
}

// Creates a new bit as a variable in a circuit.
func NewBoolFromInt(i1 int) Bool {
	if (i1 != 0) && (i1 != 1) {
		panic("x must be 0 or 1")
	}
	return Bool{Value: NewVariableFromInt(i1)}
}

func NewBoolArrayFromU32(value uint32) [32]Bool {
	var result [32]Bool
	for k := 0; k < 32; k++ {
		if (value & (1 << (31 - k))) != 0 {
			result[k] = TRUE
		} else {
			result[k] = FALSE
		}
	}
	return result
}
