use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::circuit::PlonkParameters;
use crate::backend::function::{ProofRequest, ProofResult};

/// The endpoint for submitting a proof request.
const SUBMIT_PROOF_REQUEST_ROUTE: &str = "/api/proof/new";

/// The endpoint for getting the status of a proof request.
const GET_PROOF_REQUEST_ROUTE: &str = "/api/proof/{id}";

/// A UUID V4 identifer for a proof request.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ProofId(pub Uuid);

/// The status of a proof request.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProofRequestStatus {
    Pending,
    Running,
    Failure,
    Success,
    Cancelled,
    Timeout,
}

/// The response from submitting a proof request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitProofRequestResponse {
    pub proof_id: ProofId,
}

/// The response from getting a proof.
#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "")]
pub struct GetProofRequestResponse<L: PlonkParameters<D>, const D: usize> {
    pub id: ProofId,
    pub status: ProofRequestStatus,
    pub result: ProofResult<L, D>,
}

/// A client for connecting to the proof service which can generate proofs remotely.
#[derive(Debug, Clone)]
pub struct ProofService {
    client: Client,
    base_url: String,
}

impl ProofService {
    /// Creates a new instance of the function service client.
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            base_url: url,
        }
    }

    /// Sends a GET request to the given route.
    async fn get_json<O>(&self, route: &str) -> Result<O>
    where
        O: DeserializeOwned,
    {
        self.client
            .get(format!("{}/{}", self.base_url, route))
            .send()
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    /// Sends a POST request to the given route with the given input serialized as JSON.
    async fn post_json<I, O>(&self, route: &str, input: I) -> Result<O>
    where
        I: Serialize + Sized,
        O: DeserializeOwned,
    {
        self.client
            .post(format!("{}/{}", self.base_url, route))
            .json(&input)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    /// Submits a request for the service to generate a proof. Returns the proof id.
    pub async fn submit<L: PlonkParameters<D>, const D: usize>(
        &self,
        request: ProofRequest<L, D>,
    ) -> Result<ProofId> {
        let response: SubmitProofRequestResponse =
            self.post_json(SUBMIT_PROOF_REQUEST_ROUTE, request).await?;
        Ok(response.proof_id)
    }

    /// Gets the status of a proof request with the given proof id.
    pub async fn get<L: PlonkParameters<D>, const D: usize>(
        &self,
        id: ProofId,
    ) -> Result<GetProofRequestResponse<L, D>> {
        self.get_json(&format!("{}/{}", GET_PROOF_REQUEST_ROUTE, id.0.to_string()))
            .await
    }
}
