use ethers::types::Bytes;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
/// Data to be sent to the platform API with an offchain request.
pub struct OffchainInput {
    /// The chain id of the network to be used.
    chainId: u32,
    /// The address of the contract to call.
    to: Bytes,
    /// The calldata to be used in the contract call.
    data: Bytes,
    /// The Succinct X function id to be called.
    functionId: Bytes,
    /// The input to be used in the Succinct X function call.
    input: Bytes,
}

/// Client to interact with the platform API.
pub struct PlatformClient {
    client: Client,
    /// The base url for the platform API. (ex. https://alpha.succinct.xyz/api)
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
