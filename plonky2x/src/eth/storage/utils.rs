
use ethers::abi::{AbiEncode, Token};
use ethers::types::{H256, U256};
use ethers::utils::{keccak256};

pub fn get_map_storage_location(mapping_location: u128, map_key: u128) -> H256 {
    let encoded = [
        Token::Uint(U256::from(map_key)),
        Token::Uint(U256::from(mapping_location)),
    ]
    .encode();
    let hash = keccak256(encoded);
    H256::from(hash)
}
