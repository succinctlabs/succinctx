use plonky2::util::serialization::Write;

use crate::builder::BuilderAPI;
use crate::vars::BoolVariable;

#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub [BoolVariable; 512]);

#[derive(Debug, Clone, Copy)]
pub struct U256Variable(pub [BoolVariable; 256]);

impl BuilderAPI {
    /// Initialize a new U256Variable.
    pub fn init_u256(&mut self) -> U256Variable {
        U256Variable([self.init_bool(); 256])
    }

    /// Initialize a new Bytes32Variable.
    pub fn init_bytes32(&mut self) -> Bytes32Variable {
        Bytes32Variable([self.init_bool(); 256])
    }

    /// Initialize a new BLSPubkeyVariable.
    pub fn init_bls_pubkey(&mut self) -> BLSPubkeyVariable {
        BLSPubkeyVariable([self.init_bool(); 512])
    }
}

// trait EthTypesWrite: Write {
//     fn write_u256(&mut self, value: U256Variable) {}
// }
