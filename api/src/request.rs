use ethers::types::{Address, Bytes, H256};
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
/// Data to be sent to the platform API with an offchain request.
pub struct OffchainInput {
    /// The chain id of the network to be used.
    pub chainId: u32,
    /// The address of the contract to call.
    to: Bytes,
    /// The calldata to be used in the contract call.
    data: Bytes,
    /// The Succinct X function id to be called.
    functionId: Bytes,
    /// The input to be used in the SuccinctX function call.
    input: Bytes,
}

/// Client to interact with the SuccinctX API.
pub struct SuccinctClient {
    client: Client,
    /// The base url for the SuccinctX API. (ex. https://alpha.succinct.xyz/api)
    base_url: String,
}

impl SuccinctClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Submit an offchain request to the Succinct X API.
    pub async fn submit_platform_request(
        &self,
        chain_id: u32,
        to: Address,
        calldata: Bytes,
        function_id: H256,
        input: Bytes,
    ) {
        let data = OffchainInput {
            chainId: chain_id,
            to: Bytes::from(to.as_fixed_bytes()),
            data: calldata,
            functionId: Bytes::from(function_id.as_fixed_bytes()),
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
