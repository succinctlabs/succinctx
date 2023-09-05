use std::fmt::Debug;

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{BoolVariable, CircuitVariable, EvmVariable, Variable};
use crate::builder::CircuitBuilder;
use crate::eth::mpt::generators::math::ByteSubGenerator;
use crate::ops::{
    BitAnd, BitOr, BitXor, Not, PartialEq, RotateLeft, RotateRight, Shl, Shr, Sub, Zero,
};
/// A variable in the circuit representing a byte value. Under the hood, it is represented as
/// eight bits stored in big endian.
#[derive(Debug, Clone, Copy)]
pub struct ByteVariable(pub [BoolVariable; 8]);

impl CircuitVariable for ByteVariable {
    type ValueType<F: RichField> = u8;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => BoolVariable::init(builder); 8])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(array![i => BoolVariable::constant(builder, (value >> (7 - i)) & 1 != 0); 8])
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.iter().map(|x| x.0).collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 8);
        Self(array![i => BoolVariable(variables[i]); 8])
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let mut acc: u64 = 0;
        for i in 0..8 {
            let term = (1 << (7 - i)) * (BoolVariable::get(&self.0[i], witness) as u64);
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

impl EvmVariable for ByteVariable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        _: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        vec![*self]
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        _: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        assert_eq!(bytes.len(), 1);
        bytes[0]
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        vec![value]
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        assert_eq!(bytes.len(), 1);
        bytes[0]
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

impl<F: RichField + Extendable<D>, const D: usize> PartialEq<F, D> for ByteVariable {
    fn eq(self, rhs: ByteVariable, builder: &mut CircuitBuilder<F, D>) -> BoolVariable {
        let mut result = builder.constant::<BoolVariable>(true);
        for i in 0..8 {
            let lhs_byte = self.0[i];
            let rhs_byte = rhs.0[i];
            let byte_eq = builder.eq(lhs_byte, rhs_byte);
            result = builder.and(result, byte_eq);
        }
        result
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Sub<F, D> for ByteVariable {
    type Output = Self;

    fn sub(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let output = builder.init::<ByteVariable>();
        let generator = ByteSubGenerator {
            lhs: self,
            rhs,
            output,
            _phantom: std::marker::PhantomData::<F>,
        };
        builder.add_simple_generator(&generator);
        output
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

impl<F: RichField + Extendable<D>, const D: usize> Zero<F, D> for ByteVariable {
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self {
        ByteVariable(array![_ => builder.constant(false); 8])
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::prelude::*;

    #[test]
    fn test_byte_operations() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let x_bytes = (0..256)
            .map(|_| builder.init::<ByteVariable>())
            .collect::<Vec<_>>();
        let y_bytes = (0..256)
            .map(|_| builder.init::<ByteVariable>())
            .collect::<Vec<_>>();

        let ((x_and_y_bytes, x_or_y_bytes), x_xor_y_bytes): ((Vec<_>, Vec<_>), Vec<_>) = x_bytes
            .iter()
            .cartesian_product(y_bytes.iter())
            .map(|(&x, &y)| ((builder.and(x, y), builder.or(x, y)), builder.xor(x, y)))
            .unzip();

        let (((x_shr_bytes, x_shl_bytes), x_rot_right_bytes), x_rot_left_bytes): (
            ((Vec<_>, Vec<_>), Vec<_>),
            Vec<_>,
        ) = x_bytes
            .iter()
            .cartesian_product(0..8)
            .map(|(&x, i)| {
                (
                    (
                        (builder.shr(x, i), builder.shl(x, i)),
                        builder.rotate_right(x, i),
                    ),
                    builder.rotate_left(x, i),
                )
            })
            .unzip();

        let circuit = builder.build::<C>();
        let mut pw = PartialWitness::new();

        let x_values = (0..256).map(|i| i as u8).collect::<Vec<_>>();
        let y_values = (0..256).map(|i| (i + 1) as u8).collect::<Vec<_>>();

        for (x, val) in x_bytes.iter().zip(x_values.iter()) {
            x.set(&mut pw, *val);
        }

        for (y, val) in y_bytes.iter().zip(y_values.iter()) {
            y.set(&mut pw, *val);
        }

        for ((((x, y), x_and_y), x_or_y), x_xor_y) in x_values
            .iter()
            .cartesian_product(y_values.iter())
            .zip(x_and_y_bytes)
            .zip(x_or_y_bytes)
            .zip(x_xor_y_bytes)
        {
            x_and_y.set(&mut pw, x & y);
            x_or_y.set(&mut pw, x | y);
            x_xor_y.set(&mut pw, x ^ y);
        }

        for (((((x, i), x_shr), x_shl), x_rot_right), x_rot_left) in x_values
            .iter()
            .cartesian_product(0..8)
            .zip(x_shr_bytes)
            .zip(x_shl_bytes)
            .zip(x_rot_right_bytes)
            .zip(x_rot_left_bytes)
        {
            x_shr.set(&mut pw, x >> i);
            x_shl.set(&mut pw, x << i);
            x_rot_right.set(&mut pw, x.rotate_right(i));
            x_rot_left.set(&mut pw, x.rotate_left(i));
        }

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
