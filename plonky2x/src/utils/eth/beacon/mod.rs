use core::time::Duration;

use anyhow::Result;
use ethers::types::U256;
use log::{debug, info};
use num::BigInt;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use serde_with::serde_as;

use crate::utils::serde::deserialize_bigint;

/// A client used for connecting and querying a beacon node.
#[derive(Debug, Clone)]
pub struct BeaconClient {
    rpc_url: String,
}

/// The data format returned by official Eth Beacon Node APIs.
#[derive(Debug, Deserialize)]
struct BeaconData<T> {
    #[allow(unused)]
    pub execution_optimistic: bool,
    #[allow(unused)]
    pub finalized: bool,
    #[allow(unused)]
    pub data: Vec<T>,
}

/// The format returned by official Eth Beacon Node APIs.
#[derive(Debug, Deserialize)]
struct BeaconResponse<T> {
    #[allow(unused)]
    data: BeaconData<T>,
}

/// All custom endpoints return a response with this format.
#[derive(Debug, Deserialize)]
struct CustomResponse<T> {
    success: bool,
    result: T,
}

/// The beacon header according to the consensus spec.
/// Reference: https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#beaconblockheader
#[derive(Debug, Deserialize)]
pub struct BeaconHeader {
    pub slot: String,
    pub proposer_index: String,
    pub parent_root: String,
    pub state_root: String,
    pub body_root: String,
}

/// The beacon validator struct according to the consensus spec.
/// Reference: https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#validator
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct BeaconValidator {
    pub pubkey: String,
    pub withdrawal_credentials: String,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: String,
    pub withdrawable_epoch: String,
}

/// The beacon validator balance returned by the official Beacon Node API.
/// https://ethereum.github.io/beacon-APIs/#/Beacon/getStateValidatorBalances
#[derive(Debug, Deserialize)]
struct BeaconValidatorBalance {
    #[allow(unused)]
    pub index: String,
    #[allow(unused)]
    pub balance: String,
}

/// The result returned from `/api/beacon/proof/validator/[beacon_id]`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconValidatorsRoot {
    pub validators_root: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
    pub proof: Vec<String>,
}

/// The result returned from `/api/beacon/proof/validator/[beacon_id]/[validator_idx]`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconValidator {
    pub validator_root: String,
    pub validators_root: String,
    pub validator_idx: u64,
    pub validator: BeaconValidator,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
    pub proof: Vec<String>,
}

/// The result returned from `/api/beacon/validator/[beacon_id]/[validator_idx]`.
/// Note that this endpoint returns only the validator struct, without any SSZ proofs.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconValidatorWitness {
    pub validator: BeaconValidator,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconBalancesRoot {
    pub balances_root: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
    pub proof: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconBalance {
    pub balance: u64,
    pub balance_leaf: String,
    pub balances_root: String,
    pub proof: Vec<String>,
    pub depth: u64,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconBalanceWitness {
    pub balance: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconBalanceBatchWitness {
    pub balances: Vec<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconWithdrawalsRoot {
    pub withdrawals_root: String,
    pub proof: Vec<String>,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub amount: BigInt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconWithdrawal {
    pub withdrawal: Withdrawal,
    pub withdrawal_root: String,
    pub proof: Vec<String>,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconHistoricalBlock {
    pub historical_block_root: String,
    pub proof: Vec<String>,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
}

impl BeaconClient {
    /// Creates a new BeaconClient based on a rpc url.
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    /// Gets the block root at `head`.
    pub fn get_finalized_block_root_sync(&self) -> Result<String> {
        self.get_finalized_block_root()
    }

    /// Gets the latest block root at `head` asynchronously.
    pub fn get_finalized_block_root(&self) -> Result<String> {
        let endpoint = format!("{}/eth/v1/beacon/headers/finalized", self.rpc_url);
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let parsed: Value = response.json()?;

        if let Value::Object(data) = &parsed["data"] {
            if let Value::Object(data2) = &data["data"] {
                return Ok(data2["root"].as_str().unwrap().to_string());
            }
        }

        Err(anyhow::anyhow!("failed to parse response"))
    }

    /// Gets the validators root based on a beacon_id and the SSZ proof from
    /// `stateRoot -> validatorsRoot`.
    pub fn get_validators_root(&self, beacon_id: String) -> Result<GetBeaconValidatorsRoot> {
        let endpoint = format!("{}/api/beacon/proof/validator/{}", self.rpc_url, beacon_id);
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconValidatorsRoot> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the state of a validator based on a beacon_id and index, omitting any SSZ proofs.
    /// With repeated calls on the same beacon_id, this should be faster than `get_validator`.
    pub fn get_validator_witness(
        &self,
        beacon_id: String,
        validator_idx: u64,
    ) -> Result<GetBeaconValidatorWitness> {
        let endpoint = format!(
            "{}/api/beacon/validator/{}/{}",
            self.rpc_url, beacon_id, validator_idx
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconValidatorWitness> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the state of a validator based on a beacon_id and index, including the SSZ proof from
    /// `validatorsRoot -> validator[validator_idx]`.
    pub fn get_validator(
        &self,
        beacon_id: String,
        validator_idx: u64,
    ) -> Result<GetBeaconValidator> {
        let endpoint = format!(
            "{}/api/beacon/proof/validator/{}/{}",
            self.rpc_url, beacon_id, validator_idx
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconValidator> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the state of a validator based on a beacon_id and index, including the SSZ proof from
    /// `validatorsRoot -> validator[validator_idx]`.
    pub fn get_validator_by_pubkey(
        &self,
        beacon_id: String,
        pubkey: String,
    ) -> Result<GetBeaconValidator> {
        let endpoint = format!(
            "{}/api/beacon/proof/validator/{}/{}",
            self.rpc_url, beacon_id, pubkey
        );
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconValidator> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the balances root based on a beacon_id.
    pub fn get_balances_root(&self, beacon_id: String) -> Result<GetBeaconBalancesRoot> {
        let endpoint = format!("{}/api/beacon/proof/balance/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconBalancesRoot> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_balance_witness(&self, beacon_id: String, idx: u64) -> Result<u64> {
        let endpoint = format!("{}/api/beacon/balance/{}/{}", self.rpc_url, beacon_id, idx);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(endpoint)
            .timeout(Duration::from_secs(300))
            .send()?;
        let response: GetBeaconBalanceWitness = response.json()?;
        Ok(response.balance)
    }

    pub fn get_balance_batch_witness(
        &self,
        beacon_id: String,
        start_idx: u64,
        end_idx: u64,
    ) -> Result<Vec<u64>> {
        let endpoint = format!(
            "{}/api/beacon/balance/{}/{},{}",
            self.rpc_url, beacon_id, start_idx, end_idx
        );
        debug!("{}", endpoint);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(endpoint)
            .timeout(Duration::from_secs(300))
            .send()?;
        let response: GetBeaconBalanceBatchWitness = response.json()?;
        Ok(response.balances)
    }

    /// Gets the balance of a validator based on a beacon_id and validator index.
    pub fn get_validator_balance_v2(
        &self,
        beacon_id: String,
        validator_idx: u64,
    ) -> Result<GetBeaconBalance> {
        let endpoint = format!(
            "{}/api/beacon/proof/balance/{}/{}",
            self.rpc_url, beacon_id, validator_idx
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconBalance> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the balance of a validator based on a beacon_id and validator pubkey.
    pub fn get_validator_balance_by_pubkey_v2(
        &self,
        beacon_id: String,
        pubkey: String,
    ) -> Result<GetBeaconBalance> {
        let endpoint = format!(
            "{}/api/beacon/proof/balance/{}/{}",
            self.rpc_url, beacon_id, pubkey
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconBalance> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the balance of a validator based on a beacon_id and validator index.
    #[allow(unused)]
    fn get_validator_balance_deprecated(
        &self,
        beacon_id: String,
        validator_idx: u64,
    ) -> Result<U256> {
        let endpoint = format!(
            "{}/eth/v1/beacon/states/{}/validator_balances?id={}",
            self.rpc_url, beacon_id, validator_idx
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: BeaconResponse<BeaconValidatorBalance> = response.json()?;
        let balance = response.data.data[0].balance.parse::<u64>()?;
        Ok(U256::from(balance))
    }

    /// Gets the balance of a validator based on a beacon_id and validator pubkey.
    #[allow(unused)]
    fn get_validator_balance_by_pubkey_deprecated(
        &self,
        beacon_id: String,
        pubkey: String,
    ) -> Result<U256> {
        let endpoint = format!(
            "{}/eth/v1/beacon/states/{}/validator_balances?id={}",
            self.rpc_url, beacon_id, pubkey
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: BeaconResponse<BeaconValidatorBalance> = response.json()?;
        let balance = response.data.data[0].balance.parse::<u64>()?;
        Ok(U256::from(balance))
    }

    pub fn get_withdrawals_root(&self, beacon_id: String) -> Result<GetBeaconWithdrawalsRoot> {
        let endpoint = format!("{}/api/beacon/withdrawal/{}", self.rpc_url, beacon_id);
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconWithdrawalsRoot> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_withdrawal(&self, beacon_id: String, idx: u64) -> Result<GetBeaconWithdrawal> {
        let endpoint = format!(
            "{}/api/beacon/proof/withdrawal/{}/{}",
            self.rpc_url, beacon_id, idx
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconWithdrawal> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_historical_block(
        &self,
        beacon_id: String,
        offset: u64,
    ) -> Result<GetBeaconHistoricalBlock> {
        let endpoint = format!(
            "{}/api/beacon/proof/historical/{}/{}",
            self.rpc_url, beacon_id, offset
        );
        let client = Client::new();
        let response = client.get(endpoint).send()?;
        let response: CustomResponse<GetBeaconHistoricalBlock> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }
}

#[cfg(test)]
mod tests {
    extern crate dotenv;

    use std::env;

    use anyhow::Result;
    use log::debug;

    use super::*;
    use crate::utils;

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_validators_root_by_slot() -> Result<()> {
        utils::setup_logger();
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_validators_root(slot.to_string())?;
        debug!("{:?}", result);
        Ok(())
    }

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_validators_root_by_block_root() -> Result<()> {
        utils::setup_logger();
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let block_root = "0x6b6964f45d0aeff741260ec4faaf76bb79a009fc18ae17979784d92aec374946";
        let result = client.get_validators_root(block_root.to_string())?;
        debug!("{:?}", result);
        Ok(())
    }

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_validator_by_slot() -> Result<()> {
        utils::setup_logger();
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_validator(slot.to_string(), 0)?;
        debug!("{:?}", result);
        Ok(())
    }

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_validator_by_block_root() -> Result<()> {
        utils::setup_logger();
        dotenv::dotenv()?;
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let block_root = "0x6b6964f45d0aeff741260ec4faaf76bb79a009fc18ae17979784d92aec374946";
        let result = client.get_validator(block_root.to_string(), 0)?;
        debug!("{:?}", result);
        Ok(())
    }
}
