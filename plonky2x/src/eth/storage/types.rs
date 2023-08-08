
use crate::vars::{Bytes32Variable, Variable, U256Variable, BytesVariable, ByteVariable};
use crate::eth::types::{AddressVariable};

#[derive(Debug)]
pub struct ProofVariable {
    proof: Bytes32Variable
}


#[derive(Debug)]
pub struct AccountVariable {
    pub balance: U256Variable,
    pub code_hash: Bytes32Variable,
    pub nonce: U256Variable,
    pub storage_hash: Bytes32Variable
}


impl AccountVariable {
    pub fn serialize(&self) -> Vec<ByteVariable> {
        return self.code_hash.0[..].to_vec();
    }
}