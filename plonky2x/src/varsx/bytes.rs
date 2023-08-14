use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::BasicVariable;
use crate::builder::CircuitBuilder;
use crate::varsx::ByteVariable;

/// A variable in the circuit representing a byte value.
pub struct BytesVariable<const N: usize>(Vec<ByteVariable>);

impl<const N: usize> BasicVariable for BytesVariable<N> {
    type Value = Vec<u8>;

    fn init(builder: &mut CircuitBuilder) -> Self {
        Self((0..N).map(|_| ByteVariable::init(builder)).collect())
    }

    fn constant(builder: &mut CircuitBuilder, value: Self::Value) -> Self {
        Self(
            value
                .into_iter()
                .map(|b| ByteVariable::constant(builder, b))
                .collect(),
        )
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> Self::Value {
        self.0.iter().map(|b| b.value(witness)).collect()
    }

    fn set(&self, buffer: &mut GeneratedValues<GoldilocksField>, value: Self::Value) {
        for (b, v) in self.0.iter().zip(value) {
            b.set(buffer, v);
        }
    }
}
