use std::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{CircuitVariable, FieldSerializable, Variable};

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U32Variable(pub Variable);

impl CircuitVariable for U32Variable {
    type ValueType<F: RichField> = u32;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(Variable::init(builder))
    }

    fn targets(&self) -> Vec<Target> {
        vec![self.0 .0]
    }

    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), 1);
        Self(Variable(targets[0]))
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let v = witness.get_target(self.0 .0);
        v.to_canonical_u64() as u32
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        witness.set_target(self.0 .0, F::from_canonical_u32(value));
    }
}

impl<F: RichField> FieldSerializable<F> for u32 {
    fn nb_elements() -> usize {
        1
    }

    fn elements(&self) -> Vec<F> {
        vec![F::from_canonical_u32(*self)]
    }

    fn from_elements(elements: &[F]) -> Self {
        assert_eq!(elements.len(), 1);
        elements[0].to_canonical_u64() as u32
    }
}
