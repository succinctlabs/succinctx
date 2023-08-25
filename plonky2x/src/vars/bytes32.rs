use std::fmt::Debug;

use ethers::types::H256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::ops::{PartialEq};
use crate::vars::{BytesVariable, BoolVariable, ByteVariable};

/// A variable in the circuit representing a byte32 value.
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub BytesVariable<32>);

impl Bytes32Variable {
    pub fn as_slice(&self) -> [ByteVariable; 32] {
        self.0.0
    }
}

impl From<[ByteVariable; 32]> for Bytes32Variable {
    fn from(bytes: [ByteVariable; 32]) -> Self {
        Self(BytesVariable(bytes))
    }
}

impl From<&[ByteVariable]> for Bytes32Variable {
    fn from(bytes: &[ByteVariable]) -> Self {
        let bytes_fixed: [ByteVariable; 32] = bytes.try_into().unwrap();
        Self(BytesVariable(bytes_fixed))
    }
}

impl CircuitVariable for Bytes32Variable {
    type ValueType<F: Debug> = H256;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(BytesVariable::constant(builder, value.0))
    }

    fn targets(&self) -> Vec<Target> {
        self.0.targets()
    }

    fn from_targets(targets: &[Target]) -> Self {
        Self(BytesVariable::from_targets(targets))
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let bytes = self.0.value(witness);
        H256::from_slice(&bytes[..])
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
    }
}

impl<F: RichField + Extendable<D>, const D: usize> PartialEq<F, D>
    for Bytes32Variable
{
    fn eq(self, rhs: Bytes32Variable, builder: &mut CircuitBuilder<F, D>) -> BoolVariable {
        let mut result = builder.init::<BoolVariable>();
        for i in 0..32 {
            let lhs_byte = self.0.0[i];
            let rhs_byte = rhs.0.0[i];
            let byte_eq = builder.eq(lhs_byte, rhs_byte);
            result = builder.and(result, byte_eq);
        }
        result
    }

}