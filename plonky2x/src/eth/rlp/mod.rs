use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use crate::builder::CircuitBuilder;

use crate::vars::{Bytes32Variable, BytesVariable};
use crate::{vars::ByteVariable};
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, EIP1186ProofResponse, H256};
use crate::utils::{bytes32, address};

pub mod utils;
pub mod template;
impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    // TODO: implement verify_decoded_list following the function in template
    pub fn verify_decoded_list<const L: usize, const M: usize>(&mut self, list: [Bytes32Variable; L], encoding: BytesVariable::<M>) {
        // L = 2 or 17
        // M is the max encoding length and should be checked as a function of L
        todo!()
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

    use super::*;

    #[test]
    fn test_verify_decoded_list() {
        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let block_number = 17880427u64;
        let state_root = bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
        let address = address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5");
        let location = bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");

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
