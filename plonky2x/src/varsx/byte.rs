use itertools::Itertools;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::PartitionWitness;

use super::{BoolVariable, CircuitVariable};
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a byte value. Under the hood, it is represented as
/// eight bits stored in big endian.
pub struct ByteVariable(Vec<BoolVariable>);

impl CircuitVariable for ByteVariable {
    type Value = u8;

    fn init(builder: &mut CircuitBuilder) -> Self {
        Self((0..8).map(|_| BoolVariable::init(builder)).collect_vec())
    }

    fn constant(builder: &mut CircuitBuilder, value: u8) -> Self {
        let value_be_bits = (0..8).map(|i| ((1 << (7 - i)) & value) != 0);
        let targets_be_bits = value_be_bits
            .map(|bit| BoolVariable::constant(builder, bit))
            .collect();
        Self(targets_be_bits)
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> u8 {
        let mut acc: u64 = 0;
        for i in 0..8 {
            let term = (1 << (7 - i)) * (BoolVariable::value(&self.0[i], witness) as u64);
            acc += term;
        }
        acc as u8
    }

    fn set(&self, buffer: &mut GeneratedValues<GoldilocksField>, value: u8) {
        let value_be_bits = (0..8)
            .map(|i| ((1 << (7 - i)) & value) != 0)
            .collect::<Vec<_>>();
        for i in 0..8 {
            BoolVariable::set(&self.0[i], buffer, value_be_bits[i]);
        }
    }
}
