use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::prelude::*;

#[derive(Clone, Debug, CircuitVariable)]
pub struct MerkleInclusionProofVariable<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize> {
    pub aunts: ArrayVariable<Bytes32Variable, PROOF_DEPTH>,
    pub path_indices: ArrayVariable<BoolVariable, PROOF_DEPTH>,
    pub enc_leaf: BytesVariable<LEAF_SIZE_BYTES>,
}

#[derive(Clone, Debug, CircuitVariable)]
pub struct MerkleInclusionProofVariableTest {
    pub aunts: ArrayVariable<Bytes32Variable, 5>,
    pub path_indices: ArrayVariable<BoolVariable, 5>,
    pub enc_leaf: BytesVariable<5>,
}
