use core::borrow::Borrow;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::operations::{AddCarry, SumCarry};
use crate::builder::CircuitBuilder;
use crate::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::vars::{BoolVariable, CircuitVariable, Variable};

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U32Variable(pub Variable);

impl CircuitVariable for U32Variable {
    type ValueType<F> = u32;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(Variable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(Variable::constant(builder, F::from_canonical_u32(value)))
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0 .0]
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let v = witness.get_target(self.0 .0);
        v.to_canonical_u64() as u32
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        witness.set_target(self.0 .0, F::from_canonical_u32(value));
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SumCarry<F, D> for U32Variable {
    fn carrying_sum<I: IntoIterator>(addends: I, builder: &mut CircuitBuilder<F, D>) -> (Self, Self)
    where
        I::Item: core::borrow::Borrow<Self>,
    {
        let addends = addends
            .into_iter()
            .map(|a| U32Target(*a.borrow().targets().first().unwrap()))
            .collect::<Vec<_>>();

        let (result, carry) = builder.api.add_many_u32(&addends);

        (Self(result.0.into()), Self(carry.0.into()))
    }
}
