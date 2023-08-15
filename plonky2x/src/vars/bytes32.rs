use ethers::types::H256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::vars::BytesVariable;

/// A variable in the circuit representing a byte32 value.
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub BytesVariable<32>);

impl CircuitVariable for Bytes32Variable {
    type ValueType = H256;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        let bytes = to_padded_bytes(value);
        Self(BytesVariable::constant(builder, bytes.to_vec()))
    }

    fn targets(&self) -> Vec<Target> {
        self.0.targets()
    }

    fn from_targets(targets: &[Target]) -> Self {
        Self(BytesVariable::from_targets(targets))
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        let bytes = self.0.value(witness);
        H256::from_slice(&bytes[..])
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        let bytes = to_padded_bytes(value);
        self.0.set(witness, bytes.to_vec());
    }
}

fn to_padded_bytes(value: H256) -> Vec<u8> {
    let slice = value.as_bytes();
    let mut bytes = [0u8; 32];
    bytes[..slice.len()].copy_from_slice(slice);
    bytes.to_vec()
}
