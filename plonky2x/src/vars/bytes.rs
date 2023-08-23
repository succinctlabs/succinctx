use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::ops::{BitAnd, BitOr, BitXor, Not};
use crate::vars::ByteVariable;

/// A variable in the circuit representing a byte value.
#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [ByteVariable; N]);

impl<const N: usize> CircuitVariable for BytesVariable<N> {
    type ValueType<F> = [u8; N];

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => ByteVariable::init(builder); N])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
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

    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), N * 8);
        Self(array![i => ByteVariable::from_targets(&targets[i*8..(i+1)*8]); N])
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.0.map(|b| b.value(witness))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
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

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> Not<F, D> for BytesVariable<N> {
    type Output = Self;

    fn not(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        BytesVariable(self.0.map(|x| builder.not(x)))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> BitAnd<F, D>
    for BytesVariable<N>
{
    type Output = Self;

    fn bitand(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let byte_fn = |i| builder.and(self.0[i], rhs.0[i]);
        BytesVariable(core::array::from_fn(byte_fn))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> BitOr<F, D>
    for BytesVariable<N>
{
    type Output = Self;

    fn bitor(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let byte_fn = |i| builder.or(self.0[i], rhs.0[i]);
        BytesVariable(core::array::from_fn(byte_fn))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> BitXor<F, D>
    for BytesVariable<N>
{
    type Output = Self;

    fn bitxor(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let byte_fn = |i| builder.xor(self.0[i], rhs.0[i]);
        BytesVariable(core::array::from_fn(byte_fn))
    }
}
