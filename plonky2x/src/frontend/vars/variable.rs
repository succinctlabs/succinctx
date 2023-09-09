use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::Variable;
use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::ops::{Add, Div, Mul, Neg, One, Sub, Zero};

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldVariable(pub Target);

impl Variable for FieldVariable {
    type ValueType<F: RichField> = F;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        let target = builder.api.add_virtual_target();
        builder.debug_target(target);
        Self(target)
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        // In the special case that we are creating a variable constant, we record it in the builder
        // so that we can use it to implement serialization/deserialize to/from elements for
        // ValueType automatically.
        let target = builder.api.constant(value);
        builder.debug_target(target); // TODO: not sure if I need this
        let variable = Self(target);
        builder.constants.insert(variable, value);
        Self(target)
    }

    fn variables(&self) -> Vec<FieldVariable> {
        vec![*self]
    }

    fn from_variables(variables: &[FieldVariable]) -> Self {
        assert_eq!(variables.len(), 1);
        variables[0]
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        witness.get_target(self.0)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        witness.set_target(self.0, value);
    }
}

impl From<Target> for FieldVariable {
    fn from(target: Target) -> Self {
        Self(target)
    }
}

impl<L: PlonkParameters<D>, const D: usize> Add<L, D> for FieldVariable {
    type Output = FieldVariable;
    fn add(self, rhs: FieldVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        FieldVariable(builder.api.add(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Sub<L, D> for FieldVariable {
    type Output = FieldVariable;
    fn sub(self, rhs: FieldVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        FieldVariable(builder.api.sub(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Mul<L, D> for FieldVariable {
    type Output = FieldVariable;
    fn mul(self, rhs: FieldVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        FieldVariable(builder.api.mul(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Neg<L, D> for FieldVariable {
    type Output = FieldVariable;
    fn neg(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        FieldVariable(builder.api.neg(self.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Div<L, D> for FieldVariable {
    type Output = FieldVariable;
    fn div(self, rhs: FieldVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        FieldVariable(builder.api.div(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Zero<L, D> for FieldVariable {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
        FieldVariable(builder.api.zero())
    }
}

impl<L: PlonkParameters<D>, const D: usize> One<L, D> for FieldVariable {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
        FieldVariable(builder.api.one())
    }
}
