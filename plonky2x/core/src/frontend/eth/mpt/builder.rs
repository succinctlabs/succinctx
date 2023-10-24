use std::marker::PhantomData;

use curta::math::field::Field;

use super::generators::*;
use crate::frontend::vars::Nibbles;
use crate::prelude::{
    ArrayVariable, ByteVariable, Bytes32Variable, CircuitBuilder, PlonkParameters, Variable,
};

pub fn transform_proof_to_padded<const ENCODING_LEN: usize, const PROOF_LEN: usize>(
    storage_proof: Vec<Vec<u8>>,
) -> (Vec<Vec<u8>>, Vec<usize>) {
    if storage_proof.len() > PROOF_LEN {
        panic!(
            "Proof is too long, has {} elements, but PROOF_LEN is {}",
            storage_proof.len(),
            PROOF_LEN
        );
    }

    let mut padded_elements = vec![vec![0u8; ENCODING_LEN]; PROOF_LEN];
    let mut lengths = vec![0usize; PROOF_LEN];

    for (i, inner_vec) in storage_proof.into_iter().enumerate() {
        // Check inner length
        if inner_vec.len() > ENCODING_LEN {
            panic!(
                "Proof element {} is too long, has {} elements, but ENCODING_LEN is {}",
                i,
                inner_vec.len(),
                ENCODING_LEN
            );
        }
        lengths[i] = inner_vec.len();
        for (j, &byte) in inner_vec.iter().enumerate() {
            padded_elements[i][j] = byte;
        }
    }

    (padded_elements, lengths)
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn byte_to_variable(&mut self, lhs: ByteVariable) -> Variable {
        // TODO: Need to constrain generator result
        let generator: ByteToVariableGenerator<L, D> = ByteToVariableGenerator {
            lhs,
            output: self.init::<Variable>(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator.clone());
        generator.output
    }

    pub fn sub_byte(&mut self, lhs: ByteVariable, rhs: ByteVariable) -> ByteVariable {
        // TODO: Need to constrain generator result
        let generator: ByteSubGenerator<L, D> = ByteSubGenerator {
            lhs,
            rhs,
            output: self.init::<ByteVariable>(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator.clone());
        generator.output
    }

    const PREFIX_EXTENSION_EVEN: u8 = 0;
    const PREFIX_EXTENSION_ODD: u8 = 1;
    const PREFIX_LEAF_EVEN: u8 = 2;
    const PREFIX_LEAF_ODD: u8 = 3;
    /// Get the validators for a given block root.
    /// P is the number of proof elements to be considered
    pub fn verify_mpt_proof<const ENCODING_LEN: usize, const PROOF_LEN: usize>(
        &mut self,
        key: Bytes32Variable,
        proof: ArrayVariable<ArrayVariable<ByteVariable, ENCODING_LEN>, PROOF_LEN>,
        len_nodes: ArrayVariable<Variable, PROOF_LEN>,
        root: Bytes32Variable,
        value: Bytes32Variable,
    ) {
        const ELEMENT_LEN: usize = 34; // Maximum size of list element
        const LIST_LEN: usize = 17; // Maximum length of the list for each proof element

        let tree_radix = self.constant::<Variable>(L::Field::from_canonical_u8(16u8));
        let branch_node_length = self.constant::<Variable>(L::Field::from_canonical_u8(17u8));
        let leaf_or_extension_node_length =
            self.constant::<Variable>(L::Field::from_canonical_u8(2u8));
        let prefix_leaf_even = self.constant::<ByteVariable>(Self::PREFIX_LEAF_EVEN);
        let prefix_leaf_odd = self.constant::<ByteVariable>(Self::PREFIX_LEAF_ODD);
        let prefix_extension_even = self.constant::<ByteVariable>(Self::PREFIX_EXTENSION_EVEN);
        let prefix_extension_odd = self.constant::<ByteVariable>(Self::PREFIX_EXTENSION_ODD);
        let one: Variable = self.one::<Variable>();
        let two = self.constant::<Variable>(L::Field::from_canonical_u8(2));
        let const_64 = self.constant::<Variable>(L::Field::from_canonical_u8(64));
        let const_32 = self.constant::<Variable>(L::Field::from_canonical_u8(32));
        let const_128 = self.constant::<ByteVariable>(128);

        let mut current_key_idx = self.zero::<Variable>();
        let mut finished = self._false();

        let mut padded_root = root.as_bytes().to_vec();
        while padded_root.len() < ELEMENT_LEN {
            padded_root.push(self.constant::<ByteVariable>(0));
        }
        let mut current_node_id = ArrayVariable::<ByteVariable, ELEMENT_LEN>::new(padded_root);
        let hash_key = self.keccak256(&key.as_bytes());
        let key_path: ArrayVariable<ByteVariable, 64> = hash_key
            .as_bytes()
            .to_vec()
            .to_nibbles(self)
            .try_into()
            .unwrap();

        for i in 0..PROOF_LEN {
            let current_node = proof[i].clone();
            let current_node_hash = self.keccak256_variable(current_node.as_slice(), len_nodes[i]);

            if i == 0 {
                self.assert_is_equal(current_node_hash, root);
            } else {
                let first_32_bytes_eq = self.is_equal::<Bytes32Variable>(
                    current_node[0..32].into(),
                    current_node_id[0..32].into(),
                );
                let hash_eq = self.is_equal::<Bytes32Variable>(
                    current_node_hash,
                    current_node_id.as_slice()[0..32].into(),
                );
                let node_len_le_32 = self.lte(len_nodes[i], const_32);
                let case_len_le_32 = self.and(node_len_le_32, first_32_bytes_eq);
                let inter = self.not(node_len_le_32);
                let case_len_gt_32 = self.and(inter, hash_eq);
                let equality_fulfilled = self.or(case_len_le_32, case_len_gt_32);
                let checked_equality = self.or(equality_fulfilled, finished);
                let t = self._true();
                self.assert_is_equal(checked_equality, t);
            }

            let (decoded_list, decoded_element_lens, len_decoded_list) = self
                .decode_element_as_list::<ENCODING_LEN, LIST_LEN, ELEMENT_LEN>(
                    current_node,
                    len_nodes[i],
                    finished,
                );

            let is_branch = self.is_equal(len_decoded_list, branch_node_length);
            let is_leaf = self.is_equal(len_decoded_list, leaf_or_extension_node_length);
            let key_terminated = self.is_equal(current_key_idx, const_64);
            let path = decoded_list[0].as_slice().to_vec().to_nibbles(self);
            let prefix = path[0];
            let prefix_leaf_even = self.is_equal(prefix, prefix_leaf_even);
            let prefix_leaf_odd = self.is_equal(prefix, prefix_leaf_odd);
            let prefix_extension_even = self.is_equal(prefix, prefix_extension_even);
            let prefix_extension_odd = self.is_equal(prefix, prefix_extension_odd);

            let offset_even = self.mul(prefix_extension_even.variable, two);
            let offset_odd = self.mul(prefix_extension_odd.variable, one);
            let offset = self.add(offset_even, offset_odd);
            let branch_key = self.select_array(key_path.clone().as_slice(), current_key_idx);
            let branch_key_variable: Variable = self.byte_to_variable(branch_key); // can be unsafe since nibbles are checked

            // Case 1
            let is_branch_and_key_terminated = self.and(is_branch, key_terminated);
            let case_1_value = self.mul(is_branch_and_key_terminated.variable, tree_radix);
            let b = self.not(key_terminated);
            let is_branch_and_key_not_terminated = self.and(is_branch, b);
            let case_2_value = self.mul(
                is_branch_and_key_not_terminated.variable,
                branch_key_variable,
            );
            let case_3_value = self.mul(is_leaf.variable, one);

            let c = self.add(case_1_value, case_2_value);
            let updated_current_node_id_idx = self.add(c, case_3_value); // TODO: make this more concise

            let updated_current_node_id =
                self.select_array(decoded_list.as_slice(), updated_current_node_id_idx);
            // If finished == 1, then we should not update the current_node_id
            current_node_id = self.select_array(
                &[updated_current_node_id, current_node_id],
                finished.variable,
            );

            let mut do_path_remainder_check = self.not(finished);
            do_path_remainder_check = self.and(do_path_remainder_check, is_leaf);
            let d = self.or(prefix_extension_even, prefix_extension_odd);
            do_path_remainder_check = self.and(do_path_remainder_check, d);

            let e = self.mul(decoded_element_lens[0], two);
            let f = self.mul(offset, do_path_remainder_check.variable);
            let mut check_length = self.sub(e, f);
            check_length = self.mul(check_length, do_path_remainder_check.variable);

            self.assert_subarray_equal(
                &path,
                offset,
                key_path.as_slice(),
                current_key_idx,
                check_length,
            );

            current_key_idx = self.add(current_key_idx, is_branch_and_key_not_terminated.variable);
            let j = self.mul(is_leaf.variable, check_length);
            current_key_idx = self.add(current_key_idx, j);

            let prefix_leaf_even_and_leaf = self.and(prefix_leaf_even, is_leaf);
            let prefix_leaf_odd_and_leaf = self.and(prefix_leaf_odd, is_leaf);
            let l = self.or(is_branch_and_key_terminated, prefix_leaf_even_and_leaf);
            let m = self.or(l, prefix_leaf_odd_and_leaf);
            finished = self.or(finished, m);
        }

        let current_node_len = self.sub_byte(current_node_id[0], const_128);
        let current_node_len_as_var = self.byte_to_variable(current_node_len);
        let lhs_offset = self.sub(const_32, current_node_len_as_var);

        self.assert_subarray_equal(
            &value.as_bytes(),
            lhs_offset,
            current_node_id.as_slice(),
            one,
            current_node_len_as_var,
        );
    }
}

#[cfg(test)]
mod tests {
    use curta::math::field::Field;
    use log::debug;

    use super::super::utils::{read_fixture, EIP1186ProofResponse};
    use super::*;
    use crate::frontend::eth::utils::u256_to_h256_be;
    use crate::prelude::{DefaultBuilder, GoldilocksField};
    use crate::utils;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_mpt_circuit() {
        utils::setup_logger();
        type F = GoldilocksField;

        let storage_result: EIP1186ProofResponse =
            read_fixture("./src/frontend/eth/mpt/fixtures/example.json");

        let storage_proof = storage_result.storage_proof[0]
            .proof
            .iter()
            .map(|b| b.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let root = storage_result.storage_hash;
        let key = storage_result.storage_proof[0].key;
        let value = storage_result.storage_proof[0].value;

        debug!("root {:?} key {:?} value {:?}", root, key, value);

        let value_as_h256 = u256_to_h256_be(value);
        debug!("value_as_h256 {:?}", value_as_h256);

        const ENCODING_LEN: usize = 600;
        const PROOF_LEN: usize = 16;

        let (proof_as_fixed, lengths_as_fixed) =
            transform_proof_to_padded::<ENCODING_LEN, PROOF_LEN>(storage_proof);
        let len_nodes_field_elements = lengths_as_fixed
            .iter()
            .map(|x| F::from_canonical_usize(*x))
            .collect::<Vec<F>>();

        let mut builder = DefaultBuilder::new();
        let key_variable = builder.read::<Bytes32Variable>();
        let proof_variable =
            builder.read::<ArrayVariable<ArrayVariable<ByteVariable, ENCODING_LEN>, PROOF_LEN>>();
        let len_nodes = builder.read::<ArrayVariable<Variable, PROOF_LEN>>();
        let root_variable = builder.read::<Bytes32Variable>();
        let value_variable = builder.read::<Bytes32Variable>();
        builder.verify_mpt_proof::<ENCODING_LEN, PROOF_LEN>(
            key_variable,
            proof_variable.clone(),
            len_nodes.clone(),
            root_variable,
            value_variable,
        );
        let circuit = builder.mock_build();

        let mut input = circuit.input();
        input.write::<Bytes32Variable>(key);
        input.write::<ArrayVariable<ArrayVariable<ByteVariable, ENCODING_LEN>, PROOF_LEN>>(
            proof_as_fixed,
        );
        input.write::<ArrayVariable<Variable, PROOF_LEN>>(len_nodes_field_elements);
        input.write::<Bytes32Variable>(root);
        input.write::<Bytes32Variable>(value_as_h256);

        let (_witness, mut _output) = circuit.mock_prove(&input);
    }
}
