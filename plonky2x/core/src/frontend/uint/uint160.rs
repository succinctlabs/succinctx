use array_macro::array;
use plonky2::hash::hash_types::RichField;

use super::Uint;
use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{EvmVariable, SSZVariable, U32Variable};
use crate::prelude::{
    Add, BoolVariable, ByteVariable, Bytes32Variable, BytesVariable, CircuitBuilder,
    CircuitVariable, Div, LessThanOrEqual, Mul, One, PlonkParameters, Rem, Sub, Variable, Zero,
};
use crate::{make_uint32_n, make_uint32_n_tests};

#[derive(Copy, Clone, Debug)]
pub struct U160([u32; 5]);

impl U160 {
    pub fn from_u32_limbs(limbs: [u32; 5]) -> Self {
        Self(limbs)
    }
}

impl Uint<5> for U160 {
    fn to_little_endian(&self, bytes: &mut [u8]) {
        self.0
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .zip(bytes)
            .for_each(|(a, b)| *b = a);
    }

    fn from_little_endian(slice: &[u8]) -> Self {
        let mut limbs = [0; 5];
        for i in 0..5 {
            let mut limb = [0; 4];
            limb.copy_from_slice(&slice[i * 4..(i + 1) * 4]);
            limbs[i] = u32::from_le_bytes(limb);
        }
        Self(limbs)
    }

    fn to_big_endian(&self, bytes: &mut [u8]) {
        self.0
            .iter()
            .rev()
            .flat_map(|x| x.to_be_bytes())
            .zip(bytes)
            .for_each(|(a, b)| *b = a);
    }

    fn from_big_endian(slice: &[u8]) -> Self {
        let mut limbs = [0; 5];
        for i in 0..5 {
            let mut limb = [0; 4];
            limb.copy_from_slice(&slice[i * 4..(i + 1) * 4]);
            limbs[4 - i] = u32::from_be_bytes(limb);
        }
        Self(limbs)
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut carry = 0;
        let mut result = [0; 5];
        for i in 0..5 {
            let (sum, overflow1) = self.0[i].overflowing_add(rhs.0[i]);
            let (sum, overflow2) = sum.overflowing_add(carry);
            let overflow = overflow1 || overflow2;

            result[i] = sum;
            carry = if overflow { 1 } else { 0 };
        }
        (Self(result), carry == 1)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut carry = 0;
        let mut result = [0; 5];
        for i in 0..5 {
            let (diff, overflow1) = self.0[i].overflowing_sub(rhs.0[i]);
            let (diff, overflow2) = diff.overflowing_sub(carry);
            let overflow = overflow1 || overflow2;

            result[i] = diff;
            carry = if overflow { 1 } else { 0 };
        }
        (Self(result), carry == 1)
    }

    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let a = self.0;
        let b = rhs.0;
        let mut result = [0u32; 10];
        for i in 0..5 {
            let mut carry = 0u64;
            for j in 0..5 {
                let res = a[i] as u64 * b[j] as u64 + result[i + j] as u64 + carry;
                result[i + j] = res as u32;
                carry = res >> 32;
            }
            result[i + 5] = carry as u32;
        }
        let mut overflow = false;
        for i in 5..10 {
            if result[i] != 0 {
                overflow = true;
            }
        }
        (
            Self([result[0], result[1], result[2], result[3], result[4]]),
            overflow,
        )
    }
}

make_uint32_n!(U160Variable, U160, 5);
make_uint32_n_tests!(U160Variable, U160, 5);
