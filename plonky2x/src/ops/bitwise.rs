//! Circuit builder interfaces for bitwise operations.

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;

/// The bitwise AND operation.
///
/// Types implementing this trait can be used within the `builder.and(lhs, rhs)` method.
pub trait BitAnd<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn bitand(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn and<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as BitAnd<F, D, Rhs>>::Output
    where
        Lhs: BitAnd<F, D, Rhs>,
    {
        lhs.bitand(rhs, self)
    }
}

/// The bitwise OR operation.
///
/// Types implementing this trait can be used within the `builder.or(lhs, rhs)` method.
pub trait BitOr<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn bitor(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn or<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as BitOr<F, D, Rhs>>::Output
    where
        Lhs: BitOr<F, D, Rhs>,
    {
        lhs.bitor(rhs, self)
    }
}

/// The bitwise XOR operation.
///
/// Types implementing this trait can be used within the `builder.xor(lhs, rhs)` method.
pub trait BitXor<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn bitxor(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn xor<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as BitXor<F, D, Rhs>>::Output
    where
        Lhs: BitXor<F, D, Rhs>,
    {
        lhs.bitxor(rhs, self)
    }
}

/// The bitwise NOT operation.
///
/// Types implementing this trait can be used within the `builder.not(variable)` method.
pub trait Not<F: RichField + Extendable<D>, const D: usize> {
    type Output;

    fn not(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn not<T>(&mut self, variable: T) -> <T as Not<F, D>>::Output
    where
        T: Not<F, D>,
    {
        variable.not(self)
    }
}

/// The left shift operation.
///
/// Types implementing this trait can be used within the `builder.shl(lhs, rhs)` method.
pub trait Shl<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn shl(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn shl<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Shl<F, D, Rhs>>::Output
    where
        Lhs: Shl<F, D, Rhs>,
    {
        lhs.shl(rhs, self)
    }
}

/// The right shift operation.
///
/// Types implementing this trait can be used within the `builder.shr(lhs, rhs)` method.
pub trait Shr<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn shr(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn shr<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Shr<F, D, Rhs>>::Output
    where
        Lhs: Shr<F, D, Rhs>,
    {
        lhs.shr(rhs, self)
    }
}

/// The rotate left operation.
///
/// Types implementing this trait can be used within the `builder.rotate_left(lhs, rhs)` method.
pub trait RotateLeft<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn rotate_left(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn rotate_left<Lhs, Rhs>(
        &mut self,
        lhs: Lhs,
        rhs: Rhs,
    ) -> <Lhs as RotateLeft<F, D, Rhs>>::Output
    where
        Lhs: RotateLeft<F, D, Rhs>,
    {
        lhs.rotate_left(rhs, self)
    }
}

/// The rotate right operation.
///
/// Types implementing this trait can be used within the `builder.rotate_right(lhs, rhs)` method.
pub trait RotateRight<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn rotate_right(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn rotate_right<Lhs, Rhs>(
        &mut self,
        lhs: Lhs,
        rhs: Rhs,
    ) -> <Lhs as RotateRight<F, D, Rhs>>::Output
    where
        Lhs: RotateRight<F, D, Rhs>,
    {
        lhs.rotate_right(rhs, self)
    }
}
