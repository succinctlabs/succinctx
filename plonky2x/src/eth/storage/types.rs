
use crate::vars::{Bytes32Variable, Variable, U256Variable, BytesVariable, ByteVariable};
use crate::eth::types::{AddressVariable};

#[derive(Debug)]
pub struct ProofVariable {
    proof: Bytes32Variable
}


#[derive(Debug)]
pub struct AccountVariable {
    balance: U256Variable,
    code_hash: Bytes32Variable,
    nonce: U256Variable,
    storage_hash: Bytes32Variable
}


impl AccountVariable {
    pub fn serialize(&self) -> &[ByteVariable] {
        return self.code_hash;
    }
}