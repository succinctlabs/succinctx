use ethers::types::Bytes;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
/// Struct to hold the data to be sent to the platform API.
/// chainId: The chain id of the chain to be used.
/// to: The address of the contract to call.
/// data: The calldata to be used in the contract call.
/// functionId: The Succinct X function id to be called.
/// input: The input to be used in the Succinct X function call.
pub struct OffchainInput {
    chainId: u32,
    to: Bytes,
    data: Bytes,
    functionId: Bytes,
    input: Bytes,
}

/// Client to interact with the platform API.
/// client: The reqwest client to be used.
/// base_url: The base url of the platform API. (ex. https://alpha.succinct.xyz/api)
pub struct PlatformClient {
    client: Client,
    base_url: String,
}

impl PlatformClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Submit an offchain request to the platform API.
    pub async fn submit_platform_request(&self, data: OffchainInput) {
        // Serialize the data to JSON.
        let serialized_data = serde_json::to_string(&data).unwrap();

        // Submit POST request to platform API.
        let request_url = format!("{}{}", self.base_url, "/request/new");
        let res = self
            .client
            .post(request_url)
            .header("Content-Type", "application/json")
            .body(serialized_data)
            .send()
            .await
            .unwrap();

        // Check if the request was successful.
        if res.status().is_success() {
            info!("Request successful!");
        } else {
            error!("Request failed!");
        }
    }
}
