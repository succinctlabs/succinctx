use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a boolean value.
#[derive(Debug, Clone, Copy)]
pub struct BoolVariable(pub Variable);

impl CircuitVariable for BoolVariable {
    type ValueType = bool;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(Variable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        Self(Variable::constant(builder, value as u64))
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0 .0]
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        witness.get_target(self.0 .0) == F::from_canonical_u64(1)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
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
