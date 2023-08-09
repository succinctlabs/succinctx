mod deserialize_bigint;

use deserialize_bigint::deserialize_bigint;
use eyre::Result;
use num::BigInt;
use reqwest::Client;
use serde::Deserialize;

/// A client used for connecting and querying a beacon node as well as Succinct's Beacon APIs.
#[derive(Debug)]
pub struct BeaconClient {
    rpc_url: String,
}

/// All Succinct Beacon APIs return a response with this format.
#[derive(Debug, Deserialize)]
struct Response<T> {
    success: bool,
    result: T,
}

/// The beacon validator struct according to the consensus spec.
/// Reference: https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#validator
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeaconValidator {
    pub withdraw_credentials: String,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: Option<u64>,
    pub withdrawable_epoch: Option<u64>,
}

/// The result returned from `/api/beacon/validator/[beacon_id]/[validator_idx]`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconValidator {
    pub validator: BeaconValidator,
    pub proof: Vec<String>,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub description: String,
}

/// The result returned from `/api/beacon/validator/[beacon_id]`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconValidatorsRoot {
    pub validators_root: String,
    pub proof: Vec<String>,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub description: String,
}

impl BeaconClient {
    /// Creates a new BeaconClient based on a rpc url.
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    /// Gets the validators root based on a beacon_id and the SSZ proof from
    /// `stateRoot -> validatorsRoot`.
    pub async fn get_validators_root(&self, beacon_id: String) -> Result<GetBeaconValidatorsRoot> {
        let endpoint = format!("{}/api/beacon/validator/{}", self.rpc_url, beacon_id);
        let client = Client::new();
        let response = client.get(endpoint).send().await?;
        let response: Response<GetBeaconValidatorsRoot> = response.json().await?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the state of a validator based on a beacon_id and index, including the SSZ proof from
    /// `validatorsRoot -> validator[validator_idx]`.
    pub async fn get_validator(
        &self,
        beacon_id: String,
        validator_idx: u64,
    ) -> Result<GetBeaconValidator> {
        let endpoint = format!(
            "{}/api/beacon/validator/{}/{}",
            self.rpc_url, beacon_id, validator_idx
        );
        let client = Client::new();
        let response = client.get(endpoint).send().await?;
        let response: Response<GetBeaconValidator> = response.json().await?;
        assert!(response.success);
        Ok(response.result)
    }
}

#[cfg(test)]
mod tests {
    extern crate dotenv;

    use std::env;

    use eyre::Result;

    use super::*;

    #[tokio::test]
    async fn test_get_validators_root_by_slot() -> Result<()> {
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_URL").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_validators_root(slot.to_string()).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_validators_root_by_block_root() -> Result<()> {
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_URL").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let block_root = "0x6b6964f45d0aeff741260ec4faaf76bb79a009fc18ae17979784d92aec374946";
        let result = client.get_validators_root(block_root.to_string()).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_validator_by_slot() -> Result<()> {
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_URL").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_validator(slot.to_string(), 0).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_validator_by_block_root() -> Result<()> {
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_URL").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let block_root = "0x6b6964f45d0aeff741260ec4faaf76bb79a009fc18ae17979784d92aec374946";
        let result = client.get_validator(block_root.to_string(), 0).await?;
        println!("{:?}", result);
        Ok(())
    }
}
