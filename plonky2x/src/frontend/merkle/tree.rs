use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::prelude::*;

#[derive(Clone, Debug, CircuitVariable)]
#[value_name(InclusionProof)]
pub struct MerkleInclusionProofVariable<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize> {
    pub aunts: ArrayVariable<Bytes32Variable, PROOF_DEPTH>,
    pub path_indices: ArrayVariable<BoolVariable, PROOF_DEPTH>,
    pub leaf: BytesVariable<LEAF_SIZE_BYTES>,
}
