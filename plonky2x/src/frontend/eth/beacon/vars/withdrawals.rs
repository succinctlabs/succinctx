use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2x_derive::CircuitVariable;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::prelude::Variable;

#[derive(Debug, Clone, Copy, CircuitVariable)]
#[value_name(BeaconWithdrawalsValue)]
pub struct BeaconWithdrawalsVariable {
    pub block_root: Bytes32Variable,
    pub withdrawals_root: Bytes32Variable,
}
