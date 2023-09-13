use std::fmt::Debug;

use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};
use serde::{Deserialize, Serialize};

use super::CircuitVariable;
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::ops::{Add, Div, LessThan, LessThanOrEqual, Mul, Neg, One, Sub, Zero};
use crate::frontend::vars::BoolVariable;

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable(pub Target);

impl CircuitVariable for Variable {
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

    fn variables(&self) -> Vec<Variable> {
        vec![*self]
    }

    fn from_variables(variables: &[Variable]) -> Self {
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

impl<L: PlonkParameters<D>, const D: usize> LessThan<L, D> for Variable {
    type Output = BoolVariable;

    fn lt(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        // FIXME: Check for overflows and edge cases
        const NUM_LIMBS: usize = 64;
        let max = builder.constant(L::Field::from_canonical_u64(1 << (NUM_LIMBS - 1)));
        let _one: Variable = builder.constant(L::Field::from_canonical_u64(1));
        // from circomlib: https://github.com/iden3/circomlib/blob/master/circuits/comparators.circom#L89
        let tmp = builder.add(self, max);
        let res = builder.sub(tmp, rhs);
        // typically stored in BE, but we can just index instead of reversing
        let res_bits = builder.api.split_le(res.0, NUM_LIMBS);
        let last_bit = Variable::from(res_bits[NUM_LIMBS - 1].target);
        let lt_var = builder.sub(_one, last_bit);
        BoolVariable::from(lt_var)
    }
}

impl<L: PlonkParameters<D>, const D: usize> LessThanOrEqual<L, D> for Variable {
    type Output = BoolVariable;

    fn lte(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let lt = self.lt(rhs, builder);
        let eq = builder.is_equal(self, rhs);
        builder.or(lt, eq)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_lt() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let lhs = builder.read::<Variable>();
        let rhs = builder.read::<Variable>();
        let less_than = builder.lt(lhs, rhs);
        builder.write(less_than);
    }
}
