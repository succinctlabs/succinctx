use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::builder::CircuitBuilder;
use crate::ops::{BitAnd, BitOr, BitXor, Not};

/// A variable in the circuit representing a boolean value.
#[derive(Debug, Clone, Copy, Default)]
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

    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), 1);
        Self(Variable(targets[0]))
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
        builder.add(self.0, rhs.0).into()
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
