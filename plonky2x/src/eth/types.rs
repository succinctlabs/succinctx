use crate::builder::BuilderAPI;
use crate::vars::{BoolVariable, BytesVariable, VariableMethods};
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};


#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub [BoolVariable; 512]);
// impl_variable_methods!(BLSPubkeyVariable, 512);

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub [BoolVariable; 160]);

impl VariableMethods<160> for AddressVariable {
    fn get_bits<F: Field, W: Witness<F>>(&self, witness: W) -> [bool; 160] {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<160>(self.0);
        temp.get_bits(witness)
    }

    fn get_bytes_le<F: Field, W: Witness<F>>(&self, witness: W) -> [u8; 20] {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<160>(self.0);
        temp.get_bytes_le(witness)
    }

    fn set_from_bits<F: Field, G: WitnessWrite<F>>(&self, values: [bool; 160], out_buffer: &mut G) {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<160>(self.0);
        temp.set_from_bits(values, out_buffer)
    }

    fn set_from_bytes<F: Field, G: WitnessWrite<F>>(&self, values: [u8; 20], out_buffer: &mut G) {
        // Create a temporary BytesVariable from self and use its logic
        let temp = BytesVariable::<160>(self.0);
        temp.set_from_bytes(values, out_buffer)
    }
}

impl BuilderAPI {
    /// Initialize a new BLSPubkeyVariable.
    pub fn init_bls_pubkey(&mut self) -> BLSPubkeyVariable {
        BLSPubkeyVariable([self.init_bool(); 512])
    }
}
