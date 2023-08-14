use super::vars::{AddressVariable, BLSPubkeyVariable};
use crate::builder::CircuitBuilder;
use crate::vars::BoolVariable;

impl CircuitBuilder {
    /// Initialize a new BLSPubkeyVariable.
    pub fn init_bls_pubkey(&mut self) -> BLSPubkeyVariable {
        let mut bytes = [BoolVariable::default(); 384];
        for i in 0..384 {
            bytes[i] = self.init_bool();
        }
        BLSPubkeyVariable(bytes)
    }

    /// Initialize a new Bytes32Variable.
    pub fn init_address(&mut self) -> AddressVariable {
        let mut bytes = [BoolVariable::default(); 160];
        for i in 0..160 {
            bytes[i] = self.init_bool();
        }
        AddressVariable(bytes)
    }
}
