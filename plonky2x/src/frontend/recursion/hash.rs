use plonky2::hash::hash_types::{HashOutTarget, MerkleCapTarget, NUM_HASH_OUT_ELTS};
use plonky2::hash::merkle_proofs::MerkleProofTarget;

use crate::frontend::vars::{OutputVariableStream, VariableStream};
use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleCapVariable(pub Vec<HashOutVariable>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleProofVariable {
    /// The Merkle digest of each sibling subtree, staying from the bottommost layer.
    pub siblings: Vec<HashOutVariable>,
}

/// Represents a ~256 bit hash output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, CircuitVariable)]
pub struct HashOutVariable {
    pub elements: [Variable; NUM_HASH_OUT_ELTS],
}

impl From<HashOutTarget> for HashOutVariable {
    fn from(target: HashOutTarget) -> Self {
        Self {
            elements: target.elements.map(Variable),
        }
    }
}

impl From<HashOutVariable> for HashOutTarget {
    fn from(target: HashOutVariable) -> Self {
        Self {
            elements: target.elements.map(|v| v.0),
        }
    }
}

impl From<MerkleCapVariable> for MerkleCapTarget {
    fn from(target: MerkleCapVariable) -> Self {
        Self(target.0.into_iter().map(HashOutTarget::from).collect())
    }
}

impl From<MerkleCapTarget> for MerkleCapVariable {
    fn from(target: MerkleCapTarget) -> Self {
        Self(target.0.into_iter().map(HashOutVariable::from).collect())
    }
}

impl VariableStream {
    pub fn read_merkle_cap(&mut self, cap_height: usize) -> MerkleCapVariable {
        let len = 1 << cap_height;
        let mut cap = Vec::with_capacity(len);
        for _ in 0..len {
            cap.push(self.read::<HashOutVariable>());
        }
        MerkleCapVariable(cap)
    }

    pub fn write_merkle_cap(&mut self, cap: MerkleCapVariable) -> usize {
        for elt in cap.0.iter() {
            self.write(elt);
        }
        cap.0.len()
    }

    pub fn read_merkle_proof(&mut self, len: usize) -> MerkleProofVariable {
        MerkleProofVariable {
            siblings: (0..len).map(|_| self.read::<HashOutVariable>()).collect(),
        }
    }

    pub fn write_merkle_proof(&mut self, proof: MerkleProofVariable) -> usize {
        for elt in proof.siblings.iter() {
            self.write(elt);
        }
        proof.siblings.len()
    }
}

impl<L: PlonkParameters<D>, const D: usize> OutputVariableStream<L, D> {
    pub fn read_merkle_cap(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        cap_height: usize,
    ) -> MerkleCapVariable {
        let len = 1 << cap_height;
        let mut cap = Vec::with_capacity(len);
        for _ in 0..(1 << cap_height) {
            cap.push(self.read::<HashOutVariable>(builder));
        }
        MerkleCapVariable(cap)
    }

    pub fn read_merkle_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        len: usize,
    ) -> MerkleProofVariable {
        MerkleProofVariable {
            siblings: (0..len)
                .map(|_| self.read::<HashOutVariable>(builder))
                .collect(),
        }
    }
}

impl From<MerkleProofTarget> for MerkleProofVariable {
    fn from(value: MerkleProofTarget) -> Self {
        Self {
            siblings: value
                .siblings
                .into_iter()
                .map(HashOutVariable::from)
                .collect(),
        }
    }
}

impl From<MerkleProofVariable> for MerkleProofTarget {
    fn from(value: MerkleProofVariable) -> Self {
        Self {
            siblings: value
                .siblings
                .into_iter()
                .map(HashOutTarget::from)
                .collect(),
        }
    }
}
