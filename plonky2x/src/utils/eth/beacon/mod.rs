use core::time::Duration;

use anyhow::Result;
use ethers::types::{H256, U256};
use itertools::Itertools;
use log::{debug, info};
use num::BigInt;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;

use crate::utils::hash::sha256;
use crate::utils::reqwest::ReqwestClient;
use crate::utils::serde::deserialize_bigint;

/// A client used for connecting and querying a beacon node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconClient {
    rpc_url: String,
    client: ReqwestClient,
}

/// The data format returned by official Eth Beacon Node APIs.
#[derive(Debug, Deserialize)]
struct BeaconData<T> {
    #[allow(unused)]
    pub execution_optimistic: bool,
    #[allow(unused)]
    pub finalized: bool,
    pub data: T,
}

/// All custom endpoints return a response with this format.
#[derive(Debug, Deserialize)]
struct CustomResponse<T> {
    success: bool,
    result: T,
}

#[derive(Debug, Deserialize)]
pub struct BeaconHeaderContainer {
    pub root: String,
    pub canonical: bool,
    pub header: BeaconHeaderMessage,
}

#[derive(Debug, Deserialize)]
pub struct BeaconHeaderMessage {
    pub message: BeaconHeader,
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

impl BeaconValidator {
    pub fn ssz_merkleize(&self) -> (H256, Vec<H256>) {
        let pubkey_bytes = hex::decode(&self.pubkey.as_str()[2..]).unwrap();
        let mut pubkey_p1 = [0u8; 32];
        pubkey_p1.copy_from_slice(&pubkey_bytes[..32]);
        let mut pubkey_p2 = [0u8; 32];
        pubkey_p2[0..16].copy_from_slice(&pubkey_bytes[32..]);
        let pubkey = sha256(&[pubkey_p1, pubkey_p2].concat());

        let withdrawal_credentials_bytes =
            hex::decode(&self.withdrawal_credentials.as_str()[2..]).unwrap();
        let mut withdrawal_credentials = [0u8; 32];
        withdrawal_credentials.copy_from_slice(&withdrawal_credentials_bytes[..]);

        let effective_balance_bytes = self.effective_balance.to_le_bytes();
        let mut effective_balance = [0u8; 32];
        effective_balance[0..8].copy_from_slice(&effective_balance_bytes);

        let mut slashed = [0u8; 32];
        slashed[0] = if self.slashed { 1u8 } else { 0u8 };

        let activation_eligibility_epoch_bytes = self.activation_eligibility_epoch.to_le_bytes();
        let mut activation_eligibility_epoch = [0u8; 32];
        activation_eligibility_epoch[0..8].copy_from_slice(&activation_eligibility_epoch_bytes);

        let activation_epoch_bytes = self.activation_epoch.to_le_bytes();
        let mut activation_epoch = [0u8; 32];
        activation_epoch[0..8].copy_from_slice(&activation_epoch_bytes);

        let (_, exit_epoch_bytes) = self.exit_epoch.parse::<BigInt>().unwrap().to_bytes_le();
        let mut exit_epoch = [0u8; 32];
        exit_epoch[0..exit_epoch_bytes.len()].copy_from_slice(&exit_epoch_bytes);

        let (_, withdrawable_epoch_bytes) = self
            .withdrawable_epoch
            .parse::<BigInt>()
            .unwrap()
            .to_bytes_le();
        let mut withdrawable_epoch = [0u8; 32];
        withdrawable_epoch[0..withdrawable_epoch_bytes.len()]
            .copy_from_slice(&withdrawable_epoch_bytes);

        let h11 = sha256(&[pubkey, withdrawal_credentials].concat());
        let h12 = sha256(&[effective_balance, slashed].concat());
        let h13 = sha256(&[activation_eligibility_epoch, activation_epoch].concat());
        let h14 = sha256(&[exit_epoch, withdrawable_epoch].concat());
        let h21 = sha256(&[h11, h12].concat());
        let h22 = sha256(&[h13, h14].concat());
        let h31 = sha256(&[h21, h22].concat());

        (
            H256::from(&h31),
            vec![h11, h12, h13, h14, h21, h22, h31]
                .iter()
                .map(H256::from)
                .collect_vec(),
        )
    }

    pub fn ssz_hash_tree_root(&self) -> H256 {
        let (root, _) = self.ssz_merkleize();
        root
    }
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconPartialValidatorsRoot {
    pub partial_validators_root: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
    pub proof: Vec<String>,
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
pub struct GetBeaconValidatorBatchWitness {
    pub validators: Vec<BeaconValidator>,
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
pub struct GetBeaconPartialBalancesRoot {
    pub partial_balances_root: String,
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
pub struct GetBeaconWithdrawals {
    pub withdrawals: Vec<Withdrawal>,
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
    pub target_block_root: String,
    pub far_slot_historical_summary_root: String,
    pub far_slot_historical_summary_proof: Vec<String>,
    pub far_slot_block_root_proof: Vec<String>,
    pub close_slot_block_root_proof: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconExecutionPayload {
    pub block_number: String,
    pub proof: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconSlotNumber {
    pub slot: u64,
    pub proof: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconBlockRoots {
    pub block_roots_root: String,
    pub block_roots: Vec<String>,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
    pub proof: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconGraffiti {
    pub graffiti: String,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub gindex: BigInt,
    pub depth: u64,
    pub proof: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBeaconHeadersFromOffsetRange {
    pub headers: Vec<String>,
}

impl BeaconClient {
    /// Creates a new BeaconClient based on a rpc url.
    pub fn new(rpc_url: String) -> Self {
        Self {
            rpc_url,
            client: ReqwestClient::new(),
        }
    }

    /// Gets the block root at `head`.
    pub fn get_finalized_block_root_sync(&self) -> Result<String> {
        self.get_finalized_block_root()
    }

    /// Gets the latest finalized block root asynchronously.
    pub fn get_finalized_block_root(&self) -> Result<String> {
        let endpoint = format!("{}/eth/v1/beacon/headers/finalized", self.rpc_url);
        let response = self.client.fetch(&endpoint)?;
        let parsed: Value = response.json()?;

        if let Value::Object(data) = &parsed["data"] {
            return Ok(data["root"].as_str().unwrap().to_string());
        }

        Err(anyhow::anyhow!("failed to parse response"))
    }

    /// Gets the latest finalized slot asynchronously.
    pub fn get_finalized_slot(&self) -> Result<String> {
        let endpoint = format!("{}/eth/v1/beacon/headers/finalized", self.rpc_url);
        let response = self.client.fetch(&endpoint)?;
        let parsed: Value = response.json()?;

        if let Value::Object(data) = &parsed["data"] {
            return Ok(data["header"]["message"]["slot"]
                .as_str()
                .unwrap()
                .to_string());
        }

        Err(anyhow::anyhow!("failed to parse response"))
    }

    /// Gets the partial balances root based on a beacon_id and the number of expected balances.
    pub fn get_partial_validators_root(
        &self,
        beacon_id: String,
        nb_balances: usize,
    ) -> Result<GetBeaconPartialValidatorsRoot> {
        let endpoint = format!(
            "{}/api/beacon/proof/partialValidator/{}/{}",
            self.rpc_url, beacon_id, nb_balances
        );
        info!("{}", endpoint);
        let client = Client::new();
        let response = client
            .get(endpoint)
            .timeout(Duration::from_secs(300))
            .send()?;
        let response: CustomResponse<GetBeaconPartialValidatorsRoot> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the validators root based on a beacon_id and the SSZ proof from
    /// `stateRoot -> validatorsRoot`.
    pub fn get_validators_root(&self, beacon_id: String) -> Result<GetBeaconValidatorsRoot> {
        let endpoint = format!("{}/api/beacon/proof/validator/{}", self.rpc_url, beacon_id);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(120, 0)).send()?;
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
        let response = self.client.fetch(&endpoint)?;
        let response: CustomResponse<GetBeaconValidatorWitness> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_validator_batch_witness(
        &self,
        beacon_id: String,
        start_idx: u64,
        end_idx: u64,
    ) -> Result<Vec<BeaconValidator>> {
        let endpoint = format!(
            "{}/api/beacon/validator/{}/{},{}",
            self.rpc_url, beacon_id, start_idx, end_idx
        );
        debug!("{}", endpoint);
        let response = self.client.fetch(&endpoint)?;
        let response: GetBeaconValidatorBatchWitness = response.json()?;
        Ok(response.validators)
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
        let response = self.client.fetch(&endpoint)?;
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
        let response = client.get(endpoint).timeout(Duration::new(120, 0)).send()?;
        let response: CustomResponse<GetBeaconValidator> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the balances root based on a beacon_id.
    pub fn get_balances_root(&self, beacon_id: String) -> Result<GetBeaconBalancesRoot> {
        let endpoint = format!("{}/api/beacon/proof/balance/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let response = self.client.fetch(&endpoint)?;
        let response: CustomResponse<GetBeaconBalancesRoot> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the partial balances root based on a beacon_id and the number of expected balances.
    pub fn get_partial_balances_root(
        &self,
        beacon_id: String,
        nb_balances: usize,
    ) -> Result<GetBeaconPartialBalancesRoot> {
        let endpoint = format!(
            "{}/api/beacon/proof/partialBalance/{}/{}",
            self.rpc_url, beacon_id, nb_balances
        );
        info!("{}", endpoint);
        let client = Client::new();
        let response = client
            .get(endpoint)
            .timeout(Duration::from_secs(300))
            .send()?;
        let response: CustomResponse<GetBeaconPartialBalancesRoot> = response.json()?;
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
        let response = self.client.fetch(&endpoint)?;
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
        let response = self.client.fetch(&endpoint)?;
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
        let response = self.client.fetch(&endpoint)?;
        let response: BeaconData<Vec<BeaconValidatorBalance>> = response.json()?;
        let balance = response.data[0].balance.parse::<u64>()?;
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
        let response = self.client.fetch(&endpoint)?;
        let response: BeaconData<Vec<BeaconValidatorBalance>> = response.json()?;
        let balance = response.data[0].balance.parse::<u64>()?;
        Ok(U256::from(balance))
    }

    pub fn get_withdrawals(&self, beacon_id: String) -> Result<GetBeaconWithdrawals> {
        let endpoint = format!("{}/api/beacon/proof/withdrawal/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(120, 0)).send()?;
        let response: CustomResponse<GetBeaconWithdrawals> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_withdrawal(&self, beacon_id: String, idx: u64) -> Result<GetBeaconWithdrawal> {
        let endpoint = format!(
            "{}/api/beacon/proof/withdrawal/{}/{}",
            self.rpc_url, beacon_id, idx
        );
        let response = self.client.fetch(&endpoint)?;
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
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(240, 0)).send()?;
        let response: CustomResponse<GetBeaconHistoricalBlock> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the execution payload at the given `beacon_id`.
    pub fn get_execution_payload(&self, beacon_id: String) -> Result<GetBeaconExecutionPayload> {
        let endpoint = format!(
            "{}/api/beacon/proof/executionPayload/{}",
            self.rpc_url, beacon_id
        );
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(60, 0)).send()?;
        let response: CustomResponse<GetBeaconExecutionPayload> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the slot from header + SSZ proof at the given `beacon_id`.
    pub fn get_slot_number(&self, beacon_id: String) -> Result<GetBeaconSlotNumber> {
        let endpoint = format!("{}/api/beacon/proof/slot/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(60, 0)).send()?;
        let response: CustomResponse<GetBeaconSlotNumber> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    /// Gets the block header at the given `beacon_id`.
    pub fn get_header(&self, beacon_id: String) -> Result<BeaconHeader> {
        let endpoint = format!("{}/eth/v1/beacon/headers/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let response = self.client.fetch(&endpoint)?;
        let parsed: BeaconData<BeaconHeaderContainer> = response.json()?;

        Ok(parsed.data.header.message)
    }

    pub fn get_block_roots(&self, beacon_id: String) -> Result<GetBeaconBlockRoots> {
        let endpoint = format!("{}/api/beacon/proof/blockRoots/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(60, 0)).send()?;
        let response: CustomResponse<GetBeaconBlockRoots> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_graffiti(&self, beacon_id: String) -> Result<GetBeaconGraffiti> {
        let endpoint = format!("{}/api/beacon/proof/graffiti/{}", self.rpc_url, beacon_id);
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(60, 0)).send()?;
        let response: CustomResponse<GetBeaconGraffiti> = response.json()?;
        assert!(response.success);
        Ok(response.result)
    }

    pub fn get_headers_from_offset_range(
        &self,
        beacon_id: String,
        start_offset: u64,
        end_offset: u64,
    ) -> Result<GetBeaconHeadersFromOffsetRange> {
        let endpoint = format!(
            "{}/api/beacon/header/offset/{}/{}/{}",
            self.rpc_url, beacon_id, start_offset, end_offset
        );
        info!("{}", endpoint);
        let client = Client::new();
        let response = client.get(endpoint).timeout(Duration::new(60, 0)).send()?;
        let response: CustomResponse<GetBeaconHeadersFromOffsetRange> = response.json()?;
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

    #[test]
    fn test_validator_hash_tree_root() {
        let validator = BeaconValidator {
            pubkey: "0x2a2c40d5177456d2b260cf39ee5426c2ce04096d1970aa4afe8306f1e24d1e1b5f1860d228b46ef0f7b01950b34aef17".to_string(),
            withdrawal_credentials: "0xfad764748d2fb342f8e9f88ea2ffb9833b7c2e8ae1f78921057e4749688cd13b".to_string(),
            effective_balance: 6,
            slashed: true,
            activation_eligibility_epoch: 6,
            activation_epoch: 7,
            exit_epoch: "0".to_string(),
            withdrawable_epoch: "0".to_string()
        };
        let root = validator.ssz_hash_tree_root();
        println!("{}", root);
    }

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

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_block_roots() -> Result<()> {
        utils::setup_logger();
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_block_roots(slot.to_string())?;
        debug!("{:?}", result);
        Ok(())
    }

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_graffiti() -> Result<()> {
        utils::setup_logger();
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_graffiti(slot.to_string())?;
        debug!("{:?}", result);
        Ok(())
    }

    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_get_headers_from_offset_range() -> Result<()> {
        utils::setup_logger();
        let rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc.to_string());
        let slot = 7052735;
        let result = client.get_headers_from_offset_range(slot.to_string(), 0, 16)?;
        debug!("{:?}", result);
        Ok(())
    }
}
