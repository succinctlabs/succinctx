use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
pub struct U32Variable(Target);

impl CircuitVariable for U32Variable {
    type Value = u32;

    fn init(builder: &mut CircuitBuilder) -> Self {
        let target = builder.api.add_virtual_target();
        Self(target)
    }

    fn constant(builder: &mut CircuitBuilder, value: u32) -> Self {
        let target = builder
            .api
            .constant(GoldilocksField::from_canonical_u32(value));
        Self(target)
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> u32 {
        witness.get_target(self.0).0 as u32
    }

    fn set(&self, witness: &mut GeneratedValues<GoldilocksField>, value: u32) {
        witness.set_target(self.0, GoldilocksField::from_canonical_u32(value));
    }
}
