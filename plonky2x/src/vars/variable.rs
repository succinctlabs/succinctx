use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::{CircuitBuilder, ExtendableField};

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
pub struct Variable(pub Target);

impl<F: ExtendableField> CircuitVariable<F> for Variable {
    type ValueType = F;

    fn init(builder: &mut CircuitBuilder<F>) -> Self {
        let target = builder.api.add_virtual_target();
        Self(target)
    }

    fn constant(builder: &mut CircuitBuilder<F>, value: F) -> Self {
        let target = builder.api.constant(value);
        Self(target)
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, F>) -> F {
        witness.get_target(self.0)
    }

    fn set(&self, buffer: &mut GeneratedValues<F>, value: Self::ValueType) {
        buffer.set_target(self.0, value);
    }
}

impl From<Target> for Variable {
    fn from(target: Target) -> Self {
        Self(target)
    }
}
