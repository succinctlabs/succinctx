extern crate dotenv;

use std::env;

use ethers::providers::{Http, Provider};

pub mod beacon;
pub mod beaconchain;

#[derive(Debug, Clone)]
pub struct Address(pub [u8; 20]);

#[derive(Debug, Clone)]
pub struct BLSPubkey(pub [u8; 48]);

pub fn get_provider(chain_id: u64) -> Provider<Http> {
    dotenv::dotenv().ok();
    let rpc_str = format!("RPC_{}", chain_id);
    let rpc_url = env::var(rpc_str)
        .unwrap_or_else(|_| format!("RPC_{} environment variable was not found", chain_id));
    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub fn concat_g_indices(gindexes: &[usize]) -> usize {
    let mut index = 1;
    for &g in gindexes {
        let mut num_bits = 0;
        let mut temp = g;
        while temp > 0 {
            temp >>= 1;
            num_bits += 1;
        }

        index <<= num_bits - 1;
        index += g % (2_usize.pow(num_bits as u32 - 1));
    }
    index
}
