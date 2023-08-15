use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, EIP1186ProofResponse, H256};
use ethers::types::{Bytes};
use ethers::utils::keccak256;

const TREE_RADIX: usize = 16;
const BRANCH_NODE_LENGTH: usize = 17;
const LEAF_OR_EXTENSION_NODE_LENGTH: usize = 2;
const PREFIX_EXTENSION_EVEN: usize = 0;
const PREFIX_EXTENSION_ODD: usize = 1;
const PREFIX_LEAF_EVEN: usize = 2;
const PREFIX_LEAF_ODD: usize = 3;

pub fn assert_bytes_equal(a: &Vec<u8>, b: &Vec<u8>) {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        assert!(a[i] == b[i]);
    }
}

pub fn print_vec(a: &[u8]) -> String {
    let bytes = Bytes::from_iter(a.iter());
    bytes.to_string()
}

pub fn print_vecs(a: Vec<Vec<u8>>)  {
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

fn decode_rlp_list(input: &[u8]) -> Vec<Vec<u8>> {
    // either claimed_length = 17 or 2
    let prefix = input[0];
    if prefix <= 0xF7 {
        // Short list (0-55 bytes total payload)
        let list_length = (prefix - 0xC0) as usize;
        // In our case, it's just a list, not suffixed by anything else
        assert!(input.len() == 1 + list_length);
        // assert!(list_length == 2);
        let first_ele = decode_bytes(&input[1..]).to_vec();
        let mut increment = first_ele.len(); // 1 + to denote cutting of first byte that has list_length
        if increment > 1 || increment == 0 {
            increment += 1;
        }
        println!("first ele {:?}", first_ele);
        println!("increment");
        let second_ele = decode_bytes(&input[1 + increment..]).to_vec();
        return vec![first_ele, second_ele];
    } else {
        // NOTE: these should both be constants below
        let len_of_list_length = prefix - 0xF7;
        // assert!(len_of_list_length == 2);
        let first_byte = input[1]; // TODO: make sure this isn't 0x00
        let list_length = input[2] as usize;
        // assert!(list_length == 17, "list length should be 17 but len_of_list_length {} {} {:?}", len_of_list_length, first_byte, list_length);
        // TODO: this is weird, but it works
        let mut pos = 1 + len_of_list_length as usize;
        let mut res = vec![];
        for i in 0..17 {
            let ele = decode_bytes(&input[pos..]).to_vec();
            let mut increment = ele.len();
            if increment > 1 || increment == 0 {
                increment += 1;
            }
            res.push(ele);
            pos += increment;
        }
        return res;
    } 
}

fn decode_bytes(input: &[u8]) -> &[u8] {
    let prefix = input[0];
    if prefix <= 0x7F {
        return &input[0..1];
    } else if prefix == 0x80 {
        return &[]; // null value
    }  else if prefix <= 0xB7 {
        // Short string (0-55 bytes length)
        let length = (prefix - 0x80) as usize;
        let res = &input[1..1+length];
        return res;
    } else if prefix <= 0xBF {
        panic!("Long string (56+ bytes length) not supported")
    } else {
        panic!("Prefix not fitting for decode bytes")
    }
}

// https://github.com/ethereum-optimism/optimism/blob/6e041bcd9d678a0ea2bb92cfddf9716f8ae2336c/packages/contracts-bedrock/src/libraries/trie/MerkleTrie.sol
pub fn get(key: H256, proof: Vec<Vec<u8>>, root: H256) -> Vec<u8> {
    // Get the value associated with key from root with proof
    let mut current_key_index = 0;
    let mut current_node_id = root.to_fixed_bytes().to_vec();

    let hash_key = keccak256(key.to_fixed_bytes().as_slice());
    let key_path = to_nibbles(&hash_key[..]);

    for i in 0..proof.len() {
        println!("Processing proof element {}", i);
        println!("current_key_idx {}", i);
        let current_node = proof[i].to_vec();
        println!("Current node {:?}", print_vec(&current_node));

        if current_key_index == 0 {
            let hash = keccak256(current_node.clone());
            assert_bytes_equal(&hash.to_vec(), &current_node_id);
        } else if current_node.len() >= 32 {
            let hash = keccak256(current_node.clone());
            assert_bytes_equal(&hash.to_vec(), &current_node_id.clone());
        } else {
            assert_bytes_equal(&current_node, &current_node_id);
        }

        // Either current_node is "NULL"
        // OR branch of length 17
        // OR leaf of length 2
        let decoded = decode_rlp_list(current_node.clone().as_slice()).to_vec();
        // println!("decoded");
        // print_vecs(decoded.clone());
        if decoded.len() == BRANCH_NODE_LENGTH {
            if current_key_index == key_path.len() {
                // We have reached the end
                return decode_bytes(&decoded[TREE_RADIX]).to_vec();
            } else {
                let branch_key = key_path[current_key_index];
                println!("Branch key {}", branch_key);
                current_node_id = decoded[usize::from(branch_key)].clone();
                current_key_index += 1;
            }
        } else if decoded.len() == LEAF_OR_EXTENSION_NODE_LENGTH {
            let path = to_nibbles(&decoded[0]);
            let prefix = path[0];
            if usize::from(prefix) == PREFIX_LEAF_EVEN || usize::from(prefix) == PREFIX_LEAF_ODD {
                // TODO another require in here
                return decode_bytes(&decoded[1].clone()).to_vec();
            } else if usize::from(prefix) == PREFIX_EXTENSION_EVEN {
                current_node_id = decoded[1].clone();
                // If even, this means the offset is 2
                let path_remainder = path[2..].to_vec();
                assert_bytes_equal(&path_remainder, &key_path[current_key_index..current_key_index+path_remainder.len()].to_vec());
                current_key_index += path_remainder.len();
            } else if usize::from(prefix) == PREFIX_EXTENSION_ODD {
                current_node_id = decoded[1].clone();
                // offset = 1
                let path_remainder = path[1..].to_vec();
                assert_bytes_equal(&path_remainder, &key_path[current_key_index..current_key_index+path_remainder.len()].to_vec());
                current_key_index += path_remainder.len();
            } else {
                panic!("Invalid prefix");
            }
        }
    }
    panic!("Invalid proof");
}


#[cfg(test)]
mod tests {
    use core::ops::Add;

    use anyhow::Result;
    use ethers::types::{Bytes};
    use ethers::prelude::k256::elliptic_curve::rand_core::block;
    use ethers::types::{Address, H256, U256, EIP1186ProofResponse};
    use ethers::utils::keccak256;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use subtle_encoding::hex::decode;
    use ethers::utils::rlp::{RlpStream};
    use tokio::runtime::Runtime;

    use crate::eth::storage;
    use crate::eth::utils::{u256_to_h256_be, h256_to_u256_be};

    use super::*;

    #[test]
    fn test_rlp_vanilla() {
        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let block_number = 17880427u64;
        let state_root =
            "0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e"
                .parse::<H256>()
                .unwrap();

        let address = "0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5"
            .parse::<Address>()
            .unwrap();

        let location =
            "0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
                .parse::<H256>()
                .unwrap();

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

        let proof = storage_result.storage_proof[0].proof.iter().map(|b| b.to_vec()).collect::<Vec<Vec<u8>>>();
        // println!("{:?}", storage_result.storage_proof[0].proof[0]);
        println!("Storage proof first element {:?}", storage_result.storage_proof[0].proof[0].to_string());
        let k = keccak256::<Vec<u8>>(storage_result.storage_proof[0].proof[0].to_vec()).to_vec();
        println!("keccack256 of first element {:?}", Bytes::from(k).to_string());
        println!("storage hash {:?}", storage_result.storage_hash.to_string());
        let value = get(storage_result.storage_proof[0].key, proof, storage_result.storage_hash);
        println!("recovered value {:?}", Bytes::from(value).to_string());
        // TODO have to left pad the recovered value to 32 bytes
        // println!("recovered value h256 {:?}", H256::from_slice(&value));
        println!("true value {:?}", u256_to_h256_be(storage_result.storage_proof[0].value));

    }
}
