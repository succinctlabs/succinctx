use core::str::Bytes;

use crate::builder::BuilderAPI;
use crate::vars::{BoolVariable, BytesVariable};
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};


#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub [BoolVariable; 512]);
// impl_variable_methods!(BLSPubkeyVariable, 512);

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub [BoolVariable; 160]);

impl From<AddressVariable> for BytesVariable<160> {
    fn from(value: AddressVariable) -> Self {
        Self(value.0)
    }
}

impl BuilderAPI {
    /// Initialize a new BLSPubkeyVariable.
    pub fn init_bls_pubkey(&mut self) -> BLSPubkeyVariable {
        BLSPubkeyVariable([self.init_bool(); 512])
    }
}
