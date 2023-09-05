//! Arithmetic operations.

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;
use crate::vars::BoolVariable;
/// The equality operatoin.
///
/// Types implementing this trait can be used with the `builder.eq(lhs, rhs)` method.
pub trait PartialEq<F: RichField + Extendable<D>, const D: usize, Rhs = Self> {
    fn eq(self, rhs: Rhs, builder: &mut CircuitBuilder<F, D>) -> BoolVariable;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn eq<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> BoolVariable
    where
        Lhs: PartialEq<F, D, Rhs>,
    {
        lhs.eq(rhs, self)
    }

    pub fn assert_eq<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs)
    where
        Lhs: PartialEq<F, D, Rhs>,
    {
        let is_eq = self.eq(lhs, rhs);
        self.api.assert_one(is_eq.0.0);
    }
}

// https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html