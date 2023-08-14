use ethers::types::H256;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::PartitionWitness;

use super::CircuitVariable;
use crate::builder::{CircuitBuilder, ExtendableField};
use crate::vars::BytesVariable;

/// A variable in the circuit representing a byte32 value.
pub struct Bytes32Variable(pub BytesVariable<32>);

impl<F: ExtendableField> CircuitVariable<F> for Bytes32Variable {
    type ValueType = H256;

    fn init(builder: &mut CircuitBuilder<F>) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant(builder: &mut CircuitBuilder<F>, value: H256) -> Self {
        let bytes = to_padded_bytes(value);
        Self(BytesVariable::constant(builder, bytes.to_vec()))
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, F>) -> H256 {
        let bytes = self.0.value(witness);
        H256::from_slice(&bytes[..])
    }

    fn set(&self, buffer: &mut GeneratedValues<F>, value: H256) {
        let bytes = to_padded_bytes(value);
        self.0.set(buffer, bytes.to_vec());
    }
}

fn to_padded_bytes(value: H256) -> Vec<u8> {
    let slice = value.as_bytes();
    let mut bytes = [0u8; 256];
    bytes[..slice.len()].copy_from_slice(slice);
    bytes.to_vec()
}
