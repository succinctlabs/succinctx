use array_macro::array;
use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{BoolVariable, CircuitVariable};
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a byte value. Under the hood, it is represented as
/// eight bits stored in big endian.
#[derive(Debug, Clone, Copy, Default)]
pub struct ByteVariable(pub [BoolVariable; 8]);

impl CircuitVariable for ByteVariable {
    type ValueType = u8;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => BoolVariable::init(builder); 8])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        let value_be_bits = (0..8).map(|i| ((1 << (7 - i)) & value) != 0).collect_vec();
        Self(array![i => BoolVariable::constant(builder, value_be_bits[i]); 8])
    }

    fn targets(&self) -> Vec<Target> {
        self.0
            .clone()
            .into_iter()
            .map(|x| x.targets())
            .flatten()
            .collect()
    }

    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), 8);
        Self(array![i => BoolVariable::from_targets(&[targets[i]]); 8])
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        let mut acc: u64 = 0;
        for i in 0..8 {
            let term = (1 << (7 - i)) * (BoolVariable::value(&self.0[i], witness) as u64);
            acc += term;
        }
        acc as u8
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        let value_be_bits = (0..8)
            .map(|i| ((1 << (7 - i)) & value) != 0)
            .collect::<Vec<_>>();
        for i in 0..8 {
            BoolVariable::set(&self.0[i], witness, value_be_bits[i]);
        }
    }
}
