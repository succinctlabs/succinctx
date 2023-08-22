use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateProofPayload {
    release_id: String,
    input: String,
    context: String,
}

#[derive(Deserialize)]
pub struct CreateProofResponse {
    pub proof_id: String,
}

pub struct SuccinctClient {
    client: Client,
}

impl SuccinctClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn create_proof(
        &self,
        release_id: String,
        input: String,
        context: String,
    ) -> Result<CreateProofResponse, reqwest::Error> {
        let payload = CreateProofPayload {
            release_id,
            input,
            context,
        };
        let response: CreateProofResponse = self
            .client
            .post("https://platform.succinct.xyz:8080/api/proof/new")
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_proof() {
        let client = SuccinctClient::new();
        let result = client
            .create_proof(
                "56655a48-15c6-46dc-aec0-36c9fb47c4cb".to_string(),
                "0x".to_string(),
                "map-0xc47cba1a4dedd0a3e0fe.circuit map-0xc47cba1a4dedd0a3e0fe.target".to_string(),
            )
            .await
            .unwrap();
        println!("{}", result.proof_id);
    }
}
