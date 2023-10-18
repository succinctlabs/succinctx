use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use serde::{Deserialize, Serialize};

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::ops::{BitAnd, BitOr, BitXor, Not};

/// A variable in the circuit representing a boolean value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(clippy::manual_non_exhaustive)]
pub struct BoolVariable {
    pub variable: Variable,
    /// This private field is here to force all instantiations to go the methods below
    _private: (),
}

impl CircuitVariable for BoolVariable {
    type ValueType<F: RichField> = bool;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            variable: Variable::init_unsafe(builder),
            _private: (),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        vec![self.variable]
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 1);
        Self {
            variable: variables[0],
            _private: (),
        }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        builder
            .api
            .assert_bool(BoolTarget::new_unsafe(self.targets()[0]))
    }

    fn nb_elements() -> usize {
        1
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        vec![F::from_canonical_u64(value as u64)]
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(elements.len(), 1);
        elements[0] == F::ONE
    }
}

impl From<BoolTarget> for BoolVariable {
    fn from(v: BoolTarget) -> Self {
        // BoolTarget's range is the same as BoolVariable's.
        Self::from_variables_unsafe(&[Variable(v.target)])
    }
}

impl<L: PlonkParameters<D>, const D: usize> BitAnd<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitand(self, rhs: BoolVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        // Both "self" and "rhs" have a range of [0-1] inclusive.  The result of the "AND" operation
        // will also be in a range of [0-1] inclusive.
        Self::from_variables_unsafe(&[builder.mul(self.variable, rhs.variable)])
    }
}

impl<L: PlonkParameters<D>, const D: usize> BitOr<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitor(self, rhs: BoolVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_plus_rhs = builder.add(self.variable, rhs.variable);
        let self_times_rhs = builder.mul(self.variable, rhs.variable);

        // Both "self" and "rhs" have a range of [0-1] inclusive.  The result of the "OR" operation
        // will also be in a range of [0-1] inclusive.
        Self::from_variables_unsafe(&[builder.sub(self_plus_rhs, self_times_rhs)])
    }
}

impl<L: PlonkParameters<D>, const D: usize> BitXor<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitxor(self, rhs: BoolVariable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let a_plus_b = builder.add(self.variable, rhs.variable);
        let a_b = builder.mul(self.variable, rhs.variable);
        let two_a_b = builder.add(a_b, a_b);

        // Both "self" and "rhs" have a range of [0-1] inclusive.  The result of the "XOR" operation
        // will also be in a range of [0-1] inclusive.
        Self::from_variables_unsafe(&[builder.sub(a_plus_b, two_a_b)])
    }
}

impl<L: PlonkParameters<D>, const D: usize> Not<L, D> for BoolVariable {
    type Output = BoolVariable;

    fn not(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let one = builder.one::<Variable>();

        // "self" has a range of [0-1] inclusive.  The result of the "NOT" operation
        // will also be in a range of [0-1] inclusive.
        Self::from_variables_unsafe(&[builder.sub(one, self.variable)])
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
