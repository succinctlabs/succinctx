use core::fmt::Debug;

pub mod uint128;
pub mod uint256;
pub mod uint32;
pub mod uint64;

mod uint32_n;

pub trait Uint<const N: usize>: Debug + Clone + Copy + Sync + Send + 'static {
    fn to_little_endian(&self, bytes: &mut [u8]);

    fn from_little_endian(slice: &[u8]) -> Self;

    fn to_big_endian(&self, bytes: &mut [u8]);

    fn from_big_endian(slice: &[u8]) -> Self;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    fn to_u32_limbs(self) -> [u32; N] {
        let mut bytes = vec![0u8; N * 4];
        self.to_little_endian(&mut bytes);
        let mut ret: [u32; N] = [0; N];
        for i in 0..N {
            let byte_offset = i * 4;
            ret[i] = u32::from_le_bytes([
                bytes[byte_offset],
                bytes[byte_offset + 1],
                bytes[byte_offset + 2],
                bytes[byte_offset + 3],
            ])
        }
        ret
    }

    fn from_u32_limbs(limbs: [u32; N]) -> Self {
        let bytes = limbs
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<_>>();
        Self::from_little_endian(&bytes)
    }
}
