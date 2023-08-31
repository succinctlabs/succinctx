use ethers::types::U256;

use super::uint32_n::{EthersUint, U32NVariable};

const NUM_LIMBS: usize = 8;

impl EthersUint<NUM_LIMBS> for U256 {
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

pub type U256Variable = U32NVariable<U256, NUM_LIMBS>;
