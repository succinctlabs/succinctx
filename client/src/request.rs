use std::process::Command;
use std::{env, fs};

use alloy_primitives::{Address, Bytes, B256};
use anyhow::{Error, Result};
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json as json_macro;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
/// Data to be sent to the Succinct X API with an offchain request.
struct OffchainInput {
    /// The chain id of the network to be used.
    chainId: u32,
    /// The address of the contract to call.
    to: Address,
    /// The calldata to be used in the contract call.
    data: Bytes,
    /// The Succinct X function id to be called.
    functionId: B256,
    /// The input to be used in the Succinct X function call.
    input: Bytes,
}

#[derive(Serialize, Deserialize)]
/// Data received from the Succinct X API from an offchain request.
struct OffchainRequestResponse {
    request_id: String,
}

/// Client to interact with the Succinct X API.
pub struct SuccinctClient {
    client: Client,
    /// The base url for the Succinct X API. (ex. https://alpha.succinct.xyz/api)
    base_url: String,
    /// API key for the Succinct X API.
    api_key: String,
}

impl SuccinctClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        if base_url == "local" {
            info!("Running SuccinctClient in local mode");
        }
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    pub fn local_mode(
        &self,
        chain_id: u32,
        to: Address,
        calldata: Bytes,
        function_id: B256,
        input: Bytes,
    ) -> Result<String> {
        // Generate a new request_id randomly using uuid
        let request_id = Uuid::new_v4().to_string();

        // Create a file `input.json` with { "input": input } and saves to to {request_id}_input.json
        let input_file = format!("{}_input.json", request_id);
        let input_data = serde_json::to_string(&json_macro!({ "input": input }))?;
        fs::write(&input_file, input_data)?;

        // Read prove_binary and wrapper_binary from the .env (panic if not present)
        let prove_binary = env::var("PROVE_BINARY").expect("PROVE_BINARY not found in .env");
        let wrapper_binary = env::var("WRAPPER_BINARY").expect("WRAPPER_BINARY not found in .env");

        // Execute the command
        let output = Command::new(&prove_binary)
            .args(&["prove", &input_file, "--wrapper-path", &wrapper_binary])
            .env("LOCAL_PROVER", "true")
            .env("RUST_LOG", "info")
            .output()?;

        // Check for command execution error
        if !output.status.success() {
            error!("Command execution failed");
            return Err(Error::msg("Failed to execute prove command."));
        }

        // The proof should be located at output.json (panic if not present)
        let proof_data = fs::read_to_string("output.json")?;

        // Parse proof data
        let proof_json: serde_json::Value = serde_json::from_str(&proof_data)?;
        let proof = proof_json
            .get("data")
            .and_then(|d| d.get("proof"))
            .ok_or_else(|| Error::msg("Proof not found in output.json"))?
            .to_string();
        let output_value = proof_json
            .get("data")
            .and_then(|d| d.get("output"))
            .ok_or_else(|| Error::msg("Output not found in output.json"))?
            .to_string();

        // Save to {request_id}.json
        let output_file = format!("{}.json", request_id);
        let final_data = json_macro!({
            "chain_id": chain_id,
            "to": to,
            "calldata": calldata,
            "function_id": function_id,
            "input": input,
            "proof": proof,
            "output": output_value,
        });
        fs::write(&output_file, serde_json::to_string(&final_data)?)?;

        // Delete output.json file
        fs::remove_file("output.json")?;

        Ok(request_id)
    }

    /// Submit an offchain request to the Succinct X API.
    pub async fn submit_platform_request(
        &self,
        chain_id: u32,
        to: Address,
        calldata: Bytes,
        function_id: B256,
        input: Bytes,
    ) -> Result<String> {
        if self.base_url == "local" {
            return self.local_mode(chain_id, to, calldata.clone(), function_id, input.clone());
        }

        let data = OffchainInput {
            chainId: chain_id,
            to,
            data: calldata,
            functionId: function_id,
            input,
        };

        // Serialize the data to JSON.
        let serialized_data = serde_json::to_string(&data).unwrap();

        // Make off-chain request.
        let request_url = format!("{}{}", self.base_url, "/request/new");
        let res = self
            .client
            .post(request_url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.api_key.clone())
            .body(serialized_data)
            .send()
            .await
            .unwrap();

        // Check if the request was successful.
        if res.status().is_success() {
            info!("Request successful!");
            let response: OffchainRequestResponse = res.json().await.unwrap();
            Ok(response.request_id)
        } else {
            error!("Request failed!");
            Err(Error::msg("Failed to submit request to Succinct X API."))
        }
    }
}
