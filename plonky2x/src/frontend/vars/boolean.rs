use std::fmt::Debug;

use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{Witness, WitnessWrite};
use serde::{Deserialize, Serialize};

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::ops::{BitAnd, BitOr, BitXor, Not};

/// A variable in the circuit representing a boolean value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoolVariable(pub Variable);

impl CircuitVariable for BoolVariable {
    type ValueType<F: RichField> = bool;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self(Variable::init_unsafe(builder))
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self(Variable::constant(
            builder,
            L::Field::from_canonical_u64(value as u64),
        ))
    }

    fn variables(&self) -> Vec<Variable> {
        vec![self.0]
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 1);
        Self(variables[0])
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        builder
            .api
            .assert_bool(BoolTarget::new_unsafe(self.targets()[0]))
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        witness.get_target(self.0 .0) == F::from_canonical_u64(1)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        witness.set_target(self.0 .0, F::from_canonical_u64(value as u64));
    }
}

impl From<Target> for BoolVariable {
    fn from(v: Target) -> Self {
        Self(Variable(v))
    }
}

impl From<Variable> for BoolVariable {
    fn from(v: Variable) -> Self {
        Self(v)
    }
}

impl<L: PlonkParameters<D>, const D: usize> BitAnd<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitand(self, rhs: BoolVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        builder.mul(self.0, rhs.0).into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> BitOr<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitor(self, rhs: BoolVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_plus_rhs = builder.add(self.0, rhs.0);
        let self_times_rhs = builder.mul(self.0, rhs.0);
        builder.sub(self_plus_rhs, self_times_rhs).into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> BitXor<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitxor(self, rhs: BoolVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let a_plus_b = builder.add(self.0, rhs.0);
        let a_b = builder.mul(self.0, rhs.0);
        let two_a_b = builder.add(a_b, a_b);
        builder.sub(a_plus_b, two_a_b).into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Not<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn not(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let one = builder.one::<Variable>();
        builder.sub(one, self.0).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_bit_operations() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let x = builder.init::<BoolVariable>();
        let y = builder.init::<BoolVariable>();

        let not_x = builder.not(x);
        let not_y = builder.not(y);
        let x_and_y = builder.and(x, y);
        let x_and_x = builder.and(x, x);
        let x_or_y = builder.or(x, y);
        let x_or_x = builder.or(x, x);
        let y_or_y = builder.or(y, y);
        let x_xor_y = builder.xor(x, y);
        let x_xor_x = builder.xor(x, x);
        let y_xor_y = builder.xor(y, y);

        let mut pw: PartialWitness<GoldilocksField> = PartialWitness::new();

        x.set(&mut pw, true);
        y.set(&mut pw, false);

        not_x.set(&mut pw, false);
        not_y.set(&mut pw, true);
        x_and_y.set(&mut pw, false);
        x_and_x.set(&mut pw, true);
        x_or_y.set(&mut pw, true);
        x_or_x.set(&mut pw, true);
        y_or_y.set(&mut pw, false);
        x_xor_y.set(&mut pw, true);
        x_xor_x.set(&mut pw, false);
        y_xor_y.set(&mut pw, false);

        let circuit = builder.build();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
