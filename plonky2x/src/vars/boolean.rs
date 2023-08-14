use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::builder::{CircuitBuilder, ExtendableField};

/// A variable in the circuit representing a boolean value.
pub struct BoolVariable(pub Variable);

impl<F: ExtendableField> CircuitVariable<F> for BoolVariable {
    type ValueType = bool;

    fn init(builder: &mut CircuitBuilder<F>) -> Self {
        Self(Variable::init(builder))
    }

    fn constant(builder: &mut CircuitBuilder<F>, value: bool) -> Self {
        Self(Variable::constant(
            builder,
            F::from_canonical_u64(value as u64),
        ))
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, F>) -> bool {
        witness.get_target(self.0 .0) == F::from_canonical_u64(1)
    }

    fn set(&self, buffer: &mut GeneratedValues<F>, value: bool) {
        buffer.set_target(self.0 .0, F::from_canonical_u64(value as u64));
    }
}

impl From<Target> for BoolVariable {
    fn from(v: Target) -> Self {
        Self(Variable(v))
    }
}

impl From<Variable> for BoolVariable {
    fn from(v: Variable) -> Self {
        Self(v)
    }
}
