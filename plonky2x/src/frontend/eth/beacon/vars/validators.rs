use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};
use plonky2x_derive::CircuitVariable;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::prelude::Variable;

/// The container which holds all beacon validators at specific block root as variable in the
/// circuit. Note that under the hood, we only store the commitment to the validators. To access
/// the underlying data, we witness merkle proofs.
#[derive(Debug, Clone, Copy, CircuitVariable)]
#[value_name(BeaconValidatorsValue)]
pub struct BeaconValidatorsVariable {
    pub block_root: Bytes32Variable,
    pub validators_root: Bytes32Variable,
}
