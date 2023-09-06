use std::marker::PhantomData;

use ethers::types::{Bytes, H256};
use ethers::utils::keccak256;
use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::*;
use super::rlc::subarray_equal;
use super::rlp::{decode_element_as_list, rlp_decode_bytes, rlp_decode_list_2_or_17};
use super::utils::*;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, Bytes32Variable, CircuitBuilder, Variable,
};
// use crate::utils::{address, bytes, bytes, bytes32, bytes32, hex, hex};

const TREE_RADIX: usize = 16;
const BRANCH_NODE_LENGTH: usize = 17;
const LEAF_OR_EXTENSION_NODE_LENGTH: usize = 2;
const PREFIX_EXTENSION_EVEN: usize = 0;
const PREFIX_EXTENSION_ODD: usize = 1;
const PREFIX_LEAF_EVEN: usize = 2;
const PREFIX_LEAF_ODD: usize = 3;

// Based off of the following Solidity implementation:
// https://github.com/ethereum-optimism/optimism/blob/6e041bcd9d678a0ea2bb92cfddf9716f8ae2336c/packages/contracts-bedrock/src/libraries/trie/MerkleTrie.sol
pub fn get(key: H256, proof: Vec<Vec<u8>>, root: H256) -> Vec<u8> {
    let mut current_key_index = 0;
    let mut current_node_id = root.to_fixed_bytes().to_vec();

    let hash_key = keccak256(key.to_fixed_bytes().as_slice());
    let _ = key; // Move key so that we cannot mistakely use it again
    let key_path = to_nibbles(&hash_key[..]);
    let mut finish = false;

    for i in 0..proof.len() {
        println!("i {}", i);
        let current_node = &proof[i];

        if current_key_index == 0 {
            let hash = keccak256(current_node);
            assert_bytes_equal(&hash[..], &current_node_id);
        } else if current_node.len() >= 32 {
            println!("current node length {:?}", current_node.len());
            let hash = keccak256(current_node);
            assert_bytes_equal(&hash[..], &current_node_id);
        } else {
            println!(
                "current_node {:?}",
                Bytes::from(current_node.to_vec()).to_string()
            );
            assert_bytes_equal(current_node, &current_node_id);
        }

        let decoded = rlp_decode_list_2_or_17(current_node);
        match decoded.len() {
            BRANCH_NODE_LENGTH => {
                if current_key_index == key_path.len() {
                    // We have traversed all nibbles of the key, so we return the value in the branch node
                    finish = true;
                    current_node_id = decoded[TREE_RADIX].clone();
                } else {
                    let branch_key = key_path[current_key_index];
                    current_node_id = decoded[usize::from(branch_key)].clone();
                    current_key_index += 1;
                }
            }
            LEAF_OR_EXTENSION_NODE_LENGTH => {
                current_node_id = decoded[1].clone().clone();
                let path = to_nibbles(&decoded[0]);
                let prefix = path[0];
                match usize::from(prefix) {
                    PREFIX_LEAF_EVEN | PREFIX_LEAF_ODD => {
                        // TODO there are some other checks here around length of the return value and also the path matching the key
                        finish = true;
                    }
                    PREFIX_EXTENSION_EVEN => {
                        // If prefix_extension_even, then the offset for the path is 2
                        let path_remainder = &path[2..];
                        assert_bytes_equal(
                            path_remainder,
                            &key_path[current_key_index..current_key_index + path_remainder.len()],
                        );
                        println!("path_remainder {:?}", path_remainder.len());
                        current_key_index += path_remainder.len();
                    }
                    PREFIX_EXTENSION_ODD => {
                        // If prefix_extension_odd, then the offset for the path is 1
                        let path_remainder = &path[1..];
                        assert_bytes_equal(
                            path_remainder,
                            &key_path[current_key_index..current_key_index + path_remainder.len()],
                        );
                        current_key_index += path_remainder.len();
                    }
                    _ => panic!("Invalid prefix for leaf or extension node"),
                }
            }
            _ => {
                panic!("Invalid decoded length");
            }
        }

        println!("decoded {:?}", decoded);
        println!("current_key_idx {:?}", current_key_index);
        println!("current node id at end of loop {:?}", current_node_id);

        if finish {
            println!("Finished");
            return rlp_decode_bytes(&current_node_id[..]).0;
        }
    }

    panic!("Invalid proof");
}

pub fn verified_get<const L: usize, const M: usize, const P: usize>(
    key: [u8; 32],
    proof: [[u8; M]; P],
    root: [u8; 32],
    value: [u8; 32],
    len_nodes: [u32; P],
) {
    const MAX_NODE_SIZE: usize = 34;

    let mut current_key_idx: u32 = 0;
    let mut current_node_id = [0u8; MAX_NODE_SIZE];
    for i in 0..32 {
        current_node_id[i] = root[i];
    }
    let hash_key = keccak256(key);
    let key_path = to_nibbles(&hash_key);
    let mut finish: u32 = 0;

    for i in 0..P {
        println!("i: {}", i);
        let current_node = proof[i];
        let current_node_hash = keccack_variable(current_node, len_nodes[i]);
        println!(
            "current_node_hash {:?}",
            H256::from_slice(&current_node_hash)
        );
        if i == 0 {
            let is_eq = is_bytes32_eq(current_node_hash, root);
            assert!(is_eq == 1);
        } else {
            let first_32_byte_eq = is_bytes32_eq(
                current_node[0..32].try_into().unwrap(),
                current_node_id[0..32].try_into().unwrap(),
            );
            // println!("first_32_byte_eq: {}", first_32_byte_eq);
            let hash_eq = is_bytes32_eq(
                current_node_hash,
                current_node_id[0..32].try_into().unwrap(),
            );
            // println!("hash_eq: {}", hash_eq);
            // println!("{:?} {:?}", current_node_hash, current_node_id);
            let equality_fulfilled = is_leq(len_nodes[i], 32) * first_32_byte_eq as u32
                + (1 - is_leq(len_nodes[i], 32)) * hash_eq as u32;
            // assert equality == 1 OR finish == 1
            assert!((equality_fulfilled as i32 - 1) * (1 - finish as i32) == 0);
        }
        println!("Round {} current_node {:?}", i, current_node);
        println!("Round {} len_nodes[i] {:?}", i, len_nodes[i]);

        let finish_bool = finish == 1;
        let (decoded, decoded_lens, witness_list_len) = decode_element_as_list::<M, L, MAX_NODE_SIZE>(
            &current_node,
            len_nodes[i] as usize,
            finish_bool,
        );
        // TODO: verify_decoded_list(witness_decoded_list, witness_decoded_lens, current_node, witness_list_len, len_nodes[i]);
        println!("Round {} decoded_list_len {:?}", i, witness_list_len);
        println!("Round {} decoded_element_lens {:?}", i, decoded_lens);

        let is_branch = is_eq(witness_list_len as u8, BRANCH_NODE_LENGTH);
        let is_leaf = is_eq(witness_list_len as u8, LEAF_OR_EXTENSION_NODE_LENGTH);
        let key_terminated = is_eq(current_key_idx as u8, 64);
        let path = to_nibbles(&decoded[0]);
        let prefix = path[0];
        let prefix_leaf_even = is_eq(prefix, PREFIX_LEAF_EVEN);
        let prefix_leaf_odd = is_eq(prefix, PREFIX_LEAF_ODD);
        let prefix_extension_even = is_eq(prefix, PREFIX_EXTENSION_EVEN);
        let prefix_extension_odd = is_eq(prefix, PREFIX_EXTENSION_ODD);
        let offset = 2 * (prefix_extension_even as u32) + 1 * prefix_extension_odd as u32;

        let branch_key = mux(&key_path, current_key_idx as u8);
        if (1 - finish) * is_branch * key_terminated == 1 {
            current_node_id = decoded[TREE_RADIX].clone().try_into().unwrap();
        } else if (1 - finish) * is_branch * (1 - key_terminated) == 1 {
            current_node_id = mux_nested(decoded.clone(), branch_key);
        } else if (1 - finish) * is_leaf == 1 {
            current_node_id = decoded[1].clone().try_into().unwrap();
        } else {
            // If finish = 1, all bets are off
            if finish == 0 {
                panic!("Should not happen")
            }
        }

        println!("decoded {:?}", decoded.clone());
        // The reason that we multiply decoded_lens[i] * 2 is because path and key path are both in nibbles

        // Only do the path remainder check if not finished AND is_leaf AND OR(prefix_extension_even, prefix_extension_odd)
        let do_path_remainder_check = (1 - finish)
            * is_leaf
            * (1 - prefix_leaf_even)
            * (1 - prefix_leaf_odd)
            * (prefix_extension_even + prefix_extension_odd
                - prefix_extension_even * prefix_extension_odd);
        let check_length = do_path_remainder_check
            * (decoded_lens[0] as u32 * 2 - offset * do_path_remainder_check);

        subarray_equal(
            &path,
            offset as usize,
            &key_path,
            current_key_idx as usize,
            check_length as usize,
        );

        println!("is_leaf {}", is_leaf);
        println!("decoded_lens[0] {} offset {}", decoded_lens[0], offset);
        current_key_idx += is_branch * (1 - key_terminated) * 1 + is_leaf * check_length;

        // update finish
        if finish == 0 {
            // Can only toggle finish if it's false
            println!("finished {}", finish);
            finish =
                is_branch * key_terminated + is_leaf * prefix_leaf_even + is_leaf * prefix_leaf_odd;
        }
        // TODO other checks around the paths matching
        println!("current key idx {:?}", current_key_idx);
        println!("current node id at end of loop {:?}", current_node_id);
    }

    // At the end, assert that
    // current_node_id = rlp(value)
    println!("current_node_id {:?}", current_node_id);
    println!("value {:?}", value);

    let current_node_len = current_node_id[0] - 0x80;
    subarray_equal(
        &value,
        32 - current_node_len as usize,
        &current_node_id,
        1,
        current_node_len as usize,
    );
}

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

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn le(&mut self, lhs: Variable, rhs: Variable) -> BoolVariable {
        let generator = LeGenerator {
            lhs: lhs.clone(),
            rhs: rhs.clone(),
            output: self.init::<BoolVariable>(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(&generator);
        generator.output
    }

    pub fn byte_to_variable(&mut self, lhs: ByteVariable) -> Variable {
        let generator = ByteToVariableGenerator {
            lhs: lhs.clone(),
            output: self.init::<Variable>(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(&generator);
        generator.output
    }

    pub fn sub_byte(&mut self, lhs: ByteVariable, rhs: ByteVariable) -> ByteVariable {
        let generator = ByteSubGenerator {
            lhs: lhs.clone(),
            rhs: rhs.clone(),
            output: self.init::<ByteVariable>(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(&generator);
        generator.output
    }

    #[allow(dead_code, unused_variables)]
    pub fn to_nibbles(&mut self, bytes: &[ByteVariable]) -> Vec<ByteVariable> {
        let len = bytes.len() * 2;
        let generator = NibbleGenerator {
            input: bytes.to_vec(),
            output: (0..len).map(|_| self.init::<ByteVariable>()).collect_vec(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(&generator);
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

        let tree_radix = self.constant::<Variable>(F::from_canonical_u8(16u8));
        let branch_node_length = self.constant::<Variable>(F::from_canonical_u8(17u8));
        let leaf_or_extension_node_length = self.constant::<Variable>(F::from_canonical_u8(2u8));
        let prefix_leaf_even = self.constant::<ByteVariable>(Self::PREFIX_LEAF_EVEN);
        let prefix_leaf_odd = self.constant::<ByteVariable>(Self::PREFIX_LEAF_ODD);
        let prefix_extension_even = self.constant::<ByteVariable>(Self::PREFIX_EXTENSION_EVEN);
        let prefix_extension_odd = self.constant::<ByteVariable>(Self::PREFIX_EXTENSION_ODD);
        let one: Variable = self.one::<Variable>();
        let two = self.constant::<Variable>(F::from_canonical_u8(2));
        let _64 = self.constant::<Variable>(F::from_canonical_u8(64));
        let _32 = self.constant::<Variable>(F::from_canonical_u8(32));
        let _128 = self.constant::<ByteVariable>(128);

        let mut current_key_idx = self.zero::<Variable>();
        let mut finished = self._false();

        let mut padded_root = root.as_slice().to_vec();
        while padded_root.len() < ELEMENT_LEN {
            padded_root.push(self.constant::<ByteVariable>(0));
        }
        let mut current_node_id = ArrayVariable::<ByteVariable, ELEMENT_LEN>::new(padded_root);
        let hash_key = self.keccak256(&key.as_slice());
        let key_path: ArrayVariable<ByteVariable, 64> =
            self.to_nibbles(&hash_key.as_slice()).try_into().unwrap();

        for i in 0..PROOF_LEN {
            let current_node = proof[i].clone();
            let current_node_hash = self.keccak256_variable(&current_node.as_slice(), len_nodes[i]);

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
                let a = self.constant::<Variable>(F::from_canonical_u8(32u8));
                let node_len_le_32 = self.le(len_nodes[i], a);
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
            let key_terminated = self.is_equal(current_key_idx, _64);
            let path = self.to_nibbles(&decoded_list[0].as_slice());
            let prefix = path[0];
            let prefix_leaf_even = self.is_equal(prefix, prefix_leaf_even);
            let prefix_leaf_odd = self.is_equal(prefix, prefix_leaf_odd);
            let prefix_extension_even = self.is_equal(prefix, prefix_extension_even);
            let prefix_extension_odd = self.is_equal(prefix, prefix_extension_odd);

            let offset_even = self.mul(prefix_extension_even.0, two);
            let offset_odd = self.mul(prefix_extension_odd.0, one);
            let offset = self.add(offset_even, offset_odd);
            let branch_key = self.mux(key_path.clone(), current_key_idx);
            let branch_key_variable: Variable = self.byte_to_variable(branch_key); // can be unsafe since nibbles are checked

            // Case 1
            let is_branch_and_key_terminated = self.and(is_branch, key_terminated);
            let case_1_value = self.mul(is_branch_and_key_terminated.0, tree_radix);
            let b = self.not(key_terminated);
            let is_branch_and_key_not_terminated = self.and(is_branch, b);
            let case_2_value = self.mul(is_branch_and_key_not_terminated.0, branch_key_variable);
            let case_3_value = self.mul(is_leaf.0, one);

            let c = self.add(case_1_value, case_2_value);
            let updated_current_node_id_idx = self.add(c, case_3_value); // TODO: make this more concise

            let updated_current_node_id = self.mux(decoded_list, updated_current_node_id_idx);
            // If finished == 1, then we should not update the current_node_id
            current_node_id = self.mux::<_, 2>(
                vec![updated_current_node_id, current_node_id].into(),
                finished.0,
            );

            let mut do_path_remainder_check = self.not(finished);
            do_path_remainder_check = self.and(do_path_remainder_check, is_leaf);
            let d = self.or(prefix_extension_even, prefix_extension_odd);
            do_path_remainder_check = self.and(do_path_remainder_check, d);

            let e = self.mul(decoded_element_lens[0], two);
            let f = self.mul(offset, do_path_remainder_check.0);
            let mut check_length = self.sub(e, f);
            check_length = self.mul(check_length, do_path_remainder_check.0);

            self.assert_subarray_equal(
                &path,
                offset,
                &key_path.as_slice(),
                current_key_idx,
                check_length,
            );

            current_key_idx = self.add(current_key_idx, is_branch_and_key_not_terminated.0);
            let j = self.mul(is_leaf.0, check_length);
            current_key_idx = self.add(current_key_idx, j);

            let prefix_leaf_even_and_leaf = self.and(prefix_leaf_even, is_leaf);
            let prefix_leaf_odd_and_leaf = self.and(prefix_leaf_odd, is_leaf);
            let l = self.or(is_branch_and_key_terminated, prefix_leaf_even_and_leaf);
            let m = self.or(l, prefix_leaf_odd_and_leaf);
            finished = self.or(finished, m);
        }

        let current_node_len = self.sub_byte(current_node_id[0], _128);
        let current_node_len_as_var = self.byte_to_variable(current_node_len);
        let lhs_offset = self.sub(_32, current_node_len_as_var);

        self.assert_subarray_equal(
            &value.as_slice(),
            lhs_offset,
            current_node_id.as_slice(),
            one,
            current_node_len_as_var,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use ethers::providers::{Http, Middleware, Provider};
    use ethers::types::{Bytes, EIP1186ProofResponse};
    use ethers::utils::keccak256;
    use plonky2::field::types::Field;
    use plonky2::iop::generator::generate_partial_witness;
    use tokio::runtime::Runtime;

    use super::*;
    use crate::frontend::eth::utils::u256_to_h256_be;
    use crate::prelude::{
        CircuitBuilderX, CircuitVariable, GoldilocksField, PartialWitness, PoseidonGoldilocksConfig,
    };
    use crate::utils::{address, bytes32};

    fn generate_fixtures() {
        // TODO: don't have mainnet RPC url here, read from a .env
        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let block_number = 17880427u64;
        let _state_root =
            bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
        let address = address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5");
        let location =
            bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");

        // Nouns contract
        // let address = address!("0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03");
        // let location = bytes32!("0x0000000000000000000000000000000000000000000000000000000000000003");

        let get_proof_closure = || -> EIP1186ProofResponse {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                provider
                    .get_proof(address, vec![location], Some(block_number.into()))
                    .await
                    .unwrap()
            })
        };
        let storage_result: EIP1186ProofResponse = get_proof_closure();
        let serialized = serde_json::to_string(&storage_result).unwrap();
        println!("{}", serialized);
        // TODO: save this to fixtures/example.json programatically instead of copy-paste
    }

    fn read_fixture(filename: &str) -> EIP1186ProofResponse {
        let mut file = File::open(filename).unwrap();
        let mut context = String::new();
        file.read_to_string(&mut context).unwrap();

        let context: EIP1186ProofResponse = serde_json::from_str(context.as_str()).unwrap();
        context
    }

    #[test]
    fn test_mpt_vanilla() {
        let storage_result: EIP1186ProofResponse =
            read_fixture("./src/eth/mpt/fixtures/example.json");

        let proof = storage_result.storage_proof[0]
            .proof
            .iter()
            .map(|b| b.to_vec())
            .collect::<Vec<Vec<u8>>>();
        println!(
            "Storage proof first element {:?}",
            storage_result.storage_proof[0].proof[0].to_string()
        );
        let k = keccak256::<Vec<u8>>(storage_result.storage_proof[0].proof[0].to_vec()).to_vec();
        println!(
            "keccack256 of first element {:?}",
            Bytes::from(k).to_string()
        );
        println!("storage hash {:?}", storage_result.storage_hash.to_string());
        let value = get(
            storage_result.storage_proof[0].key,
            proof,
            storage_result.storage_hash,
        );
        println!("recovered value {:?}", Bytes::from(value).to_string());
        // TODO have to left pad the recovered value to 32 bytes
        // println!("recovered value h256 {:?}", H256::from_slice(&value));
        println!(
            "true value {:?}",
            u256_to_h256_be(storage_result.storage_proof[0].value)
        );
        // TODO: make this a real test with assert

        // TODO: for some reason this doesn't work...not sure why
        // let account_key = keccak256(address.as_bytes());
        // let account_proof = storage_result.account_proof.iter().map(|b| b.to_vec()).collect::<Vec<Vec<u8>>>();
        // let account_value = get(account_key.into(), account_proof, state_root);
        // println!("account value {:?}", Bytes::from(account_value).to_string());
    }

    #[test]
    fn test_mpt_circuit() {
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

        println!("root {:?} key {:?} value {:?}", root, key, value);

        let value_as_h256 = u256_to_h256_be(value);
        println!("value_as_h256 {:?}", value_as_h256);

        const ENCODING_LEN: usize = 600;
        const PROOF_LEN: usize = 16;

        let (proof_as_fixed, lengths_as_fixed) =
            transform_proof_to_padded::<ENCODING_LEN, PROOF_LEN>(storage_proof);

        type F = GoldilocksField;
        let mut builder: CircuitBuilder<GoldilocksField, 2> = CircuitBuilderX::new();
        // builder.debug(77867);
        let key_variable = builder.init::<Bytes32Variable>();
        let proof_variable =
            builder.init::<ArrayVariable<ArrayVariable<ByteVariable, ENCODING_LEN>, PROOF_LEN>>();
        let len_nodes = builder.init::<ArrayVariable<Variable, PROOF_LEN>>();
        let root_variable = builder.init::<Bytes32Variable>();
        let value_variable = builder.init::<Bytes32Variable>();
        builder.verify_mpt_proof::<ENCODING_LEN, PROOF_LEN>(
            key_variable,
            proof_variable.clone(),
            len_nodes.clone(),
            root_variable,
            value_variable,
        );
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        let mut partial_witness: PartialWitness<GoldilocksField> = PartialWitness::new();
        key_variable.set(&mut partial_witness, key);
        root_variable.set(&mut partial_witness, root);
        value_variable.set(&mut partial_witness, value_as_h256);
        proof_variable.set(&mut partial_witness, proof_as_fixed);

        // TODO: make this a macro instead of .map().iter()
        len_nodes.set(
            &mut partial_witness,
            lengths_as_fixed
                .iter()
                .map(|x| F::from_canonical_usize(*x))
                .collect::<Vec<F>>(),
        );

        // This is to generate the witness only without generating the proof
        // TODO: turn this into a nice method on `Circuit`
        // let prover_data = circuit.data.prover_only;
        // let common_data = circuit.data.common;
        // let witness = generate_partial_witness(partial_witness, &prover_data, &common_data);

        let proof = circuit.data.prove(partial_witness).unwrap();
        circuit.data.verify(proof.clone()).unwrap();
    }
}
