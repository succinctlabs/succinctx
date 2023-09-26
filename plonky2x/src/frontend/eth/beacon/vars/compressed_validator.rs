use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::prelude::{Bytes32Variable, CircuitBuilder, CircuitVariable, PlonkParameters, Variable};

#[derive(Debug, Clone, CircuitVariable)]
pub struct CompressedBeaconValidatorVariable {
    pub pubkey: BLSPubkeyVariable,
    pub withdrawal_credentials: Bytes32Variable,
}
