//! Arithmetic operations.

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::frontend::builder::CircuitBuilder;

/// The addition operation.
///
/// Types implementing this trait can be used within the `builder.add(lhs, rhs)` method.
pub trait Add<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    /// The output type of the operation.
    type Output;

    fn add(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn add<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Add<F, D, Rhs>>::Output
    where
        Lhs: Add<F, D, Rhs>,
    {
        lhs.add(rhs, self)
    }
}

/// The subtraction operation.
///
/// Types implementing this trait can be used within the `builder.sub(lhs, rhs)` method.
pub trait Sub<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn sub(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn sub<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Sub<F, D, Rhs>>::Output
    where
        Lhs: Sub<F, D, Rhs>,
    {
        lhs.sub(rhs, self)
    }
}

/// The multiplication operation.
///
/// Types implementing this trait can be used within the `builder.mul(lhs, rhs)` method.
pub trait Mul<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn mul(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn mul<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Mul<F, D, Rhs>>::Output
    where
        Lhs: Mul<F, D, Rhs>,
    {
        lhs.mul(rhs, self)
    }
}

/// The negation operation.
///
/// Types implementing this trait can be used within the `builder.neg(value)` method.
pub trait Neg<F: RichField + Extendable<D>, const D: usize> {
    type Output;

    fn neg(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn neg<T>(&mut self, value: T) -> <T as Neg<F, D>>::Output
    where
        T: Neg<F, D>,
    {
        value.neg(self)
    }
}

/// The division operation.
///
/// Types implementing this trait can be used within the `builder.div(lhs, rhs)` method.
pub trait Div<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    type Output;

    fn div(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn div<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Div<F, D, Rhs>>::Output
    where
        Lhs: Div<F, D, Rhs>,
    {
        lhs.div(rhs, self)
    }
}

/// A zero element
///
/// Types implementing this trait can be used via the `builder.zero()` method.
pub trait Zero<F: RichField + Extendable<D>, const D: usize> {
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn zero<T: Zero<F, D>>(&mut self) -> T {
        T::zero(self)
    }
}

/// A One element
///
/// Types implementing this trait can be used via the `builder.one()` method.
pub trait One<F: RichField + Extendable<D>, const D: usize> {
    fn one(builder: &mut CircuitBuilder<F, D>) -> Self;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn one<T: One<F, D>>(&mut self) -> T {
        T::one(self)
    }
}
