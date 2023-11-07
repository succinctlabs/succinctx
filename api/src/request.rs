use ethers::types::Bytes;
use log::{error, info};
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

/// Submit an offchain request to the platform API.
pub async fn submit_platform_request(data: OffchainInput, request_url: &str) {
    // Serialize the data to JSON.
    let serialized_data = serde_json::to_string(&data).unwrap();

    // Submit POST request to platform API.
    let client = reqwest::Client::new();
    let res = client
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
