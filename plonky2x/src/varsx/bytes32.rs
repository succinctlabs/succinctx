use ethers::types::H256;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::PartitionWitness;

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::varsx::BytesVariable;

/// A variable in the circuit representing a byte32 value.
pub struct Bytes32Variable(BytesVariable<32>);

impl CircuitVariable for Bytes32Variable {
    type Value = H256;

    fn init(builder: &mut CircuitBuilder) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant(builder: &mut CircuitBuilder, value: H256) -> Self {
        let bytes = to_padded_bytes(value);
        Self(BytesVariable::constant(builder, bytes.to_vec()))
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> H256 {
        let bytes = self.0.value(witness);
        H256::from_slice(&bytes[..])
    }

    fn set(&self, buffer: &mut GeneratedValues<GoldilocksField>, value: H256) {
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
