
use std::convert::TryFrom;

use alloc::sync::Arc;
use ethers::abi::{Abi, AbiEncode, Token};
use ethers::prelude::*;
use ethers::providers::{Http, JsonRpcClient, Middleware, Provider};
use ethers::types::{Address, H160, H256, U256, StorageProof};
use ethers::utils::keccak256;
use super::utils::{get_map_storage_location, u256_to_h256_be, h256_to_u256_be};

abigen!(NodeOperatorRegistry, "./src/ethereum/node_operators_abi.json");

struct LidoMetadata {
    provider: Provider<Http>,
    NODE_OPERATOR_REGISTRY_ADDR: Address,
    SIGNING_KEYS_MAPPING_NAME: H256,
    contract: NodeOperatorRegistry<Provider<Http>>
}

impl LidoMetadata {
    fn new(rpc_url: &str) -> Result<LidoMetadata, Box<dyn std::error::Error>> {
        let provider1 = Provider::<Http>::try_from(rpc_url)?;
        let NODE_OPERATOR_REGISTRY_ADDR: Address = "0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5".parse()?;
        let SIGNING_KEYS_MAPPING_NAME: H256 = "0xeb2b7ad4d8ce5610cfb46470f03b14c197c2b751077c70209c5d0139f7c79ee9".parse()?;
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let client = Arc::new(provider);
        let contract = NodeOperatorRegistry::new(NODE_OPERATOR_REGISTRY_ADDR, client);
        Ok(LidoMetadata { provider: provider1, NODE_OPERATOR_REGISTRY_ADDR, SIGNING_KEYS_MAPPING_NAME, contract })
    }

    async fn get_node_operator_info(&self, block: BlockId, operator_id: u64) -> Result<H256, Box<dyn std::error::Error>> {
        // Get the storage value
        // Sanity check against calling the contract
        // Return [storagekey, storagevalue, storageproof]
        let location: H256 = get_map_storage_location(0, operator_id.into()).unwrap();
        let storage_value = self.provider
            .get_storage_at(self.NODE_OPERATOR_REGISTRY_ADDR, location, Some(block.into()))
            .await?;
        println!("Storage at {} in block {:?}: {:?}", self.NODE_OPERATOR_REGISTRY_ADDR, block, storage_value);
        let result: (bool, std::string::String, Address, u64, u64, u64, u64) = self.contract.get_node_operator(
            U256::from(operator_id), true
        ).call().await?;
        println!("result.active: {:?}", result.0);
        println!("result.pubkey: {:?}", result.2);
        // TODO verify storage == H256(leftpad(result.pubkey, result.active))
        // TODO use provider to get storage proof
        // TODO verify said proof
        // TODO: return (location, storage_value, storage_proof)
        Ok(storage_value)
    }

    fn get_key_offset(position: H256, node_operator_id: U256, key_index: U256) -> H256 {
        // Convert U256 values to bytes and concatenate
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&position.as_bytes());
        buffer.extend_from_slice(&u256_to_h256_be(node_operator_id).as_bytes());
        buffer.extend_from_slice(&u256_to_h256_be(key_index).as_bytes());

        // Compute keccak256 and convert to U256
        H256::from(keccak256(buffer))
    }

    async fn get_operator_key_info(&self, block: BlockId, operator_id: u64, key_idx: u64) -> eyre::Result<()> {

        let signing_key: (Bytes, Bytes, Vec<bool>) = self.contract.get_signing_keys(
            U256::from(operator_id), U256::from(key_idx), U256::from(1)
        ).call().await?;
        println!("signing_key.pubkeys {:?}", signing_key.0);
        println!("signing_key.bools {:?}", signing_key.2);

        let key_offset = LidoMetadata::get_key_offset(
            self.SIGNING_KEYS_MAPPING_NAME,
            U256::from(operator_id),
            U256::from(key_idx)
        );
        let key_offset_plus_1 = h256_to_u256_be(key_offset) + U256::from(1);
        let key_offset_plus_1_h256 = u256_to_h256_be(key_offset_plus_1);

        let storage_1 = self.provider
            .get_storage_at(self.NODE_OPERATOR_REGISTRY_ADDR, key_offset, Some(block.into()))
            .await?;
        println!("Storage at {} in block {}: {:?}", self.NODE_OPERATOR_REGISTRY_ADDR, key_offset, storage_1);

        let storage_2 = self.provider
            .get_storage_at(self.NODE_OPERATOR_REGISTRY_ADDR, key_offset_plus_1_h256, Some(block.into()))
            .await?;
        println!("Storage at {} in block {}: {:?}", self.NODE_OPERATOR_REGISTRY_ADDR, key_offset_plus_1_h256, storage_2);

        // TODO verify that signing_key.0 == truncate(concat(storage_1, storage_2), 48)
        // TODO get storage proofs & return them along with key_offset
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lido_metadata() {
        let rpc_url = "https://eth.llamarpc.com";
        let lido_metadata = LidoMetadata::new(rpc_url).unwrap();
        let block = lido_metadata.provider.get_block_number().await.unwrap();
        let operator_id = 0;
        let node_operator_storage = lido_metadata.get_node_operator_info(
            block.into(), operator_id
        ).await.unwrap();
        let key_idx = 0;
        let node_pubkey_storage = lido_metadata.get_operator_key_info(
            block.into(), operator_id, key_idx
        ).await.unwrap();
    }
}
