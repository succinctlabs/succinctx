use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::PartitionWitness;

use super::CircuitVariable;
use crate::builder::{CircuitBuilder, ExtendableField};
use crate::vars::ByteVariable;

/// A variable in the circuit representing a byte value.
pub struct BytesVariable<const N: usize>(pub Vec<ByteVariable>);

impl<F: ExtendableField, const N: usize> CircuitVariable<F> for BytesVariable<N> {
    type ValueType = Vec<u8>;

    fn init(builder: &mut CircuitBuilder<F>) -> Self {
        Self((0..N).map(|_| ByteVariable::init(builder)).collect())
    }

    fn constant(builder: &mut CircuitBuilder<F>, value: Vec<u8>) -> Self {
        assert!(value.len() == N, "vector of values has wrong length");
        Self(
            value
                .into_iter()
                .map(|b| ByteVariable::constant(builder, b))
                .collect(),
        )
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, F>) -> Vec<u8> {
        self.0.iter().map(|b| b.value(witness)).collect()
    }

    fn set(&self, buffer: &mut GeneratedValues<F>, value: Vec<u8>) {
        assert!(value.len() == N, "vector of values has wrong length");
        for (b, v) in self.0.iter().zip(value) {
            b.set(buffer, v);
        }
    }
}
