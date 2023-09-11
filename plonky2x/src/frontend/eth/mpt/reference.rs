use ethers::types::H256;
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
#[allow(dead_code)] // We allow dead_code since it's used in the tests below
pub(crate) fn get(key: H256, proof: Vec<Vec<u8>>, root: H256, account_proof: bool) -> Vec<u8> {
    let mut current_key_index = 0;
    let mut current_node_id = root.to_fixed_bytes().to_vec();

    let hash_key = key.to_fixed_bytes();
    let _ = key; // Move key so that we cannot mistakely use it again
    let key_path = to_nibbles(&hash_key[..]);
    let mut finish = false;

    for i in 0..proof.len() {
        let current_node = &proof[i];

        if current_key_index == 0 {
            let hash = keccak256(current_node);
            assert_bytes_equal(&hash[..], &current_node_id);
        } else if current_node.len() >= 32 {
            let hash = keccak256(current_node);
            assert_bytes_equal(&hash[..], &current_node_id);
        } else {
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

        if finish {
            if account_proof {
                return current_node_id;
            } else {
                return rlp_decode_bytes(&current_node_id[..]).0;
            }
        }
    }

    panic!("Invalid proof");
}

#[cfg(test)]
mod tests {
    use super::super::utils::{read_fixture, EIP1186ProofResponse};
    use super::{get, *};
    use crate::frontend::eth::utils::u256_to_h256_be;
    use crate::utils::bytes32;

    #[test]
    fn test_mpt_storage_proof() {
        let storage_result: EIP1186ProofResponse =
            read_fixture("./src/frontend/eth/mpt/fixtures/example.json");

        let proof = storage_result.storage_proof[0]
            .proof
            .iter()
            .map(|b| b.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let storage_key = keccak256(storage_result.storage_proof[0].key.as_bytes());
        let mut value = get(
            storage_key.into(),
            proof,
            storage_result.storage_hash,
            false,
        );
        // Left pad the recovered value to 32 bytes
        value.splice(0..0, vec![0; 32 - value.len()]);

        let recovered_value_h256 = H256::from_slice(&value);
        let true_value_h256 = u256_to_h256_be(storage_result.storage_proof[0].value);
        assert_eq!(recovered_value_h256, true_value_h256);
    }

    #[test]
    fn test_mpt_account_proof() {
        let storage_result: EIP1186ProofResponse =
            read_fixture("./src/frontend/eth/mpt/fixtures/example.json");

        // TODO: put this state root in the fixture
        let state_root =
            bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
        let address = storage_result.address;
        let account_key = keccak256(address.as_bytes());

        let account_proof = storage_result
            .account_proof
            .iter()
            .map(|b| b.to_vec())
            .collect::<Vec<Vec<u8>>>();

        let account_value = get(account_key.into(), account_proof, state_root, true);

        println!("account value {:?}", Bytes::from(account_value).to_string());
        println!("account nonce {:?}", storage_result.nonce);
        println!("account balance {:?}", storage_result.balance);
        println!("account storage_hash {:?}", storage_result.storage_hash);
        println!("account code_hash {:?}", storage_result.code_hash);

        // TODO:
        // assert that account_value =
        //    0xAA 0xBB || rlp_encode_byte(nonce) || rlp_encode_byte(balance)
        //         || 0xa0 || storage_hash || 0xa0 || code_hash
    }
}
