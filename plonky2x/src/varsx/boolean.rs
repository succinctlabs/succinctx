use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::BasicVariable;
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a boolean value.
pub struct BoolVariable(Target);

impl BasicVariable for BoolVariable {
    type Value = bool;

    fn init(builder: &mut CircuitBuilder) -> Self {
        let target = builder.api.add_virtual_target();
        Self(target)
    }

    fn constant(builder: &mut CircuitBuilder, value: Self::Value) -> Self {
        let target = builder
            .api
            .constant(GoldilocksField::from_canonical_u64(value as u64));
        Self(target)
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> Self::Value {
        witness.get_target(self.0) == GoldilocksField::from_canonical_u64(1)
    }

    fn set(&self, buffer: &mut GeneratedValues<GoldilocksField>, value: Self::Value) {
        buffer.set_target(self.0, GoldilocksField::from_canonical_u64(value as u64));
    }
}
