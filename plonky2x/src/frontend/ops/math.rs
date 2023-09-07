//! Arithmetic operations.

use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

/// The addition operation.
///
/// Types implementing this trait can be used within the `builder.add(lhs, rhs)` method.
pub trait Add<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    /// The output type of the operation.
    type Output;

    fn add(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn add<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Add<L, D, Rhs>>::Output
    where
        Lhs: Add<L, D, Rhs>,
    {
        lhs.add(rhs, self)
    }
}

/// The subtraction operation.
///
/// Types implementing this trait can be used within the `builder.sub(lhs, rhs)` method.
pub trait Sub<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn sub(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn sub<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Sub<L, D, Rhs>>::Output
    where
        Lhs: Sub<L, D, Rhs>,
    {
        lhs.sub(rhs, self)
    }
}

/// The multiplication operation.
///
/// Types implementing this trait can be used within the `builder.mul(lhs, rhs)` method.
pub trait Mul<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn mul(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn mul<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Mul<L, D, Rhs>>::Output
    where
        Lhs: Mul<L, D, Rhs>,
    {
        lhs.mul(rhs, self)
    }
}

/// The negation operation.
///
/// Types implementing this trait can be used within the `builder.neg(value)` method.
pub trait Neg<L: PlonkParameters<D>, const D: usize> {
    type Output;

    fn neg(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn neg<T>(&mut self, value: T) -> <T as Neg<L, D>>::Output
    where
        T: Neg<L, D>,
    {
        value.neg(self)
    }
}

/// The division operation.
///
/// Types implementing this trait can be used within the `builder.div(lhs, rhs)` method.
pub trait Div<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn div(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn div<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Div<L, D, Rhs>>::Output
    where
        Lhs: Div<L, D, Rhs>,
    {
        lhs.div(rhs, self)
    }
}

/// The remainder operation.
///
/// Types implementing this trait can be used within the `builder.rem(lhs, rhs)` method.
pub trait Rem<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn rem(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn rem<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Rem<L, D, Rhs>>::Output
    where
        Lhs: Rem<L, D, Rhs>,
    {
        lhs.rem(rhs, self)
    }
}

/// A zero element
///
/// Types implementing this trait can be used via the `builder.zero()` method.
pub trait Zero<L: PlonkParameters<D>, const D: usize> {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn zero<T: Zero<L, D>>(&mut self) -> T {
        T::zero(self)
    }
}

/// A One element
///
/// Types implementing this trait can be used via the `builder.one()` method.
pub trait One<L: PlonkParameters<D>, const D: usize> {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn one<T: One<L, D>>(&mut self) -> T {
        T::one(self)
    }
}
