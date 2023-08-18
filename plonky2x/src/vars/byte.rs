use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{BoolVariable, CircuitVariable};
use crate::builder::CircuitBuilder;
use crate::ops::{BitAnd, BitOr, BitXor, Not, RotateLeft, RotateRight, Shl, Shr};

/// A variable in the circuit representing a byte value. Under the hood, it is represented as
/// eight bits stored in big endian.
#[derive(Debug, Clone, Copy)]
pub struct ByteVariable(pub [BoolVariable; 8]);

impl CircuitVariable for ByteVariable {
    type ValueType<F> = u8;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => BoolVariable::init(builder); 8])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        let value_be_bit = |i| ((1 << (7 - i) as u8) & value) != 0;
        Self(array![i => BoolVariable::constant(builder, value_be_bit(i)); 8])
    }

    fn targets(&self) -> Vec<Target> {
        self.0.into_iter().flat_map(|x| x.targets()).collect()
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let mut acc: u64 = 0;
        for i in 0..8 {
            let term = (1 << (7 - i)) * (BoolVariable::value(&self.0[i], witness) as u64);
            acc += term;
        }
        acc as u8
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        let value_be_bits = (0..8)
            .map(|i| ((1 << (7 - i)) & value) != 0)
            .collect::<Vec<_>>();
        for i in 0..8 {
            BoolVariable::set(&self.0[i], witness, value_be_bits[i]);
        }
    }
}

impl ByteVariable {
    pub fn to_be_bits(self) -> [BoolVariable; 8] {
        self.0
    }

    pub fn to_le_bits(self) -> [BoolVariable; 8] {
        let mut bits = self.to_be_bits();
        bits.reverse();
        bits
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Not<F, D> for ByteVariable {
    type Output = Self;

    fn not(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        ByteVariable(self.to_be_bits().map(|x| builder.not(x)))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> BitAnd<F, D> for ByteVariable {
    type Output = Self;

    fn bitand(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let rhs_bits = rhs.to_be_bits();
        let mut and_bit = |i| builder.and(self_bits[i], rhs_bits[i]);
        ByteVariable(array![i => and_bit(i); 8])
    }
}

impl<F: RichField + Extendable<D>, const D: usize> BitOr<F, D> for ByteVariable {
    type Output = Self;

    fn bitor(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let rhs_bits = rhs.to_be_bits();
        let mut or_bit = |i| builder.or(self_bits[i], rhs_bits[i]);
        ByteVariable(array![i => or_bit(i); 8])
    }
}

impl<F: RichField + Extendable<D>, const D: usize> BitXor<F, D> for ByteVariable {
    type Output = Self;

    fn bitxor(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let rhs_bits = rhs.to_be_bits();
        let mut xor_bit = |i| builder.xor(self_bits[i], rhs_bits[i]);
        ByteVariable(array![i => xor_bit(i); 8])
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Shl<F, D, usize> for ByteVariable {
    type Output = Self;

    fn shl(self, rhs: usize, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let mut shl_bit = |i| {
            if i + rhs > 7 {
                builder.constant(false)
            } else {
                self_bits[i + rhs]
            }
        };
        ByteVariable(array![i => shl_bit(i); 8])
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Shr<F, D, usize> for ByteVariable {
    type Output = Self;

    fn shr(self, rhs: usize, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let mut shr_bit = |i| {
            if i < rhs {
                builder.constant(false)
            } else {
                self_bits[i - rhs]
            }
        };
        ByteVariable(array![i => shr_bit(i); 8])
    }
}

impl<F: RichField + Extendable<D>, const D: usize> RotateLeft<F, D, usize> for ByteVariable {
    type Output = Self;

    fn rotate_left(self, rhs: usize, _builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let rot_bit = |i| self_bits[(i + rhs) % 8];
        ByteVariable(array![i => rot_bit(i); 8])
    }
}

impl<F: RichField + Extendable<D>, const D: usize> RotateRight<F, D, usize> for ByteVariable {
    type Output = Self;

    fn rotate_right(self, rhs: usize, _builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self.to_be_bits();
        let rot_bit = |i| self_bits[(i + 8 - rhs) % 8];
        ByteVariable(array![i => rot_bit(i); 8])
    }
}
