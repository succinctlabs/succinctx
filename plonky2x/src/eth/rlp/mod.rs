use crate::{builder::BuilderAPI, vars::ByteVariable};
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, EIP1186ProofResponse, H256};

pub mod utils;

pub struct RecursiveLengthPrefixAPI {
    pub api: BuilderAPI,
}

impl RecursiveLengthPrefixAPI {
    pub fn new(api: BuilderAPI) -> Self {
        Self { api }
    }


    // pub fn verify_sized_list_rlp(list: [[u8; 32];17], ele_lengths: [usize; 17], claimed_encoding: [u8; MAX_ENCODING_LENGTH]) {
    //     let s_accum = 0;
    //     for i in 0..17 {
    //         let mut poly, eles_to_add = first_entry_and_addition_length(ele_lengths[i], list[i]); // i.e. 0x80 if 0, 0xa0 if 32, etc.
    //         for j in 0..32 {
    //             poly += list_padded[i][j] * (random**(1 + s_accum + j)) * is_leq(i, eles_to_add[i]);
    //         }
    //         s_accum += 1 + eles_to_add;
    //     }

    //     claim_poly = 0;
    //     for i in 0..MAX_ENCODING_LENGTH {
    //         claim_poly += claimed_encoding[i] * (random**i) * is_leq(i, s_accum);
    //     }

    //     require(poly == claim_poly)
    // }

    // pub fn get_constrained(key: H256, proof: [[u8; MAX_BYTES_IN_NODE]; MAX_PROOF_ELES], node_type: [u8; MAX_PROOF_ELES], root: H256, claimed_value: H256) {
    //     let current_key_index = F::Zero();
    //     let current_node_id = root;
    //     let finished = 0;

    //     for i in 0..MAX_PROOF_ELES {
    //         // TODO do the hash verifications
    //         verify_decoded_witness(proof[i], node_type[i], list_value[i]);

    //         current_node_id = apply_transition(current_node_id) * (1-finished) + finished * current_node_id;


    //         // finished = finished + (current_key_index == key.len() - 1);
    //     }

    // }
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
    fn test_rlp_decode() {
        let mut stream = RlpStream::new();
        stream.begin_list(1);
        let h256 =
            "0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e"
                .parse::<H256>()
                .unwrap();
        let u256 = h256_to_u256_be(h256);
        stream.append::<U256>(&u256.into());
        let encoding = stream.out().freeze();
        println!("encoding {:x?}", encoding);
    }

    #[test]
    fn test_mpt_proof() {
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
    }

}
