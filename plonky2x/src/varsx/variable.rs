use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
pub struct Variable(pub Target);

impl CircuitVariable for Variable {
    type Value = GoldilocksField;

    fn init(builder: &mut CircuitBuilder) -> Self {
        let target = builder.api.add_virtual_target();
        Self(target)
    }

    fn constant(builder: &mut CircuitBuilder, value: Self::Value) -> Self {
        let target = builder.api.constant(value);
        Self(target)
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> Self::Value {
        witness.get_target(self.0)
    }

    fn set(&self, buffer: &mut GeneratedValues<GoldilocksField>, value: Self::Value) {
        buffer.set_target(self.0, value);
    }
}
