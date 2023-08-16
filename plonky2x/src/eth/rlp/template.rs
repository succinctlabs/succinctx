use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, EIP1186ProofResponse, H256};
use ethers::types::{Bytes};
use ethers::utils::keccak256;
use crate::utils::{bytes32, address, bytes, hex};
use num_bigint::{ToBigInt, RandBigInt, BigInt};

const TREE_RADIX: usize = 16;
const BRANCH_NODE_LENGTH: usize = 17;
const LEAF_OR_EXTENSION_NODE_LENGTH: usize = 2;
const PREFIX_EXTENSION_EVEN: usize = 0;
const PREFIX_EXTENSION_ODD: usize = 1;
const PREFIX_LEAF_EVEN: usize = 2;
const PREFIX_LEAF_ODD: usize = 3;

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
    todo!();
}

pub fn keccack_variable<const M: usize>(input: [u8; M], len: u32) -> [u8; 32] {
    todo!();
}

pub fn witness_decoding<const M: usize, const L:usize>(encoded: [u8; M], len: u32) -> ([[u8; 32]; L], [u8; L], u8) {
    todo!();
}

pub fn is_eq(a: u8, b: usize) -> u32 {
    todo!();
}

pub fn mux<const N: usize>(a: [u8; N], sel: u8) -> u8 {
    todo!();
}


pub fn mux_nested<const N: usize, const M: usize>(a: [[u8; N]; M], sel: u8) -> [u8; N] {
    todo!();
}

pub fn rlc_subarray_equal<const N: usize>(a: [u8; N], a_offset: u32, b: [u8; N], b_offset: u32, len: u8) -> u32 {
    todo!();
}

pub fn verified_get<const L: usize, const M: usize, const P: usize>(key: [u8; 32], proof: [[u8; M]; P], root: [u8; 32], value: [u8; 32], len_nodes: [u32; P]) {
    let mut current_key_idx: u32 = 0;
    let mut current_node_id = root;
    let hash_key = keccak256(key);
    let key_path = to_sized_nibbles(key);
    let mut finish = 0;
    let mut current_node = proof[0];
    for i in 0..P {
        current_node = proof[i];
        let current_node_hash = keccack_variable(current_node, len_nodes[i]);

        if (i == 0) {
            let is_eq = is_bytes32_eq(current_node_hash, root);
            assert!(is_eq == 1);
        } else {
            let first_32_byte_eq = is_bytes32_eq(current_node[0..32].try_into().unwrap(), current_node_id);
            let hash_eq = is_bytes32_eq(current_node_hash, current_node_id);
            let equality_fulfilled = is_leq(len_nodes[i], 32) * first_32_byte_eq as u32 + (1 - is_leq(len_nodes[i], 32)) * hash_eq as u32;
            assert!(equality_fulfilled == 1);
        }

        let (decoded, decoded_lens, witness_list_len) = witness_decoding::<M, L>(current_node, len_nodes[i]);
        // TODO: verify_decoded_list(witness_decoded_list, witness_decoded_lens, current_node, witness_list_len, len_nodes[i]);
        
        let is_branch = is_eq(witness_list_len, BRANCH_NODE_LENGTH);
        let key_terminated = is_eq(current_key_idx as u8, 64);
        let is_leaf = is_eq(witness_list_len, LEAF_OR_EXTENSION_NODE_LENGTH);
        let path = to_nibbles(decoded[0]);
        let prefix = path[0];
        let prefix_leaf_even = is_eq(prefix, PREFIX_LEAF_EVEN);
        let prefix_leaf_odd = is_eq(prefix, PREFIX_LEAF_ODD);
        let offset = 2 * (1 - prefix_leaf_even as u32) + 1 * prefix_leaf_even as u32;
        let prefix_extension_even = is_eq(prefix, PREFIX_EXTENSION_EVEN);
        let prefix_extension_odd = is_eq(prefix, PREFIX_EXTENSION_ODD);

        // update finish
        if finish == 0 { // Can only toggle finish if it's false
            finish = is_branch * is_eq(current_key_idx as u8, key_path.len()) + is_leaf * prefix_leaf_even + is_leaf * prefix_leaf_odd;
        }

        let branch_key = mux(key_path, current_key_idx as u8);
        if is_branch * key_terminated == 1 {
            current_node_id = decoded[TREE_RADIX];
        } else if (1-key_terminated) == 1 {
            current_node_id = mux_nested(decoded, branch_key);
        } else if is_leaf == 1 {
            current_node_id = decoded[1];
        } else {
            panic!("Should not happen")
        }

        rlc_subarray_equal(path, offset, key_path, current_key_idx.into(), decoded_lens[i]*2 - offset as u8);
        current_key_idx += is_branch * (1 - key_terminated) + is_leaf * (decoded_lens[i] as u32*2 - offset);

        // TODO other checks around the paths matching
    }
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
    use crate::eth::rlp::utils::{rlp_decode_list_2_or_17};

    use super::*;

    #[test]
    fn test_verify_decoded_list() {
        const MAX_SIZE: usize = 17 * 32 + 20;
        let rlp_encoding = bytes!("0xf90211a0c5becd7f8e5d47c1fe63ad9fa267d86fe0811bea0a4115aac7123b85fba2d662a03ab19202cb1de4f10fb0da8b5992c54af3dabb2312203f7477918df1393e24aea0b463eb71bcae8fa3183d0232b0d50e2400c21a0131bd48d918330e8683149b76a0d49a6c09224f74cef1286dad36a7f0e23e43e8ba4013fa386a3cda8903a3fe1ea06b6702bcfe04d3a135b786833b2748614d3aea00c728f86b2d1bbbb01b4e2311a08164a965258f9be5befcbf4de8e6cb4cd028689aad98e36ffc612b7255e4fa30a0b90309c6cb6383b2cb4cfeef9511004b705f1bca2c0556aadc2a5fe7ddf665e7a0749c3cee27e5ce85715122b76c18b7b945f1a19f507d5142445b42d50b2dd65aa0dbe35c115e9013b339743ebc2d9940158fb63b9e39f248b15ab74fade183c556a0a2b202f9b8003d73c7c84c8f7eb03298c064842382e57cecac1dfc2d5cabe2ffa02c5f8eba535bf5f18ca5aec74b51e46f219150886618c0301069dfb947006810a0dc01263a3b7c7942b5f0ac23931e0fda54fabaa3e6a58d2aca7ec65957cf8131a07d47344efa308df47f7e0e10491fa22d0564dbce634397c7748cd325fadd6b90a0cf9e45e08b8d60c68a86359adfa31c82883bb4a75b1d854392deb1f4499ba113a0081a664033eb00d5a69fc60f1f8b30e41eb643c5b9772d47301b602902b8d184a058b0bcf02a206cfa7b5f275ef09c97b4ae56abd8e9072f89dad8df1b98dfaa0280");
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
}
