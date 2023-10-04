use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2x_derive::CircuitVariable;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, U256Variable};
use crate::prelude::Variable;

#[derive(Debug, Clone, Copy, CircuitVariable)]
#[value_name(EthProof)]
pub struct EthProofVariable {
    pub proof: Bytes32Variable,
}

#[derive(Debug, Clone, Copy, CircuitVariable)]
#[value_name(EthAccount)]
pub struct EthAccountVariable {
    pub balance: U256Variable,
    pub code_hash: Bytes32Variable,
    pub nonce: U256Variable,
    pub storage_hash: Bytes32Variable,
}

#[derive(Debug, Clone, Copy, CircuitVariable)]
#[value_name(EthLog)]
#[value_derive(PartialEq, Eq)]
pub struct EthLogVariable {
    pub address: AddressVariable,
    pub topics: [Bytes32Variable; 3],
    pub data_hash: Bytes32Variable,
}
