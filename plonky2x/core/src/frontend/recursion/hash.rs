use plonky2::hash::hash_types::{HashOutTarget, MerkleCapTarget};
use plonky2::hash::merkle_proofs::{MerkleProof, MerkleProofTarget};
use plonky2::hash::merkle_tree::MerkleCap;
use plonky2::plonk::config::AlgebraicHasher;

use crate::frontend::hash::poseidon::poseidon256::PoseidonHashOutVariable;
use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleCapVariable(pub Vec<PoseidonHashOutVariable>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleProofVariable {
    /// The Merkle digest of each sibling subtree, staying from the bottommost layer.
    pub siblings: Vec<PoseidonHashOutVariable>,
}

impl VariableStream {
    pub fn read_merkle_cap(&mut self, cap_height: usize) -> MerkleCapVariable {
        let len = 1 << cap_height;
        let mut cap = Vec::with_capacity(len);
        for _ in 0..len {
            cap.push(self.read::<PoseidonHashOutVariable>());
        }
        MerkleCapVariable(cap)
    }

    pub fn write_merkle_cap(&mut self, cap: &MerkleCapVariable) -> usize {
        for elt in cap.0.iter() {
            self.write(elt);
        }
        cap.0.len()
    }

    pub fn read_merkle_proof(&mut self, len: usize) -> MerkleProofVariable {
        MerkleProofVariable {
            siblings: (0..len)
                .map(|_| self.read::<PoseidonHashOutVariable>())
                .collect(),
        }
    }

    pub fn write_merkle_proof(&mut self, proof: &MerkleProofVariable) -> usize {
        for elt in proof.siblings.iter() {
            self.write(elt);
        }
        proof.siblings.len()
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    pub fn read_merkle_cap<H: AlgebraicHasher<L::Field>>(
        &mut self,
        cap_height: usize,
    ) -> MerkleCap<L::Field, H> {
        let len = 1 << cap_height;
        MerkleCap(
            (0..len)
                .map(|_| self.read_value::<PoseidonHashOutVariable>())
                .collect(),
        )
    }

    pub fn write_merkle_cap<H: AlgebraicHasher<L::Field>>(
        &mut self,
        cap: MerkleCap<L::Field, H>,
    ) -> usize {
        let len = cap.0.len();
        for elt in cap.0 {
            self.write_value::<PoseidonHashOutVariable>(elt);
        }
        len
    }

    pub fn read_merkle_proof<H: AlgebraicHasher<L::Field>>(
        &mut self,
        len: usize,
    ) -> MerkleProof<L::Field, H> {
        MerkleProof {
            siblings: (0..len)
                .map(|_| self.read_value::<PoseidonHashOutVariable>())
                .collect(),
        }
    }

    pub fn write_merkle_proof<H: AlgebraicHasher<L::Field>>(
        &mut self,
        proof: MerkleProof<L::Field, H>,
    ) -> usize {
        let len = proof.siblings.len();
        for elt in proof.siblings {
            self.write_value::<PoseidonHashOutVariable>(elt);
        }
        len
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
            cap.push(self.read::<PoseidonHashOutVariable>(builder));
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
                .map(|_| self.read::<PoseidonHashOutVariable>(builder))
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
                .map(PoseidonHashOutVariable::from)
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

impl From<MerkleCapVariable> for MerkleCapTarget {
    fn from(target: MerkleCapVariable) -> Self {
        Self(target.0.into_iter().map(HashOutTarget::from).collect())
    }
}

impl From<MerkleCapTarget> for MerkleCapVariable {
    fn from(target: MerkleCapTarget) -> Self {
        Self(
            target
                .0
                .into_iter()
                .map(PoseidonHashOutVariable::from)
                .collect(),
        )
    }
}
