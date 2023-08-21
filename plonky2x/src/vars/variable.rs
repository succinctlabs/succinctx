use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::ops::{Add, Div, Mul, Neg, One, Sub, Zero};

/// A variable in the circuit. It represents a field element`.
#[derive(Debug, Clone, Copy)]
pub struct Variable(pub Target);

impl CircuitVariable for Variable {
    type ValueType<F> = F;

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
        let target = builder.api.constant(value);
        Self(target)
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0]
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> F {
        witness.get_target(self.0)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: F) {
        witness.set_target(self.0, value);
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
