use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::vars::ByteVariable;

/// A variable in the circuit representing a byte value.
#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [ByteVariable; N]);

impl<const N: usize> CircuitVariable for BytesVariable<N> {
    type ValueType = Vec<u8>;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => ByteVariable::init(builder); N])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        assert!(
            value.len() == N,
            "vector of values has wrong length: expected {} got {}",
            N,
            value.len()
        );
        Self(array![i => ByteVariable::constant(builder, value[i]); N])
    }

    fn targets(&self) -> Vec<Target> {
        self.0.iter().flat_map(|b| b.targets()).collect()
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        self.0.iter().map(|b| b.value(witness)).collect()
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        assert!(
            value.len() == N,
            "vector of values has wrong length: expected {} got {}",
            N,
            value.len()
        );
        for (b, v) in self.0.iter().zip(value) {
            b.set(witness, v);
        }
    }
}
