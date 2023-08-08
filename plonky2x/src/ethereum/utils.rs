
use ethers::abi::{AbiEncode, Token};
use ethers::types::{H256, U256, EIP1186ProofResponse, BlockId};
use ethers::utils::{keccak256,serialize};
use ethers::providers::{Http, JsonRpcClient, Middleware, Provider};

// TODO: simply import these methods from foundry
/// These are taken from: https://github.com/foundry-rs/foundry/blob/6672134672c8e442684d7d9c51fa8f8717b0f600/evm/src/utils.rs#L21
/// Small helper function to convert [U256] into [H256].
pub fn u256_to_h256_le(u: U256) -> H256 {
    let mut h = H256::default();
    u.to_little_endian(h.as_mut());
    h
}

/// Small helper function to convert [U256] into [H256].
pub fn u256_to_h256_be(u: U256) -> H256 {
    let mut h = H256::default();
    u.to_big_endian(h.as_mut());
    h
}

/// Small helper function to convert [H256] into [U256].
pub fn h256_to_u256_be(storage: H256) -> U256 {
    U256::from_big_endian(storage.as_bytes())
}

/// Small helper function to convert [H256] into [U256].
pub fn h256_to_u256_le(storage: H256) -> U256 {
    U256::from_little_endian(storage.as_bytes())
}

pub fn get_map_storage_location(mapping_location: u128, map_key: u128) -> Result<H256, Box<dyn std::error::Error>> {
    let encoded = [
        Token::Uint(U256::from(map_key)),
        Token::Uint(U256::from(mapping_location)),
    ]
    .encode();
    let hash = keccak256(encoded);
    Ok(H256::from(hash))
}
