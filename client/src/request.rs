use std::path::Path;
use std::process::{Child, Command, Stdio};
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

const LOCAL_PROOF_FOLDER: &str = "./proofs";
const LOCAL_STRING: &str = "local";

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
        if base_url == LOCAL_STRING {
            info!("Running SuccinctClient in local mode");
        }
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    pub fn local_mode(&self) -> bool {
        self.base_url == LOCAL_STRING
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
        if self.local_mode() {
            return self.submit_local_request(
                chain_id,
                to,
                calldata.clone(),
                function_id,
                input.clone(),
            );
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
