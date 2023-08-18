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