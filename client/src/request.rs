use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::{env, fs};

use alloy_primitives::{hex, Address, Bytes, B256};
use anyhow::{Error, Result};
use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::H160;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json as json_macro;
use uuid::Uuid;

use crate::utils::get_gateway_address;

// Note: Update ABI when updating contract.
abigen!(SuccinctGateway, "./abi/SuccinctGateway.abi.json");

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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
/// Proof data for a Succinct X function call.
/// This is the data that is returned from the Succinct X API.
struct SuccinctProofData {
    /// The chain id of the network to be used.
    chain_id: u32,
    /// The address of the contract to call.
    to: Address,
    /// The Succinct X function id to be called.
    function_id: B256,
    /// The calldata to be used in the contract call.
    calldata: Bytes,
    /// The input to be used in the Succinct X function call.
    input: Bytes,
    /// The proof for the Succinct X function call.
    proof: Bytes,
    /// The output of the Succinct X function call.
    output: Bytes,
}

#[derive(Serialize, Deserialize)]
/// Data received from the Succinct X API from an offchain request.
struct OffchainRequestResponse {
    request_id: String,
}

const LOCAL_PROOF_FOLDER: &str = "./proofs";

/// Client to interact with the Succinct X API.
pub struct SuccinctClient {
    /// HTTP client.
    client: Client,
    /// The base url for the Succinct X API. (ex. https://alpha.succinct.xyz/api)
    succinct_api_url: String,
    /// API key for the Succinct X API.
    succinct_api_key: String,
    /// Local prove mode flag.
    local_prove_mode: Option<bool>,
    /// Local relay mode flag.
    local_relay_mode: Option<bool>,
}

impl SuccinctClient {
    pub fn new(
        succinct_api_url: String,
        succinct_api_key: String,
        local_prove_mode: Option<bool>,
        local_relay_mode: Option<bool>,
    ) -> Self {
        if local_prove_mode == Some(true) {
            info!("Running SuccinctClient in local prover mode");
        }
        if local_relay_mode == Some(true) {
            info!("Running SuccinctClient in local relay mode");
        }
        // TODO: For now, if local_relay_mode is true, local_prove_mode must also be true. Once
        // local_relay_mode fetches from the Succinct X API, this can be removed.
        if local_relay_mode == Some(true) && local_prove_mode != Some(true) {
            panic!("local_relay_mode must be true if local_prove_mode is true")
        }

        Self {
            client: Client::new(),
            succinct_api_url,
            succinct_api_key,
            local_prove_mode,
            local_relay_mode,
        }
    }

    pub fn check_command_success(mut child: Child, error_msg: String) -> Result<(), Error> {
        // Check for command execution success
        let status = child.wait()?;
        if !status.success() {
            error!("Command execution failed");
            return Err(Error::msg(error_msg));
        }
        Ok(())
    }

    pub fn run_local_prover_docker_image(
        wrapper_binary: &str,
        prove_binary_dir: &str,
        prove_file_name: &str,
        input_file: &str,
    ) -> Result<(), Error> {
        let current_dir = env::current_dir()?;
        let current_dir_str = current_dir.to_str().unwrap();

        let mount_proofs_dir = format!("{}/proofs:/proofs", current_dir_str);
        let mount_prove_binary_dir = format!("{}/{}:/build", current_dir_str, prove_binary_dir);
        let mount_verifier_build_dir =
            format!("{}/{}:/verifier-build", current_dir_str, wrapper_binary);
        let mount_env_file = format!("{}/.env:/.env", current_dir_str);

        info!(
            "Running local prove command with Docker:\ndocker run --rm -it -v {} -v {} -v {} -v {} -e PROVE_FILE={} -e INPUT_FILE={} succinctlabs/succinct-local-prover",
            mount_proofs_dir, mount_prove_binary_dir, mount_verifier_build_dir, mount_env_file, prove_file_name, input_file
        );

        let prove = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-it",
                "-v",
                &mount_proofs_dir,
                "-v",
                &mount_prove_binary_dir,
                "-v",
                &mount_verifier_build_dir,
                "-v",
                &mount_env_file,
                "-e",
                format!("PROVE_FILE={}", prove_file_name).as_str(),
                "-e",
                format!("INPUT_FILE={}", input_file).as_str(),
                "succinctlabs/succinct-local-prover",
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        Self::check_command_success(prove, "Failed to execute prove command.".to_string())?;

        Ok(())
    }

    /// Generate and submit a local proof for a Succinct X function call.
    pub fn submit_local_request(
        &self,
        chain_id: u32,
        to: Address,
        calldata: Bytes,
        function_id: B256,
        input: Bytes,
    ) -> Result<String> {
        // Create the local proof directory
        let dir_path = Path::new(LOCAL_PROOF_FOLDER);

        // Create the directory if it does not exist
        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
            info!("Local proof folder created at {:?}", dir_path);
        } else {
            info!("Local proof folder already exists at {:?}", dir_path);
        }

        // Generate a new request_id randomly using uuid
        let request_id = Uuid::new_v4().to_string();

        // Create a file `input.json` with proof inputs and saves to to `proofs/{request_id}_input.json`
        let input_file = format!("{}/{}_input.json", LOCAL_PROOF_FOLDER, request_id);
        // The input_data should be of the type "ProofRequest" in plonky2x/core/src/backend/function/request.rs
        let input_data = serde_json::to_string(
            &json_macro!({ "type": "req_bytes", "releaseId": "", "parentId": "", "files": [], "data": { "input": input}  }),
        )?;
        fs::write(&input_file, input_data)?;

        // Read prove_binary and wrapper_binary from the .env file.
        let prove_binary_env_var = format!("PROVE_BINARY_{}", function_id);
        let prove_binary = env::var(&prove_binary_env_var).unwrap_or_else(|_| panic!("{} not found in .env. You must have this env variable set for every function_id you want to generate local proofs for.", prove_binary_env_var));
        let wrapper_binary = env::var("WRAPPER_BINARY").expect("WRAPPER_BINARY not found in .env");
        let prove_binary_dir = Path::new(&prove_binary).parent().unwrap_or_else(|| {
            panic!(
                "{} should be a file in a directory with all circuit artifacts",
                prove_binary_env_var
            )
        });
        let prove_file_name = Path::new(&prove_binary).file_name().unwrap_or_else(|| {
            panic!(
                "{} should be a file in a directory with all circuit artifacts",
                prove_binary_env_var
            )
        });

        // Generate the proof locally using the Succinct X local prover docker image.
        Self::run_local_prover_docker_image(
            &wrapper_binary,
            prove_binary_dir.to_str().unwrap(),
            prove_file_name.to_str().unwrap(),
            &input_file,
        )?;

        // The proof should be located at proofs/output.json.
        let proof_data = fs::read_to_string("./proofs/output.json")?;

        // Parse the proof data.
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

        // Save to proofs/output_{request_id}.json
        let output_file = format!("{}/output_{}.json", LOCAL_PROOF_FOLDER, request_id);
        let final_data = json_macro!({
            "chain_id": chain_id,
            "to": to,
            "calldata": calldata,
            "function_id": function_id,
            "input": input,
            "proof": proof,
            "output": output_value,
        });
        fs::write(output_file, serde_json::to_string(&final_data)?)?;

        info!(
            "Local proof generated successfully! Request ID: {}",
            request_id
        );

        // Delete the input and output.json file.
        fs::remove_file("./proofs/output.json")?;
        fs::remove_file(input_file)?;

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
        let request_url = format!("{}{}", self.succinct_api_url, "/request/new");
        let res = self
            .client
            .post(request_url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.succinct_api_key.clone())
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

    /// Submit a request to the Succinct X API.
    /// If in local prove mode, generates a local proof and returns the request_id after completion.
    pub async fn submit_request(
        &self,
        chain_id: u32,
        to: Address,
        calldata: Bytes,
        function_id: B256,
        input: Bytes,
    ) -> Result<String> {
        if Some(true) == self.local_prove_mode {
            return self.submit_local_request(
                chain_id,
                to,
                calldata.clone(),
                function_id,
                input.clone(),
            );
        }

        self.submit_platform_request(chain_id, to, calldata, function_id, input)
            .await
    }

    /// If in local relay mode, ethereum_rpc_url and wallet must be provided. If you wish to submit
    /// to your own gateway (ex. on a chain that doesn't have a canonical gateway), gateway_address
    /// must be provided.
    // TODO: Add support for hosted proving + local relaying.
    pub async fn relay_proof(
        &self,
        request_id: String,
        ethereum_rpc_url: Option<&str>,
        wallet: Option<LocalWallet>,
        gateway_address: Option<&str>,
    ) -> Result<()> {
        // If local mode, submit proof from local directory at proofs/output_{request_id}.json
        if Some(true) == self.local_relay_mode {
            let ethereum_rpc_url = ethereum_rpc_url
                .expect("Ethereum RPC URL must be provided when relaying a proof in local mode.");
            let wallet =
                wallet.expect("Local wallet must be provided when relaying a proof in local mode.");

            // Check if the proof file exists.
            let proof_file = format!("{}/output_{}.json", LOCAL_PROOF_FOLDER, request_id);
            if !Path::new(&proof_file).exists() {
                return Err(Error::msg(format!(
                    "Proof file {} does not exist.",
                    proof_file
                )));
            }

            // If it exists, attempt to submit the proof.
            let proof_data = fs::read_to_string(proof_file)?;
            let proof_json: serde_json::Value = serde_json::from_str(&proof_data)?;

            let succinct_proof_data: SuccinctProofData = serde_json::from_value(proof_json)?;

            let provider =
                Provider::<Http>::try_from(ethereum_rpc_url).expect("could not connect to client");
            let client = Arc::new(SignerMiddleware::new(provider, wallet));

            let address = get_gateway_address(succinct_proof_data.chain_id);
            // If gateway_address is provided, use that instead of the canonical gateway address.
            let gateway_address = gateway_address.or(address).expect(
                "Gateway address must be provided when relaying a proof in local mode
                if the chain does not have a canonical gateway address.",
            );

            let gateway_address_bytes: [u8; 20] =
                hex::decode(gateway_address).unwrap().try_into().unwrap();
            let contract = SuccinctGateway::new(H160::from(gateway_address_bytes), client);

            // Submit the proof to the Succinct X API.
            contract
                .fulfill_call(
                    succinct_proof_data.function_id.0,
                    ethers::types::Bytes(succinct_proof_data.input.0),
                    ethers::types::Bytes(succinct_proof_data.output.0),
                    ethers::types::Bytes(succinct_proof_data.proof.0),
                    H160(succinct_proof_data.to.0 .0),
                    ethers::types::Bytes(succinct_proof_data.calldata.0),
                )
                .await?;
        }
        Ok(())
    }
}
