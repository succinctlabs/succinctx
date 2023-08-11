use alloc::sync::Arc;
use std::convert::TryFrom;

use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, H256, U256};
use ethers::utils::keccak256;
use eyre::Result;

use crate::eth::storage::utils::get_map_storage_location;
use crate::eth::utils::{h256_to_u256_be, u256_to_h256_be};

abigen!(NodeOperatorRegistry, "./src/lido/node_operators_abi.json");

const NODE_OPERATOR_REGISTRY_ADDR: &str = "0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5";
const SIGNING_KEYS_MAPPING_NAME: &str =
    "0xeb2b7ad4d8ce5610cfb46470f03b14c197c2b751077c70209c5d0139f7c79ee9";

pub struct LidoUtils {
    provider: Provider<Http>,
    contract: NodeOperatorRegistry<Provider<Http>>,
}

impl LidoUtils {
    pub fn new(rpc_url: &str) -> LidoUtils {
        let provider1 = Provider::<Http>::try_from(rpc_url).unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let client = Arc::new(provider);
        let contract = NodeOperatorRegistry::new(
            NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>().unwrap(),
            client,
        );
        LidoUtils {
            provider: provider1,
            contract,
        }
    }

    pub async fn get_node_operator_info(
        &self,
        block: BlockId,
        operator_id: u64,
    ) -> Result<EIP1186ProofResponse> {
        // Get the storage value
        // Sanity check against calling the contract
        // Return [storagekey, storagevalue, storageproof]
        let location: H256 = get_map_storage_location(0, operator_id.into());
        let storage_value = self
            .provider
            .get_storage_at(
                NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
                location,
                Some(block.into()),
            )
            .await?;
        println!(
            "Storage at {} in block {:?}: {:?}",
            NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
            block,
            storage_value
        );
        let result: (bool, std::string::String, Address, u64, u64, u64, u64) = self
            .contract
            .get_node_operator(U256::from(operator_id), true)
            .call()
            .await?;
        println!("result.active: {:?}", result.0);
        println!("result.pubkey: {:?}", result.2);
        // TODO verify storage == H256(leftpad(result.pubkey, result.active))
        // TODO verify below proof
        let proof = self
            .provider
            .get_proof(
                NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
                vec![location],
                Some(block.into()),
            )
            .await?;
        Ok(proof)
    }

    pub fn get_key_offset(position: H256, node_operator_id: U256, key_index: U256) -> H256 {
        // Convert U256 values to bytes and concatenate
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&position.as_bytes());
        buffer.extend_from_slice(&u256_to_h256_be(node_operator_id).as_bytes());
        buffer.extend_from_slice(&u256_to_h256_be(key_index).as_bytes());

        // Compute keccak256 and convert to U256
        H256::from(keccak256(buffer))
    }

    pub async fn get_operator_key_info(
        &self,
        block: BlockId,
        operator_id: u64,
        key_idx: u64,
    ) -> Result<EIP1186ProofResponse> {
        let signing_key: (Bytes, Bytes, Vec<bool>) = self
            .contract
            .get_signing_keys(U256::from(operator_id), U256::from(key_idx), U256::from(1))
            .call()
            .await?;
        println!("signing_key.pubkeys {:?}", signing_key.0);
        println!("signing_key.bools {:?}", signing_key.2);

        let key_offset = LidoUtils::get_key_offset(
            SIGNING_KEYS_MAPPING_NAME.parse::<H256>()?,
            U256::from(operator_id),
            U256::from(key_idx),
        );
        let key_offset_plus_1 = h256_to_u256_be(key_offset) + U256::from(1);
        let key_offset_plus_1_h256 = u256_to_h256_be(key_offset_plus_1);

        let storage_1 = self
            .provider
            .get_storage_at(
                NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
                key_offset,
                Some(block.into()),
            )
            .await?;
        println!(
            "Storage at {} in block {}: {:?}",
            NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
            key_offset,
            storage_1
        );

        let storage_2 = self
            .provider
            .get_storage_at(
                NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
                key_offset_plus_1_h256,
                Some(block.into()),
            )
            .await?;
        println!(
            "Storage at {} in block {}: {:?}",
            NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
            key_offset_plus_1_h256,
            storage_2
        );

        // TODO verify that signing_key.0 == truncate(concat(storage_1, storage_2), 48)

        let proof = self
            .provider
            .get_proof(
                NODE_OPERATOR_REGISTRY_ADDR.parse::<Address>()?,
                vec![key_offset, key_offset_plus_1_h256],
                Some(block.into()),
            )
            .await?;
        Ok(proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lido_metadata() {
        let rpc_url = "https://eth.llamarpc.com";
        let lido_metadata = LidoUtils::new(rpc_url);
        let block = lido_metadata.provider.get_block_number().await.unwrap();
        let operator_id = 0;
        let node_operator_proof = lido_metadata
            .get_node_operator_info(block.into(), operator_id)
            .await
            .unwrap();
        let full_block = lido_metadata.provider.get_block(block).await.unwrap();
        let state_root = full_block.unwrap().state_root;
        println!("block number {:?} state root {:?}", block, state_root);
        println!(
            "node_operator_proof {:?} {:?}",
            node_operator_proof.storage_proof[0].key,
            u256_to_h256_be(node_operator_proof.storage_proof[0].value)
        );
        let key_idx = 0;
        let operator_key_proof = lido_metadata
            .get_operator_key_info(block.into(), operator_id, key_idx)
            .await
            .unwrap();
        // println!("operator_key_proof {:?}", operator_key_proof);
    }
}
