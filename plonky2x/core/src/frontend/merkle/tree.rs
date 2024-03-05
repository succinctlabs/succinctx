use crate::prelude::*;

#[derive(Clone, Debug, CircuitVariable)]
#[value_name(InclusionProof)]
pub struct MerkleInclusionProofVariable<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize> {
    pub proof: ArrayVariable<Bytes32Variable, PROOF_DEPTH>,
    pub leaf: BytesVariable<LEAF_SIZE_BYTES>,
}
