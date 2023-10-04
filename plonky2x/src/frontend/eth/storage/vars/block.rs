use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2x_derive::CircuitVariable;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, U256Variable};
use crate::prelude::Variable;

/// A variable representing the Ethereum Block Header
/// Follow the following struct in go-ethereum
/// https://github.com/ethereum/go-ethereum/blob/b6d4f6b66e99c08f419e6a469259cbde1c8b0582/core/types/block.go#L70
/// https://github.com/gnosis/hashi/blob/main/packages/evm/contracts/adapters/BlockHashOracleAdapter.sol#L24

/// Includes a subset of fields in a block header
#[derive(Debug, Clone, Copy, CircuitVariable)]
#[value_name(EthHeader)]
#[value_derive(PartialEq, Eq)]
pub struct EthHeaderVariable {
    pub parent_hash: Bytes32Variable,
    pub uncle_hash: Bytes32Variable,
    pub coinbase: AddressVariable,
    pub root: Bytes32Variable,
    pub tx_hash: Bytes32Variable,
    pub receipt_hash: Bytes32Variable,
    // pub bloom: BytesVariable, // TODO: add back once we have arbitrary bytes variables
    pub difficulty: U256Variable,
    pub number: U64Variable,
    pub gas_limit: U256Variable,
    pub gas_used: U256Variable,
    pub time: U256Variable,
    // pub extra: Bytes32Variable, // TODO: add back once we have arbitrary bytes variables
}
