use curta::math::field::PrimeField64;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;

/// A variable in the circuit. It represents a value between `[0, 2**64 - 2**32 + 1)`.
#[derive(Debug, Clone, Copy)]
pub struct Variable(pub Target);

impl CircuitVariable for Variable {
    type ValueType = u64;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        let target = builder.api.add_virtual_target();
        Self(target)
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        let target = builder.api.constant(F::from_canonical_u64(value));
        Self(target)
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0]
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> u64 {
        witness.get_target(self.0).as_canonical_u64()
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: u64) {
        witness.set_target(self.0, F::from_canonical_u64(value));
    }
}

impl From<Target> for Variable {
    fn from(target: Target) -> Self {
        Self(target)
    }
}
