use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};

use super::{BoolVariable, BytesVariable, ByteVariable};
use super::bytes::VariableMethods;
use crate::utils::{le_bits_to_bytes, bytes_to_bits};
// use crate::impl_variable_methods;
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);


impl VariableMethods<256> for Bytes32Variable {
    fn get_bits<F: Field, W: Witness<F>>(&self, witness: &W) -> [bool; 256] {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<256>(self.0);
        temp.get_bits(witness)
    }

    fn get_bytes_le<F: Field, W: Witness<F>>(&self, witness: &W) -> [u8; 32] {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<256>(self.0);
        temp.get_bytes_le(witness)
    }

    fn set_from_bits<F: Field, G: WitnessWrite<F>>(&self, values: [bool; 256], out_buffer: &mut G) {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<256>(self.0);
        temp.set_from_bits(values, out_buffer)
    }

    fn set_from_bytes<F: Field, G: WitnessWrite<F>>(&self, values: [u8; 32], out_buffer: &mut G) {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<256>(self.0);
        temp.set_from_bytes(values, out_buffer)
    }
}