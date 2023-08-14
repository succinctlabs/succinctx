use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::BasicVariable;
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a byte value. Under the hood, it is represented as
/// eight bits stored in big endian.
pub struct ByteVariable(Vec<Target>);

impl BasicVariable for ByteVariable {
    type Value = u8;

    fn init(builder: &mut CircuitBuilder) -> Self {
        Self((0..8).map(|_| builder.api.add_virtual_target()).collect())
    }

    fn constant(builder: &mut CircuitBuilder, value: Self::Value) -> Self {
        let value_be_bits = (0..8).map(|i| ((1 << (7 - i)) & value) != 0);
        let targets_be_bits = value_be_bits
            .map(|bit| {
                let f = GoldilocksField::from_canonical_u64(bit as u64);
                builder.api.constant(f)
            })
            .collect();
        Self(targets_be_bits)
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> Self::Value {
        let mut acc: u64 = 0;
        for i in 0..8 {
            let term = (1 << (7 - i)) * witness.get_target(self.0[i]).0;
            acc += term;
        }
        acc as u8
    }

    fn set(&self, buffer: &mut GeneratedValues<GoldilocksField>, value: Self::Value) {
        let value_be_bits = (0..8)
            .map(|i| ((1 << (7 - i)) & value) != 0)
            .collect::<Vec<_>>();
        for i in 0..8 {
            let f = GoldilocksField::from_canonical_u64(value_be_bits[i] as u64);
            buffer.set_target(self.0[i], f);
        }
    }
}
