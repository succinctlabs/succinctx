use core::borrow::Borrow;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;
use crate::vars::BoolVariable;

pub trait AddCarry<F: RichField + Extendable<D>, const D: usize>: Sized {
    fn carrying_add(
        self,
        rhs: Self,
        carry: BoolVariable,
        builder: &mut CircuitBuilder<F, D>,
    ) -> (Self, BoolVariable);
}

pub trait MulCarry<F: RichField + Extendable<D>, const D: usize>: Sized {
    fn carrying_mul(
        self,
        rhs: Self,
        carry: Self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> (Self, Self);
}

pub trait SumCarry<F: RichField + Extendable<D>, const D: usize>: Sized {
    fn carrying_sum<I: IntoIterator>(
        addends: I,
        builder: &mut CircuitBuilder<F, D>,
    ) -> (Self, Self)
    where
        I::Item: Borrow<Self>;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn carrying_add<T: AddCarry<F, D>>(
        &mut self,
        lhs: T,
        rhs: T,
        carry: BoolVariable,
    ) -> (T, BoolVariable) {
        lhs.carrying_add(rhs, carry, self)
    }

    pub fn carrying_mul<T: MulCarry<F, D>>(&mut self, lhs: T, rhs: T, carry: T) -> (T, T) {
        lhs.carrying_mul(rhs, carry, self)
    }

    pub fn carrying_sum<T: SumCarry<F, D>, I: IntoIterator>(&mut self, addends: I) -> (T, T)
    where
        I::Item: Borrow<T>,
    {
        T::carrying_sum(addends, self)
    }
}
