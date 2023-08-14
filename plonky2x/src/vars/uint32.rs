use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::builder::{CircuitBuilder, ExtendableField};

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
pub struct U32Variable(pub Variable);

impl<F: ExtendableField> CircuitVariable<F> for U32Variable {
    type ValueType = u32;

    fn init(builder: &mut CircuitBuilder<F>) -> Self {
        Self(Variable::init(builder))
    }

    fn constant(builder: &mut CircuitBuilder<F>, value: u32) -> Self {
        Self(Variable::constant(builder, F::from_canonical_u32(value)))
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, F>) -> u32 {
        let v = witness.get_target(self.0 .0);
        todo!()
    }

    fn set(&self, witness: &mut GeneratedValues<F>, value: u32) {
        witness.set_target(self.0 .0, F::from_canonical_u32(value));
    }
}
