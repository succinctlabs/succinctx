use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy, Default)]
pub struct U32Variable(pub Variable);

impl CircuitVariable for U32Variable {
    type ValueType = u32;

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

    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), 1);
        Self(Variable(targets[0]))
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        let v = witness.get_target(self.0 .0);
        v.to_canonical_u64() as u32
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        witness.set_target(self.0 .0, F::from_canonical_u32(value));
    }
}
