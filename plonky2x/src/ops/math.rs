//! Arithmetic operations

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;

/// The addition operation
///
/// Computes lhs + rhs.
/// This operation is invoked by builder.add(lhs, rhs) and returns a result
pub trait Add<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
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

/// The subtraction operation
///
/// Computes lhs - rhs.
/// This operation is invoked by builder.sub(lhs, rhs) and returns a result
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

/// The multiplication operation
///
/// Computes lhs * rhs.
/// This operation is invoked by builder.mul(lhs, rhs) and returns a result
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

/// The negation operation
///
/// Computes -self.
/// This operation is invoked by builder.neg(self) and returns a result
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

/// The division operation
///
/// Computes lhs / rhs.
/// This operation is invoked by builder.div(lhs, rhs) and returns a result
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
pub trait Zero<F: RichField + Extendable<D>, const D: usize> {
    type Output;

    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn zero<T>(&mut self) -> <T as Zero<F, D>>::Output
    where
        T: Zero<F, D>,
    {
        T::zero(self)
    }
}

/// A One element
pub trait One<F: RichField + Extendable<D>, const D: usize> {
    type Output;

    fn one(builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn one<T>(&mut self) -> <T as One<F, D>>::Output
    where
        T: One<F, D>,
    {
        T::one(self)
    }
}
