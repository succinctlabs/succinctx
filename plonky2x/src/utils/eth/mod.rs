extern crate dotenv;

use std::env;

use ethers::providers::{Http, Provider};

pub mod beacon;

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
