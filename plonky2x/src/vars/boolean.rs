use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::builder::CircuitBuilder;
use crate::ops::{BitAnd, BitOr, BitXor, Not};

/// A variable in the circuit representing a boolean value.
#[derive(Debug, Clone, Copy)]
pub struct BoolVariable(pub Variable);

impl CircuitVariable for BoolVariable {
    type ValueType<F> = bool;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(Variable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(Variable::constant(
            builder,
            F::from_canonical_u8(value as u8),
        ))
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0 .0]
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
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

impl<F: RichField + Extendable<D>, const D: usize> BitAnd<F, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitand(self, rhs: BoolVariable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        builder.mul(self.0, rhs.0).into()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> BitOr<F, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitor(self, rhs: BoolVariable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_plus_rhs = builder.add(self.0, rhs.0);
        let self_times_rhs = builder.mul(self.0, rhs.0);
        builder.sub(self_plus_rhs, self_times_rhs).into()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> BitXor<F, D> for BoolVariable {
    type Output = BoolVariable;

    fn bitxor(self, rhs: BoolVariable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let a_plus_b = builder.add(self.0, rhs.0);
        let a_b = builder.mul(self.0, rhs.0);
        let two_a_b = builder.add(a_b, a_b);
        builder.sub(a_plus_b, two_a_b).into()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Not<F, D> for BoolVariable {
    type Output = BoolVariable;

    fn not(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let one = builder.one::<Variable>();
        builder.sub(one, self.0).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_bit_ops() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

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

        let mut pw = PartialWitness::new();

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

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}
