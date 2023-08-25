use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, EIP1186ProofResponse, H256};
use ethers::types::{Bytes};
use ethers::utils::keccak256;
use crate::utils::{bytes32, address, bytes, hex};
use num_bigint::{ToBigInt, RandBigInt, BigInt};

use crate::eth::mpt::utils::rlp_decode_list_2_or_17;

const TREE_RADIX: usize = 16;
const BRANCH_NODE_LENGTH: usize = 17;
const LEAF_OR_EXTENSION_NODE_LENGTH: usize = 2;
const PREFIX_EXTENSION_EVEN: usize = 0;
const PREFIX_EXTENSION_ODD: usize = 1;
const PREFIX_LEAF_EVEN: usize = 2;
const PREFIX_LEAF_ODD: usize = 3;


// This is simply for getting witness, we return the decoded list, the lengths of the elements in the decoded list and also the list length
pub fn witness_decoding<const M: usize, const L:usize>(encoded: [u8; M], len: u32, finish: u32) -> ([[u8; 34]; L], [u8; L], u8) {
    let mut decoded_list_as_fixed = [[0u8; 34]; L];
    let mut decoded_list_lens = [0u8; L];
    let mut decoded_list_len = 0;
    if finish == 1 { // terminate early
        return (decoded_list_as_fixed, decoded_list_lens, decoded_list_len);
    }
    let decoded_element = rlp_decode_list_2_or_17(&encoded[..len as usize]);
    for (i, element) in decoded_element.iter().enumerate() {
        let len: usize = element.len();
        assert!(len <= 34, "The decoded element should have length <= 34!");
        decoded_list_as_fixed[i][..len].copy_from_slice(&element);
        decoded_list_lens[i] = len as u8;
    }
    return (decoded_list_as_fixed, decoded_list_lens, decoded_element.len() as u8);
}

// Below everything would be implemented as constraints on the builder

pub fn is_leq(x: u32, y: u32) -> u32 {
    if x <= y {
        return 1;
    } else {
        return 0;
    }
}

pub fn is_le(x: u32, y: u32) -> u32 {
    if x < y {
        return 1;
    } else {
        return 0;
    }
}

fn parse_list_element(element: [u8; 32], len: u8) -> (u32, u32) {
    let prefix = element[0];
    if len == 0 {
        return (0x80, 0);
    } else if len == 1 && prefix <= 0x7F {
        return (prefix as u32, 0);
    } else if len == 1 && prefix > 0x7F { // TODO: maybe this is the same as the below case
        return (0x80 + 0x01, 1);
    } else if len <= 55 {
        return (len as u32 + 0x80 as u32, len as u32);
    } else {
        panic!("Invalid length and prefix combo {} {}", len, prefix)
    }
}

// This is the vanilla implementation of the RLC trick for verifying the decoded_list
pub fn verify_decoded_list<const L: usize, const M: usize>(list: [[u8; 32]; L], lens: [u8; L], encoding: [u8; M]) {
    let random = 1000.to_bigint().unwrap();

    let mut size_accumulator: u32 = 0;
    let mut claim_poly = BigInt::default();
    for i in 0..L {
        let (mut start_byte, list_len) = parse_list_element(list[i], lens[i]);
        let mut poly = start_byte.to_bigint().unwrap() * random.pow(size_accumulator);
        for j in 0..32 {
            poly += list[i][j] as u32 * (random.pow(1 + size_accumulator + j as u32)) * is_leq(j as u32, list_len);
        }
        size_accumulator += 1 + list_len;
        claim_poly += poly;
    }

    let mut encoding_poly = BigInt::default();
    for i in 3..M { // TODO: don't hardcode 3 here
        let idx = i - 3;
        encoding_poly += encoding[i] as u32 * (random.pow(idx as u32)) * is_le(idx as u32, size_accumulator);
    }

    assert!(claim_poly == encoding_poly);
}

pub fn to_sized_nibbles(bytes: [u8; 32]) -> [u8; 64] {
    let mut nibbles = [0u8; 64];
    let mut i = 0;
    for byte in bytes {
        nibbles[i] = byte >> 4;
        nibbles[i+1] = byte & 0x0F;
        i += 2;
    }
    nibbles
}

pub fn to_nibbles<const N: usize>(bytes: [u8; N]) -> [u8; 2*N] {
    let mut nibbles = [0u8; 2*N];
    let mut i = 0;
    for byte in bytes {
        nibbles[i] = byte >> 4;
        nibbles[i+1] = byte & 0x0F;
        i += 2;
    }
    nibbles
}

pub fn is_bytes32_eq(a: [u8; 32], b: [u8; 32]) -> u32 {
    for i in 0..32 {
        if a[i] != b[i] {
            return 0;
        }
    }
    return 1;
}

// TODO: have to implement the constrained version of this
pub fn keccack_variable<const M: usize>(input: [u8; M], len: u32) -> [u8; 32] {
    return keccak256(&input[..len as usize]);
}

pub fn is_eq(a: u8, b: usize) -> u32 {
    if a == b as u8 {
        return 1;
    } else {
        return 0;
    }
}

pub fn mux<const N: usize>(a: [u8; N], sel: u8) -> u8 {
    return a[sel as usize];
}


pub fn mux_nested<const N: usize, const M: usize>(a: [[u8; N]; M], sel: u8) -> [u8; N] {
    return a[sel as usize];
}

// Checks that a[a_offset:a_offset_len] = b[b_offset:b_offset+len]
pub fn rlc_subarray_equal<const N: usize, const M: usize>(a: [u8; N], a_offset: u32, b: [u8; M], b_offset: u32, len: u8) -> u32 {
    for i in 0..len as u32 {
        if a[(a_offset + i) as usize] != b[(b_offset + i) as usize] {
            return 0;
        }
    }
    return 1;
}

pub fn verified_get<const L: usize, const M: usize, const P: usize>(key: [u8; 32], proof: [[u8; M]; P], root: [u8; 32], value: [u8; 32], len_nodes: [u32; P]) {
    const MAX_NODE_SIZE: usize = 34;

    let mut current_key_idx: u32 = 0;
    let mut current_node_id = [0u8; MAX_NODE_SIZE];
    for i in 0..32 {
        current_node_id[i] = root[i];
    }
    let hash_key = keccak256(key);
    let key_path = to_sized_nibbles(hash_key);
    let mut finish: u32 = 0;
    let mut current_node = proof[0];
    for i in 0..P {
        println!("i: {}", i);
        current_node = proof[i];
        let current_node_hash = keccack_variable(current_node, len_nodes[i]);
        println!("current_node_hash {:?}", H256::from_slice(&current_node_hash));
        if (i == 0) {
            let is_eq = is_bytes32_eq(current_node_hash, root);
            assert!(is_eq == 1);
        } else {
            let first_32_byte_eq = is_bytes32_eq(current_node[0..32].try_into().unwrap(), current_node_id[0..32].try_into().unwrap());
            // println!("first_32_byte_eq: {}", first_32_byte_eq);
            let hash_eq = is_bytes32_eq(current_node_hash, current_node_id[0..32].try_into().unwrap());
            // println!("hash_eq: {}", hash_eq);
            // println!("{:?} {:?}", current_node_hash, current_node_id);
            let equality_fulfilled = is_leq(len_nodes[i], 32) * first_32_byte_eq as u32 + (1 - is_leq(len_nodes[i], 32)) * hash_eq as u32;
            // assert equality == 1 OR finish == 1
            assert!((equality_fulfilled as i32 -1) * (1-finish as i32) == 0);
        }

        let (decoded, decoded_lens, witness_list_len) = witness_decoding::<M, L>(current_node, len_nodes[i], finish);
        // TODO: verify_decoded_list(witness_decoded_list, witness_decoded_lens, current_node, witness_list_len, len_nodes[i]);
        
        let is_branch = is_eq(witness_list_len, BRANCH_NODE_LENGTH);
        let is_leaf = is_eq(witness_list_len, LEAF_OR_EXTENSION_NODE_LENGTH);
        let key_terminated = is_eq(current_key_idx as u8, 64);
        let path = to_nibbles(decoded[0]);
        let prefix = path[0];
        let prefix_leaf_even = is_eq(prefix, PREFIX_LEAF_EVEN);
        let prefix_leaf_odd = is_eq(prefix, PREFIX_LEAF_ODD);
        let prefix_extension_even = is_eq(prefix, PREFIX_EXTENSION_EVEN);
        let prefix_extension_odd = is_eq(prefix, PREFIX_EXTENSION_ODD);
        let offset = 2 * (prefix_extension_even as u32) + 1 * prefix_extension_odd as u32;

        let branch_key = mux(key_path, current_key_idx as u8);
        if (1-finish) * is_branch * key_terminated == 1 {
            current_node_id = decoded[TREE_RADIX];
        } else if (1-finish) * is_branch * (1-key_terminated) == 1 {
            current_node_id = mux_nested(decoded, branch_key);
        } else if (1-finish) * is_leaf == 1 {
            current_node_id = decoded[1];
        } else {
            // If finish = 1, all bets are off
            if (finish == 0) {
                panic!("Should not happen")
            }
        }

        println!("decoded {:?}", decoded);
        // The reason that we multiply decoded_lens[i] * 2 is because path and key path are both in nibbles

        // Only do the path remainder check if not finished AND is_leaf AND OR(prefix_extension_even, prefix_extension_odd)
        let do_path_remainder_check = (1-finish) * is_leaf * (1-prefix_leaf_even) * (1-prefix_leaf_odd) * (prefix_extension_even + prefix_extension_odd - prefix_extension_even * prefix_extension_odd);
        let check_length = do_path_remainder_check * (decoded_lens[0] as u32*2 - offset * do_path_remainder_check);

        rlc_subarray_equal(path, offset, key_path, current_key_idx.into(), check_length as u8);

        println!("is_leaf {}", is_leaf);
        println!("decoded_lens[0] {} offset {}", decoded_lens[0], offset);
        current_key_idx += is_branch * (1 - key_terminated) * 1 + is_leaf * check_length;

                // update finish
        if finish == 0 { // Can only toggle finish if it's false
            println!("finished {}", finish);
            finish = is_branch * key_terminated + is_leaf * prefix_leaf_even + is_leaf * prefix_leaf_odd;
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
    rlc_subarray_equal(value, 32 - current_node_len as u32, current_node_id, 1, current_node_len);
}


pub fn get_proof_witnesses<const M: usize, const P: usize>(storage_proof: Vec<Vec<u8>>) -> ([[u8; M]; P], [u32; P]) {
    if storage_proof.len() > P {
       panic!("Outer vector has incorrect length")
    }

    let mut result: [[u8; M]; P] = [[0u8; M]; P];
    let mut lengths: [u32; P] = [0u32; P];

    for (i, inner_vec) in storage_proof.into_iter().enumerate() {
        // Check inner length
        if inner_vec.len() > M {
            println!("{:?} {}", inner_vec, inner_vec.len());
            panic!("Inner vector has incorrect length");
        }
        lengths[i] = inner_vec.len() as u32;

        let mut array: [u8; M] = [0u8; M];
        // Copy the inner vec to the array
        for (j, &byte) in inner_vec.iter().enumerate() {
            array[j] = byte;
        }
        result[i] = array;
    }

    (result, lengths)
}


#[cfg(test)]
mod tests {
    use core::ops::Add;

    use anyhow::Result;
    use ethers::prelude::verify;
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
    use crate::eth::mpt::utils::{rlp_decode_list_2_or_17};

    use super::*;

    #[test]
    fn test_verify_decoded_list() {
        const MAX_SIZE: usize = 17 * 32 + 20;
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0c5becd7f8e5d47c1fe63ad9fa267d86fe0811bea0a4115aac7123b85fba2d662a03ab19202cb1de4f10fb0da8b5992c54af3dabb2312203f7477918df1393e24aea0b463eb71bcae8fa3183d0232b0d50e2400c21a0131bd48d918330e8683149b76a0d49a6c09224f74cef1286dad36a7f0e23e43e8ba4013fa386a3cda8903a3fe1ea06b6702bcfe04d3a135b786833b2748614d3aea00c728f86b2d1bbbb01b4e2311a08164a965258f9be5befcbf4de8e6cb4cd028689aad98e36ffc612b7255e4fa30a0b90309c6cb6383b2cb4cfeef9511004b705f1bca2c0556aadc2a5fe7ddf665e7a0749c3cee27e5ce85715122b76c18b7b945f1a19f507d5142445b42d50b2dd65aa0dbe35c115e9013b339743ebc2d9940158fb63b9e39f248b15ab74fade183c556a0a2b202f9b8003d73c7c84c8f7eb03298c064842382e57cecac1dfc2d5cabe2ffa02c5f8eba535bf5f18ca5aec74b51e46f219150886618c0301069dfb947006810a0dc01263a3b7c7942b5f0ac23931e0fda54fabaa3e6a58d2aca7ec65957cf8131a07d47344efa308df47f7e0e10491fa22d0564dbce634397c7748cd325fadd6b90a0cf9e45e08b8d60c68a86359adfa31c82883bb4a75b1d854392deb1f4499ba113a0081a664033eb00d5a69fc60f1f8b30e41eb643c5b9772d47301b602902b8d184a058b0bcf02a206cfa7b5f275ef09c97b4ae56abd8e9072f89dad8df1b98dfaa0280");
        let mut encoding_fixed_size = [0u8; MAX_SIZE];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let decoded_list = rlp_decode_list_2_or_17(&rlp_encoding);
        assert!(decoded_list.len() == 17);
        let element_lengths = decoded_list.iter().map(|item| item.len() as u8).collect::<Vec<u8>>();

        let mut decoded_list_fixed_size = [[0u8; 32]; 17];
        let mut element_lengths_fixed_size = [0u8; 17];
        for (i, item) in decoded_list.iter().enumerate() {
            let len = item.len();
            assert!(len <= 32, "The nested vector is longer than 32 bytes!");
            decoded_list_fixed_size[i][..len].copy_from_slice(&item);
            element_lengths_fixed_size[i] = element_lengths[i] as u8;
        }

        verify_decoded_list::<17, MAX_SIZE>(decoded_list_fixed_size, element_lengths_fixed_size, encoding_fixed_size);
    }

    #[test]
    fn test_verified_get() {
        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let block_number = 17880427u64;
        let state_root = bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
        let address = address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5");
        let location = bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");

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

        let storage_proof = storage_result.storage_proof[0].proof.iter().map(|b| b.to_vec()).collect::<Vec<Vec<u8>>>();
        let root = storage_result.storage_hash;
        let key = storage_result.storage_proof[0].key;
        let value = storage_result.storage_proof[0].value;

        println!("root {:?} key {:?} value {:?}", root, key, value);

        let value_as_h256 = u256_to_h256_be(value);
        let (proof_as_fixed, lengths_as_fixed) = get_proof_witnesses::<600, 16>(storage_proof);
        // 17 = max length of RLP decoding of proof element as list
        // 600 = max length of proof element as bytes
        // 16 = max number of elements in proof
        verified_get::<17, 600, 16>(key.to_fixed_bytes(), proof_as_fixed, root.to_fixed_bytes(), value_as_h256.to_fixed_bytes(), lengths_as_fixed);

        // Now test verified get for account proof
    }
}
