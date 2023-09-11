//! Circuit builder interfaces for bitwise operations.

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

/// The bitwise AND operation.
///
/// Types implementing this trait can be used within the `builder.and(lhs, rhs)` method.
pub trait BitAnd<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn bitand(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn and<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as BitAnd<L, D, Rhs>>::Output
    where
        Lhs: BitAnd<L, D, Rhs>,
    {
        lhs.bitand(rhs, self)
    }
}

/// The bitwise OR operation.
///
/// Types implementing this trait can be used within the `builder.or(lhs, rhs)` method.
pub trait BitOr<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn bitor(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn or<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as BitOr<L, D, Rhs>>::Output
    where
        Lhs: BitOr<L, D, Rhs>,
    {
        lhs.bitor(rhs, self)
    }
}

/// The bitwise XOR operation.
///
/// Types implementing this trait can be used within the `builder.xor(lhs, rhs)` method.
pub trait BitXor<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn bitxor(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn xor<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as BitXor<L, D, Rhs>>::Output
    where
        Lhs: BitXor<L, D, Rhs>,
    {
        lhs.bitxor(rhs, self)
    }
}

/// The bitwise NOT operation.
///
/// Types implementing this trait can be used within the `builder.not(variable)` method.
pub trait Not<L: PlonkParameters<D>, const D: usize> {
    type Output;

    fn not(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn not<T>(&mut self, variable: T) -> <T as Not<L, D>>::Output
    where
        T: Not<L, D>,
    {
        variable.not(self)
    }
}

/// The left shift operation.
///
/// Types implementing this trait can be used within the `builder.shl(lhs, rhs)` method.
pub trait Shl<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn shl(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn shl<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Shl<L, D, Rhs>>::Output
    where
        Lhs: Shl<L, D, Rhs>,
    {
        lhs.shl(rhs, self)
    }
}

/// The right shift operation.
///
/// Types implementing this trait can be used within the `builder.shr(lhs, rhs)` method.
pub trait Shr<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn shr(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn shr<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Shr<L, D, Rhs>>::Output
    where
        Lhs: Shr<L, D, Rhs>,
    {
        lhs.shr(rhs, self)
    }
}

/// The rotate left operation.
///
/// Types implementing this trait can be used within the `builder.rotate_left(lhs, rhs)` method.
pub trait RotateLeft<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn rotate_left(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn rotate_left<Lhs, Rhs>(
        &mut self,
        lhs: Lhs,
        rhs: Rhs,
    ) -> <Lhs as RotateLeft<L, D, Rhs>>::Output
    where
        Lhs: RotateLeft<L, D, Rhs>,
    {
        lhs.rotate_left(rhs, self)
    }
}

/// The rotate right operation.
///
/// Types implementing this trait can be used within the `builder.rotate_right(lhs, rhs)` method.
pub trait RotateRight<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn rotate_right(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn rotate_right<Lhs, Rhs>(
        &mut self,
        lhs: Lhs,
        rhs: Rhs,
    ) -> <Lhs as RotateRight<L, D, Rhs>>::Output
    where
        Lhs: RotateRight<L, D, Rhs>,
    {
        lhs.rotate_right(rhs, self)
    }
}
