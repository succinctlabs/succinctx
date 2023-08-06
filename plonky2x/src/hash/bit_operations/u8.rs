use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder;

#[derive(Debug, Clone, Copy)]
pub struct U8Target(pub Target);

pub trait U8Builder {
    /// Creates a zero byte.
    fn zero_u8(&mut self) -> U8Target;

    /// Creates a new byte from an existing target.
    fn new_u8(&mut self, target: Target) -> U8Target;

    /// Splits a single byte into bits in little-endian order.
    fn u8_to_bits_le(&mut self, byte: U8Target) -> [BoolTarget; 8];
}

impl<F: RichField + Extendable<D>, const D: usize> U8Builder for CircuitBuilder<F, D> {
    fn zero_u8(&mut self) -> U8Target {
        U8Target(self.zero())
    }

    fn new_u8(&mut self, target: Target) -> U8Target {
        U8Target(target)
    }

    fn u8_to_bits_le(&mut self, byte: U8Target) -> [BoolTarget; 8] {
        // Note: The gate being used under the hood here is probably unoptimized for this usecase.
        // In particular, we can "batch decompose" the bits to fill the entire width of the table.
        let mut res = [self._false(); 8];
        let bits = self.split_le(byte.0, 8);
        res.copy_from_slice(&bits[..8]);
        res
    }
}
