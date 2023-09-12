use ethers::types::H256;
use itertools::Itertools;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use super::tree::MerkleInclusionProofVariable;
use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, BytesVariable, CircuitBuilder, CircuitVariable,
};

/// Merkle Tree implementation for the Tendermint spec (follows RFC-6962: https://tools.ietf.org/html/rfc6962).
/// TODO: Add a generic interface for Merkle trees to implement.
impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn get_root_from_merkle_proof<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize>(
        &mut self,
        inclusion_proof: &MerkleInclusionProofVariable<PROOF_DEPTH, LEAF_SIZE_BYTES>,
    ) -> Bytes32Variable {
        let mut hash_so_far = self.leaf_hash(&inclusion_proof.enc_leaf.0);

        for i in 0..PROOF_DEPTH {
            let aunt = inclusion_proof.aunts[i];
            let path_index = inclusion_proof.path_indices[i];
            let left_hash_pair = self.inner_hash(&hash_so_far, &aunt);
            let right_hash_pair = self.inner_hash(&aunt, &hash_so_far);

            hash_so_far = self.select(path_index, right_hash_pair, left_hash_pair)
        }
        hash_so_far
    }

    fn leaf_hash(&mut self, leaf: &[ByteVariable]) -> Bytes32Variable {
        let zero_byte = ByteVariable::constant(self, 0u8);

        let mut encoded_leaf = vec![zero_byte];

        // Append the leaf bytes to the zero byte.
        encoded_leaf.extend(leaf.to_vec());

        // Load the output of the hash.
        // Use curta gadget to generate SHA's.
        // Note: This can be removed when sha256 interface is fixed.
        self.curta_sha256(&encoded_leaf)
    }

    fn inner_hash(&mut self, left: &Bytes32Variable, right: &Bytes32Variable) -> Bytes32Variable {
        // Calculate the length of the message for the inner hash.
        // 0x01 || left || right
        let one_byte = ByteVariable::constant(self, 1u8);

        let mut encoded_leaf = vec![one_byte];

        // Append the left bytes to the one byte.
        encoded_leaf.extend(left.as_bytes().to_vec());

        // Append the right bytes to the bytes so far.
        encoded_leaf.extend(right.as_bytes().to_vec());

        // Load the output of the hash.
        // Note: Calculate the inner hash as if both validators are enabled.
        self.curta_sha256(&encoded_leaf)
    }

    fn hash_merkle_layer(
        &mut self,
        merkle_hashes: Vec<Bytes32Variable>,
        merkle_hash_enabled: Vec<BoolVariable>,
        num_hashes: usize,
    ) -> (Vec<Bytes32Variable>, Vec<BoolVariable>) {
        let zero = self._false();
        let one = self._true();

        let mut new_merkle_hashes = Vec::new();
        let mut new_merkle_hash_enabled = Vec::new();

        for i in (0..num_hashes).step_by(2) {
            let both_nodes_enabled = self.and(merkle_hash_enabled[i], merkle_hash_enabled[i + 1]);

            let first_node_disabled = self.not(merkle_hash_enabled[i]);
            let second_node_disabled = self.not(merkle_hash_enabled[i + 1]);
            let both_nodes_disabled = self.and(first_node_disabled, second_node_disabled);

            // Calculuate the inner hash.
            let inner_hash = self.inner_hash(&merkle_hashes[i], &merkle_hashes[i + 1]);

            new_merkle_hashes.push(self.select(both_nodes_enabled, inner_hash, merkle_hashes[i]));

            // Set the inner node one level up to disabled if both nodes are disabled.
            new_merkle_hash_enabled.push(self.select(both_nodes_disabled, zero, one));
        }

        // Return the hashes and enabled nodes for the next layer up.
        (new_merkle_hashes, new_merkle_hash_enabled)
    }

    fn hash_leaves<const NB_LEAVES: usize, const LEAF_SIZE_BYTES: usize>(
        &mut self,
        leaves: &ArrayVariable<BytesVariable<LEAF_SIZE_BYTES>, NB_LEAVES>,
    ) -> ArrayVariable<Bytes32Variable, NB_LEAVES>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        ArrayVariable::<Bytes32Variable, NB_LEAVES>::new(
            leaves
                .as_vec()
                .iter()
                .map(|leaf| self.leaf_hash(&leaf.0))
                .collect_vec(),
        )
    }

    /// Fixed-size leaves, fixed number of leaves.
    /// TODO: Ideally, this function takes in NUM_LEAVES_ENABLED as a target, and sets the rest to disabled.
    /// Note: This function assumes the leaves are already hashed.
    fn get_root_from_hashed_leaves<const NB_ENABLED_LEAVES: usize, const NB_LEAVES: usize>(
        &mut self,
        leaf_hashes: &ArrayVariable<Bytes32Variable, NB_ENABLED_LEAVES>,
    ) -> Bytes32Variable
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        assert!(NB_ENABLED_LEAVES <= NB_LEAVES);
        assert!(NB_LEAVES.is_power_of_two());

        let mut leaves = leaf_hashes.as_vec();
        leaves.extend(vec![
            self.constant::<Bytes32Variable>(H256::default());
            NB_LEAVES - NB_ENABLED_LEAVES
        ]);

        let mut leaf_enabled = vec![self._true(); NB_ENABLED_LEAVES];
        leaf_enabled.extend(vec![self._false(); NB_LEAVES - NB_ENABLED_LEAVES]);

        // Hash each of the validators to get their corresponding leaf hash.
        let mut current_nodes = leaves.clone();

        // Whether to treat the validator as empty.
        let mut current_node_enabled = leaf_enabled.clone();

        let mut merkle_layer_size = NB_LEAVES;

        // Hash each layer of nodes to get the root according to the Tendermint spec, starting from the leaves.
        while merkle_layer_size > 1 {
            (current_nodes, current_node_enabled) =
                self.hash_merkle_layer(current_nodes, current_node_enabled, merkle_layer_size);
            merkle_layer_size /= 2;
        }

        // Return the root hash.
        current_nodes[0]
    }

    // TODO: Is there a clean way to compute NB_LEAVES from NB_ENABLED_LEAVES to avoid having to pass it in?
    fn compute_root_from_leaves<
        const NB_ENABLED_LEAVES: usize,
        const NB_LEAVES: usize,
        const LEAF_SIZE_BYTES: usize,
    >(
        &mut self,
        leaves: &ArrayVariable<BytesVariable<LEAF_SIZE_BYTES>, NB_ENABLED_LEAVES>,
    ) -> Bytes32Variable
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let hashed_leaves = self.hash_leaves(leaves);
        self.get_root_from_hashed_leaves::<NB_ENABLED_LEAVES, NB_LEAVES>(&hashed_leaves)
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_root_from_merkle_proof() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let array =
            builder.constant::<ArrayVariable<BytesVariable<48>, 32>>([[0u8; 48]; 32].to_vec());

        let root = builder.compute_root_from_leaves::<32, 32, 48>(&array);

        builder.watch(&root, "root");
        builder.curta_constrain_sha256();

        let expected_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xde8624485c0a1b8f9ecc858312916104cc3ee3ed601e405c11eaf9c5cbe05117"
        ));
        builder.assert_is_equal(root, expected_root);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        // circuit.verify(&proof, &input, &output);
        // circuit.test_default_serializers();
    }
}
