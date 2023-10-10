use std::fmt::Debug;

use itertools::Itertools;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use serde::{Deserialize, Serialize};

use super::{BoolVariable, ByteVariable, CircuitVariable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::ops::{Add, Div, Mul, Neg, One, Sub, Zero};

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable(pub Target);

impl CircuitVariable for Variable {
    type ValueType<F: RichField> = F;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        let target = builder.api.add_virtual_target();
        builder.debug_target(target);
        Self(target)
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        let target = builder.api.constant(value);
        builder.debug_target(target);
        Self(target)
    }

    fn variables(&self) -> Vec<Variable> {
        vec![*self]
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 1);
        variables[0]
    }

    #[allow(unused_variables)]
    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
    }

    fn nb_elements() -> usize {
        1
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        vec![value]
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(elements.len(), 1);
        elements[0]
    }
}

impl Variable {
    pub fn to_bits<const N: usize, L: PlonkParameters<D>, const D: usize>(
        self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> [BoolVariable; N] {
        builder
            .api
            .split_le(self.0, N)
            .iter()
            .rev()
            .map(|x| BoolVariable::from(x.target))
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn to_byte_variable<L: PlonkParameters<D>, const D: usize>(
        self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> ByteVariable {
        let bits: [BoolVariable; 8] = self.to_bits(builder);
        ByteVariable(bits)
    }
}

impl From<Target> for Variable {
    fn from(target: Target) -> Self {
        Self(target)
    }
}

impl<L: PlonkParameters<D>, const D: usize> Add<L, D> for Variable {
    type Output = Variable;
    fn add(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        Variable(builder.api.add(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Sub<L, D> for Variable {
    type Output = Variable;
    fn sub(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        Variable(builder.api.sub(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Mul<L, D> for Variable {
    type Output = Variable;
    fn mul(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        Variable(builder.api.mul(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Neg<L, D> for Variable {
    type Output = Variable;
    fn neg(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        Variable(builder.api.neg(self.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Div<L, D> for Variable {
    type Output = Variable;
    fn div(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        Variable(builder.api.div(self.0, rhs.0))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Zero<L, D> for Variable {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
        Variable(builder.api.zero())
    }
}

impl<L: PlonkParameters<D>, const D: usize> One<L, D> for Variable {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
        Variable(builder.api.one())
    }
}
