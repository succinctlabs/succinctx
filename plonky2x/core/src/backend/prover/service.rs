use core::fmt::Debug;
use core::time::Duration;
use std::collections::HashMap;
use std::env;
use std::net::ToSocketAddrs;

use anyhow::Result;
use log::{debug, trace};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::circuit::PlonkParameters;
use crate::backend::function::{ProofRequest, ProofResult};

/// The endpoint for submitting a proof request.
const SUBMIT_PROOF_REQUEST_ROUTE: &str = "/api/proof/new";

/// The endpoint for submitting a batch of proof requests.
const SUBMIT_PROOF_BATCH_REQUEST_ROUTE: &str = "/api/proof/batch/new";

/// The endpoint for getting the status of a proof request.
const GET_PROOF_REQUEST_ROUTE: &str = "/api/proof";

/// The endpoint for getting the status of a proof request.
const GET_PROOF_BATCH_REQUEST_ROUTE: &str = "/api/proof/batch/status";

/// A UUID V4 identifer for a proof request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProofId(pub Uuid);

/// A UUID V4 identifer for a proof request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchProofId(pub Uuid);

/// The status of a proof request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProofRequestStatus {
    Pending,
    Running,
    Failure,
    Success,
    Cancelled,
    Timeout,
    Requested,
}

/// The response from submitting a proof request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitProofRequestResponse {
    pub proof_id: ProofId,
}

/// The response from submitting a batch of proof requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitProofBatchRequestResponse {
    pub proof_batch_id: BatchProofId,
    pub proof_ids: Vec<ProofId>,
}

/// The response from getting a proof.
#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "")]
pub struct GetProofRequestResponse<L: PlonkParameters<D>, const D: usize> {
    pub id: ProofId,
    pub status: ProofRequestStatus,
    pub result: Option<ProofResult<L, D>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "")]
pub struct GetProofBatchRequestResponse {
    pub statuses: HashMap<ProofRequestStatus, u64>,
}

/// A client for connecting to the proof service which can generate proofs remotely.
#[derive(Debug, Clone)]
pub struct ProofService {
    client: Client,
    base_url: String,
}

impl ProofService {
    pub fn new_from_env() -> Self {
        let service_url = env::var("PROOF_SERVICE_URL").unwrap();
        ProofService::new(service_url)
    }

    /// Creates a new instance of the function service client.
    pub fn new(url: String) -> Self {
        let host = url.split("://").last().unwrap();
        let sock_addrs = format!("{}:443", host)
            .to_socket_addrs()
            .unwrap()
            .collect::<Vec<_>>();
        Self {
            client: Client::builder()
                .resolve_to_addrs(host, &sock_addrs)
                .build()
                .unwrap(),
            base_url: url,
        }
    }

    /// Sends a GET request to the given route.
    fn get_json<O>(&self, route: &str) -> Result<O>
    where
        O: DeserializeOwned,
    {
        let endpoint = format!("{}{}", self.base_url, route);
        trace!("sending get request: url={}", endpoint);
        self.client
            .get(endpoint)
            .timeout(Duration::from_secs(300))
            .send()?
            .json()
            .map_err(|e| e.into())
    }

    /// Sends a POST request to the given route with the given input serialized as JSON.
    fn post_json<I, O>(&self, route: &str, input: I) -> Result<O>
    where
        I: Debug + Serialize + Sized,
        O: DeserializeOwned,
    {
        let endpoint = format!("{}{}", self.base_url, route);
        trace!("sending post request: url={}, input={:?}", endpoint, input);
        let response = self
            .client
            .post(endpoint)
            .timeout(Duration::from_secs(300))
            .json(&input)
            .send()?;
        let text = response.text()?;
        debug!("response: {:?}", text);
        Ok(serde_json::from_str(&text).unwrap())
    }

    /// Submits a request for the service to generate a proof. Returns the proof id.
    pub fn submit<L: PlonkParameters<D>, const D: usize>(
        &self,
        request: ProofRequest<L, D>,
    ) -> Result<ProofId> {
        let response: SubmitProofRequestResponse =
            self.post_json(SUBMIT_PROOF_REQUEST_ROUTE, request)?;
        Ok(response.proof_id)
    }

    /// Submits a batch of requests for the service to generate proofs.
    pub fn submit_batch<L: PlonkParameters<D>, const D: usize>(
        &self,
        requests: &[ProofRequest<L, D>],
    ) -> Result<(BatchProofId, Vec<ProofId>)> {
        let response: SubmitProofBatchRequestResponse =
            self.post_json(SUBMIT_PROOF_BATCH_REQUEST_ROUTE, requests)?;
        Ok((response.proof_batch_id, response.proof_ids))
    }

    /// Gets the status of a proof request with the given proof id.
    pub fn get<L: PlonkParameters<D>, const D: usize>(
        &self,
        id: ProofId,
    ) -> Result<GetProofRequestResponse<L, D>> {
        self.get_json(&format!("{}/{}", GET_PROOF_REQUEST_ROUTE, id.0))
    }

    /// Gets the status of a proof request with the given proof id.
    pub fn get_batch<L: PlonkParameters<D>, const D: usize>(
        &self,
        id: BatchProofId,
    ) -> Result<GetProofBatchRequestResponse> {
        self.get_json(&format!("{}/{}", GET_PROOF_BATCH_REQUEST_ROUTE, id.0))
    }
}
