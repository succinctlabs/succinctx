use core::time::Duration;
use std::env;
use std::net::ToSocketAddrs;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::debug;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use rand::Rng;
use reqwest::Client;
use tokio::time::sleep;

use super::ProverOutput;
use crate::backend::circuit::{PlonkParameters, PublicInput};
use crate::backend::function::ProofRequest;
use crate::backend::prover::service::{ProofRequestStatus, ProofService};
use crate::backend::prover::ProverOutputs;

/// A prover that generates proofs remotely on another machine.
#[derive(Debug, Clone, Default)]
pub struct RemoteProver {
    pub client: Client,
}

impl RemoteProver {
    pub fn new() -> Self {
        let proof_service_url = env::var("PROOF_SERVICE_URL").unwrap();
        let host = &proof_service_url.split("://").last().unwrap();
        let sock_addrs = format!("{}:443", host)
            .to_socket_addrs()
            .unwrap()
            .collect::<Vec<_>>();
        Self {
            client: Client::builder()
                .resolve_to_addrs(host, &sock_addrs)
                .build()
                .unwrap(),
        }
    }

    pub async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit_id: &str,
        input: &PublicInput<L, D>,
    ) -> Result<ProverOutput<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        debug!("prove: circuit_id={}", circuit_id);

        // Initialize the proof service.
        let service = ProofService::new_from_env();

        // Submit the proof request.
        let mut rng = rand::thread_rng();
        let sleep_time = rng.gen_range(0..=5000);
        sleep(Duration::from_millis(sleep_time)).await;
        let request = ProofRequest::new(circuit_id, input);
        let proof_id = service
            .submit::<L, D>(request)
            .expect("failed to submit proof request");

        // Wait for the proof to be generated.
        const MAX_RETRIES: usize = 500;
        let mut status = ProofRequestStatus::Pending;
        for i in 0..MAX_RETRIES {
            sleep(Duration::from_secs(10)).await;
            let request = service.get::<L, D>(proof_id)?;
            debug!(
                "proof {:?}: status={:?}, nb_retries={}/{}",
                proof_id,
                request.status,
                i + 1,
                MAX_RETRIES
            );

            status = request.status;
            match request.status {
                ProofRequestStatus::Pending => {}
                ProofRequestStatus::Running => {}
                ProofRequestStatus::Success => {
                    let (proof, output) = request.result.unwrap().as_proof_and_output();
                    return Ok(ProverOutput::Local(proof, output));
                }
                _ => break,
            };
        }

        // Return an error if the proof failed to generate.
        Err(anyhow!(
            "could not generate proof {:?}: status={:?}",
            proof_id,
            status
        ))
    }

    pub async fn batch_prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit_id: &str,
        inputs: &[PublicInput<L, D>],
    ) -> Result<ProverOutputs<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        // Initialize the proof service.
        let service = ProofService::new_from_env();

        // Submit the batch proof request.
        let requests = inputs
            .iter()
            .map(|input| ProofRequest::new(circuit_id, input))
            .collect_vec();
        let (batch_id, proof_ids) = service.submit_batch(&requests)?;

        const MAX_RETRIES: usize = 1500;
        for _ in 0..MAX_RETRIES {
            sleep(Duration::from_secs(10)).await;
            let request = match service.get_batch::<L, D>(batch_id) {
                Ok(request) => request,
                Err(e) => {
                    debug!("proof batch {:?}: error={:?}", batch_id, e);
                    continue;
                }
            };
            request.statuses.iter().for_each(|(status, count)| {
                debug!(
                    "proof batch {:?}: status={:?}, count={}",
                    batch_id, status, count
                );
            });
            if let Some(failed) = request.statuses.get(&ProofRequestStatus::Failure) {
                if *failed > 0 {
                    let count = request
                        .statuses
                        .get(&ProofRequestStatus::Success)
                        .unwrap_or(&0);
                    return Err(anyhow!(
                        "batch proof failed: nb_failed={}, nb_success={}",
                        failed,
                        *count,
                    ));
                }
            } else if request.statuses.len() == 1
                && request.statuses.contains_key(&ProofRequestStatus::Success)
            {
                return Ok(ProverOutputs::Remote(proof_ids));
            }
        }

        // Return an error if the proof failed to generate.
        Err(anyhow!("could not generate proof {:?}", batch_id,))
    }
}
