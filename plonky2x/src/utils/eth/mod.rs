extern crate dotenv;

use std::env;

use ethers::providers::{Http, Provider};

use crate::utils::byte_to_bits_be;
pub mod beacon;
pub mod deserialize_bigint;

pub struct Address(pub [bool; 160]);

pub struct BLSPubkey(pub [bool; 384]);

impl From<Vec<u8>> for BLSPubkey {
    fn from(item: Vec<u8>) -> Self {
        let mut a = [false; 384];
        for i in 0..48 {
            let b = byte_to_bits_be(item[i]);
            for j in 0..8 {
                a[i * 8 + j] = b[j];
            }
        }
        Self(a)
    }
}

pub fn get_provider(chain_id: u64) -> Provider<Http> {
    dotenv::dotenv().ok();
    let rpc_str = format!("RPC_{}", chain_id);
    let rpc_url = env::var(rpc_str)
        .expect(format!("RPC_{} environment variable was not found", chain_id).as_str());
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    provider
}
