use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;

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
