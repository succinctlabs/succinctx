use curta::prelude::Field;
use ethers::types::H256;
use itertools::Itertools;
use num::pow;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::merkle::utils::log2_ceil_usize;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{
    ArrayVariable, ByteVariable, BytesVariable, CircuitBuilder, CircuitVariable, Variable,
};

pub trait SimpleMerkleTree {
    fn leaf_hash(&mut self, leaf: &[ByteVariable]) -> Bytes32Variable;

    fn inner_hash(&mut self, left: &Bytes32Variable, right: &Bytes32Variable) -> Bytes32Variable;

    fn hash_merkle_layer(&mut self, merkle_hashes: Vec<Bytes32Variable>) -> Vec<Bytes32Variable>;

    fn hash_leaves<const LEAF_SIZE_BYTES: usize>(
        &mut self,
        leaves: Vec<BytesVariable<LEAF_SIZE_BYTES>>,
    ) -> Vec<Bytes32Variable>;

    fn get_root_from_hashed_leaves<const MAX_NB_LEAVES: usize>(
        &mut self,
        leaf_hashes: ArrayVariable<Bytes32Variable, MAX_NB_LEAVES>,
        nb_enabled_leaves: Variable,
    ) -> Bytes32Variable;

    fn compute_root_from_leaves<const MAX_NB_LEAVES: usize, const LEAF_SIZE_BYTES: usize>(
        &mut self,
        leaves: ArrayVariable<BytesVariable<LEAF_SIZE_BYTES>, MAX_NB_LEAVES>,
        nb_enabled_leaves: Variable,
    ) -> Bytes32Variable;
}

/// Implementation of a simple merkle tree.
/// Does not add a pre-image prefix to the leaf/inner nodes.
/// Fills in leaf nodes with empty bytes as pre-image.
impl<L: PlonkParameters<D>, const D: usize> SimpleMerkleTree for CircuitBuilder<L, D> {
    fn leaf_hash(&mut self, leaf: &[ByteVariable]) -> Bytes32Variable {
        // Load the output of the hash.
        self.curta_sha256(leaf)
    }

    fn inner_hash(&mut self, left: &Bytes32Variable, right: &Bytes32Variable) -> Bytes32Variable {
        // Append the left bytes.
        let mut encoded_leaf = left.as_bytes().to_vec();

        // Append the right bytes to the bytes so far.
        encoded_leaf.extend(right.as_bytes().to_vec());

        // Load the output of the hash.
        self.curta_sha256(&encoded_leaf)
    }

    fn hash_merkle_layer(&mut self, merkle_hashes: Vec<Bytes32Variable>) -> Vec<Bytes32Variable> {
        let mut new_merkle_hashes = Vec::new();

        for i in (0..merkle_hashes.len()).step_by(2) {
            // Calculuate the inner hash.
            new_merkle_hashes.push(self.inner_hash(&merkle_hashes[i], &merkle_hashes[i + 1]));
        }

        // Return the hashes and enabled nodes for the next layer up.
        new_merkle_hashes
    }

    fn hash_leaves<const LEAF_SIZE_BYTES: usize>(
        &mut self,
        leaves: Vec<BytesVariable<LEAF_SIZE_BYTES>>,
    ) -> Vec<Bytes32Variable> {
        leaves
            .iter()
            .map(|leaf| self.leaf_hash(&leaf.0))
            .collect_vec()
    }

    fn get_root_from_hashed_leaves<const MAX_NB_LEAVES: usize>(
        &mut self,
        leaf_hashes: ArrayVariable<Bytes32Variable, MAX_NB_LEAVES>,
        nb_enabled_leaves: Variable,
    ) -> Bytes32Variable {
        let empty_bytes = Bytes32Variable::constant(self, H256::from_slice(&[0u8; 32]));

        // Extend leaf_hashes and leaves_enabled to be a power of 2.
        let padded_nb_leaves = pow(2, log2_ceil_usize(MAX_NB_LEAVES));
        assert!(padded_nb_leaves >= MAX_NB_LEAVES && padded_nb_leaves.is_power_of_two());

        // Hash each of the validators to get their corresponding leaf hash.
        // Pad the leaves to be a power of 2.
        let mut current_nodes = leaf_hashes.data;
        current_nodes.resize(padded_nb_leaves, empty_bytes);

        // Fill in the disabled leaves with empty bytes.
        for i in 0..padded_nb_leaves {
            let idx = self.constant::<Variable>(L::Field::from_canonical_usize(i));
            let enabled = self.lt(idx, nb_enabled_leaves);
            current_nodes[i] = self.select(enabled, current_nodes[i], empty_bytes)
        }

        let mut merkle_layer_size = padded_nb_leaves;
        // Hash each layer of nodes to get the root.
        while merkle_layer_size > 1 {
            current_nodes = self.hash_merkle_layer(current_nodes);
            merkle_layer_size /= 2;
        }

        // Return the root hash.
        current_nodes[0]
    }

    fn compute_root_from_leaves<const MAX_NB_LEAVES: usize, const LEAF_SIZE_BYTES: usize>(
        &mut self,
        leaves: ArrayVariable<BytesVariable<LEAF_SIZE_BYTES>, MAX_NB_LEAVES>,
        nb_enabled_leaves: Variable,
    ) -> Bytes32Variable {
        let hashed_leaves = self.hash_leaves::<LEAF_SIZE_BYTES>(leaves.as_vec());
        let hashed_leaves = ArrayVariable::<Bytes32Variable, MAX_NB_LEAVES>::new(hashed_leaves);
        self.get_root_from_hashed_leaves::<MAX_NB_LEAVES>(hashed_leaves, nb_enabled_leaves)
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::merkle::simple::SimpleMerkleTree;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;
    type F = GoldilocksField;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_root_from_leaves_simple() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let leaves = builder.read::<ArrayVariable<BytesVariable<48>, 32>>();
        let nb_enabled_leaves = builder.read::<Variable>();
        let root = builder.compute_root_from_leaves::<32, 48>(leaves, nb_enabled_leaves);
        builder.write::<Bytes32Variable>(root);
        let circuit = builder.build();
        circuit.test_default_serializers();

        let mut input = circuit.input();

        input.write::<ArrayVariable<BytesVariable<48>, 32>>([[0u8; 48]; 32].to_vec());
        input.write::<Variable>(F::from_canonical_usize(32));

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let root = output.read::<Bytes32Variable>();

        assert_eq!(
            root,
            bytes32!("0x31fe8409d402eacbabe198c26363af456b59f9c4bdab9ede33e278a1a527a399"),
        );
    }
}
