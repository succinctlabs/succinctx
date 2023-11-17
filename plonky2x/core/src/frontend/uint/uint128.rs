use array_macro::array;
use ethers::types::U128;
use plonky2::hash::hash_types::RichField;

use super::Uint;
use crate::frontend::uint::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::uint::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{EvmVariable, SSZVariable, U256Variable, U32Variable};
use crate::prelude::{
    Add, BoolVariable, ByteVariable, Bytes32Variable, BytesVariable, CircuitBuilder,
    CircuitVariable, Div, LessThanOrEqual, Mul, One, PlonkParameters, Rem, Sub, Variable, Zero,
};
use crate::{make_uint32_n, make_uint32_n_tests};

impl Uint<4> for U128 {
    fn to_little_endian(&self, bytes: &mut [u8]) {
        self.to_little_endian(bytes);
    }

    fn from_little_endian(slice: &[u8]) -> Self {
        Self::from_little_endian(slice)
    }

    fn to_big_endian(&self, bytes: &mut [u8]) {
        self.to_big_endian(bytes);
    }

    fn from_big_endian(slice: &[u8]) -> Self {
        Self::from_big_endian(slice)
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }
}

make_uint32_n!(U128Variable, U128, 4);
make_uint32_n_tests!(U128Variable, U128, 4);

impl U128Variable {
    pub fn to_u256<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> U256Variable {
        let zero = builder.zero::<U32Variable>();
        let result = builder.init::<U256Variable>();
        for i in 0..result.limbs.len() {
            if i < self.limbs.len() {
                builder.connect(self.limbs[i], result.limbs[i]);
            } else {
                builder.connect(zero, result.limbs[i]);
            }
        }
        result
    }
}
