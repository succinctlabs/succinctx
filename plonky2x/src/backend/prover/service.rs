use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::backend::function::request::FunctionRequest;
use crate::backend::function::result::FunctionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProofRequest(pub FunctionRequest);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProofResponse {
    pub proof_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProofResponse {
    pub id: String,
    pub status: String,
    pub result: Option<FunctionResult>,
}

#[derive(Default)]
pub struct ProvingService {
    client: Client,
    base_url: String,
}

impl ProvingService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://platform.succinct.xyz:8080".to_string(),
        }
    }

    /// Submits a request for the service to create a proof. The function returns the proof id.
    pub async fn create_proof(&self, request: FunctionRequest) -> String {
        let payload = CreateProofRequest(request);
        let create_response: CreateProofResponse = self
            .client
            .post(format!("{}/api/proof/new", self.base_url))
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        create_response.proof_id
    }

    /// Gets the status of a proof request with the given proof id.
    pub async fn get_proof(&self, proof_id: String) -> GetProofResponse {
        self.client
            .get(&format!("{}/api/proof/{}", self.base_url, proof_id))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
