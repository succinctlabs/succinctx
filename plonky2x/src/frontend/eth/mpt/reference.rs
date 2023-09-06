use ethers::types::{Bytes, H256};
use ethers::utils::keccak256;

use crate::frontend::eth::rlp::rlp::{rlp_decode_bytes, rlp_decode_list_2_or_17};

const TREE_RADIX: usize = 16;
const BRANCH_NODE_LENGTH: usize = 17;
const LEAF_OR_EXTENSION_NODE_LENGTH: usize = 2;
const PREFIX_EXTENSION_EVEN: usize = 0;
const PREFIX_EXTENSION_ODD: usize = 1;
const PREFIX_LEAF_EVEN: usize = 2;
const PREFIX_LEAF_ODD: usize = 3;

pub fn to_nibbles(data: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(data.len() * 2);
    for byte in data {
        // High nibble (upper 4 bits)
        nibbles.push(byte >> 4);
        // Low nibble (lower 4 bits)
        nibbles.push(byte & 0x0F);
    }
    nibbles
}

pub fn assert_bytes_equal(a: &[u8], b: &[u8]) {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        assert!(a[i] == b[i]);
    }
}

// Based off of the following Solidity implementation:
// https://github.com/ethereum-optimism/optimism/blob/6e041bcd9d678a0ea2bb92cfddf9716f8ae2336c/packages/contracts-bedrock/src/libraries/trie/MerkleTrie.sol
pub(crate) fn get(key: H256, proof: Vec<Vec<u8>>, root: H256) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::super::utils::{read_fixture, EIP1186ProofResponse};
    use super::{get, *};
    use crate::frontend::eth::utils::u256_to_h256_be;

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
}
