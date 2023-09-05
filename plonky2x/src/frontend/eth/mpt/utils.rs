use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, Bytes, EIP1186ProofResponse, H256};
use ethers::utils::keccak256;
use num_bigint::{BigInt, RandBigInt, ToBigInt};

use crate::utils::{address, bytes, bytes32, hex};

const TREE_RADIX: usize = 16;
const BRANCH_NODE_LENGTH: usize = 17;
const LEAF_OR_EXTENSION_NODE_LENGTH: usize = 2;
const PREFIX_EXTENSION_EVEN: usize = 0;
const PREFIX_EXTENSION_ODD: usize = 1;
const PREFIX_LEAF_EVEN: usize = 2;
const PREFIX_LEAF_ODD: usize = 3;

pub fn assert_bytes_equal(a: &[u8], b: &[u8]) {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        assert!(a[i] == b[i]);
    }
}

pub fn print_vec(a: &[u8]) -> String {
    let bytes = Bytes::from_iter(a.iter());
    bytes.to_string()
}

pub fn print_vecs(a: Vec<Vec<u8>>) {
    for i in 0..a.len() {
        println!("{} {:?}", i, print_vec(&a[i]));
    }
}

fn to_nibbles(data: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(data.len() * 2);
    for byte in data {
        // High nibble (upper 4 bits)
        nibbles.push(byte >> 4);
        // Low nibble (lower 4 bits)
        nibbles.push(byte & 0x0F);
    }
    nibbles
}

pub fn rlp_decode_list_2_or_17(input: &[u8]) -> Vec<Vec<u8>> {
    let prefix = input[0];
    // println!("input {:?}", Bytes::from(input.to_vec()).to_string());
    if prefix <= 0xF7 {
        // Short list (0-55 bytes total payload)
        let list_length = (prefix - 0xC0) as usize;
        // We assert that the input is simply [list_length, list_content...] and not suffixed by anything else
        assert!(input.len() == 1 + list_length);
        let (ele_1, increment) = rlp_decode_bytes(&input[1..]);
        let (ele_2, _) = rlp_decode_bytes(&input[1 + increment..]);
        return vec![ele_1, ele_2];
    } else {
        // TODO check that prefix is bounded within a certain range
        let len_of_list_length = prefix - 0xF7;
        // println!("len_of_list_length {:?}", len_of_list_length);
        // TODO: figure out what to do with len_of_list_length
        let mut pos = 1 + len_of_list_length as usize;
        let mut res = vec![];
        for _ in 0..17 {
            let (ele, increment) = rlp_decode_bytes(&input[pos..]);
            res.push(ele);
            pos += increment;
            // println!("ele {:?}", Bytes::from(ele.clone()).to_string());
            // println!("increment {:?}", increment);
            if pos == input.len() {
                break;
            }
        }
        assert!(pos == input.len()); // Check that we have iterated through all the input
        assert!(res.len() == 17 || res.len() == 2);
        return res;
    }
}

// Note this only decodes bytes and doesn't support long strings
fn rlp_decode_bytes(input: &[u8]) -> (Vec<u8>, usize) {
    let prefix = input[0];
    if prefix <= 0x7F {
        return (vec![prefix], 1);
    } else if prefix == 0x80 {
        return (vec![], 1); // null value
    } else if prefix <= 0xB7 {
        // Short string (0-55 bytes length)
        let length = (prefix - 0x80) as usize;
        let res = &input[1..1 + length];
        return (res.into(), 1 + length);
    } else if prefix <= 0xBF {
        panic!("Long string (56+ bytes length) not supported in rlp_decode_bytes")
    } else {
        panic!("Invalid prefix rlp_decode_bytes")
    }
}
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

#[cfg(test)]
mod tests {
    use core::ops::Add;

    use anyhow::Result;
    use ethers::prelude::k256::elliptic_curve::rand_core::block;
    use ethers::types::{Address, Bytes, EIP1186ProofResponse, H256, U256};
    use ethers::utils::keccak256;
    use ethers::utils::rlp::RlpStream;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use subtle_encoding::hex::decode;
    use tokio::runtime::Runtime;

    use super::*;
    use crate::eth::storage;
    use crate::eth::utils::{h256_to_u256_be, u256_to_h256_be};

    #[test]
    fn test_rlp_vanilla() {
        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let block_number = 17880427u64;
        let state_root =
            bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
        let address = address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5");
        let location =
            bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");

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

        let proof = storage_result.storage_proof[0]
            .proof
            .iter()
            .map(|b| b.to_vec())
            .collect::<Vec<Vec<u8>>>();
        // println!("{:?}", storage_result.storage_proof[0].proof[0]);
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

        // TODO: for some reason this doesn't work...not sure why
        // let account_key = keccak256(address.as_bytes());
        // let account_proof = storage_result.account_proof.iter().map(|b| b.to_vec()).collect::<Vec<Vec<u8>>>();
        // let account_value = get(account_key.into(), account_proof, state_root);
        // println!("account value {:?}", Bytes::from(account_value).to_string());
    }
}
