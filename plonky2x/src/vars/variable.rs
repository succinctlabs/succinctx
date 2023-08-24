use std::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, FieldSerializable};
use crate::builder::CircuitBuilder;
use crate::ops::{Add, Div, Mul, Neg, One, Sub, Zero};

/// A wrapper struct around F to deal with a bug regarding multiple implementations of the same
/// trait. You can cast from F -> Value<F> with `.into()` and cast out with `.0`.
#[derive(Debug, Clone, Copy)]
pub struct Value<F: RichField>(pub F);

impl<F: RichField> From<F> for Value<F> {
    fn from(value: F) -> Self {
        Self(value)
    }
}

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Variable(pub Target);

impl CircuitVariable for Variable {
    type ValueType<F: RichField> = Value<F>;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        let target = builder.api.add_virtual_target();
        Self(target)
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        let target = builder.api.constant(value.0);
        Self(target)
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0]
    }

    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), 1);
        Self(targets[0])
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        Value(witness.get_target(self.0))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        witness.set_target(self.0, value.0);
    }
}

impl<F: RichField> FieldSerializable<F> for Value<F> {
    fn nb_elements() -> usize {
        1
    }

    fn elements(&self) -> Vec<F> {
        vec![self.0]
    }

    fn from_elements(elements: &[F]) -> Self {
        assert_eq!(elements.len(), 1);
        Self(elements[0])
    }
}

impl From<Target> for Variable {
    fn from(target: Target) -> Self {
        Self(target)
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Add<F, D> for Variable {
    type Output = Variable;
    fn add(self, rhs: Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Variable(builder.api.add(self.0, rhs.0))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Sub<F, D> for Variable {
    type Output = Variable;
    fn sub(self, rhs: Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Variable(builder.api.sub(self.0, rhs.0))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Mul<F, D> for Variable {
    type Output = Variable;
    fn mul(self, rhs: Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Variable(builder.api.mul(self.0, rhs.0))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Neg<F, D> for Variable {
    type Output = Variable;
    fn neg(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Variable(builder.api.neg(self.0))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Div<F, D> for Variable {
    type Output = Variable;
    fn div(self, rhs: Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Variable(builder.api.div(self.0, rhs.0))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Zero<F, D> for Variable {
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self {
        Variable(builder.api.zero())
    }
}

impl<F: RichField + Extendable<D>, const D: usize> One<F, D> for Variable {
    fn one(builder: &mut CircuitBuilder<F, D>) -> Self {
        Variable(builder.api.one())
    }
}
