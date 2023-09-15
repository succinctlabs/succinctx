use core::time::Duration;
use std::env;

use anyhow::{anyhow, Result};
use futures::future::join_all;
use itertools::Itertools;
use log::debug;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use reqwest::Client;
use tokio::time::sleep;

use super::Prover;
use crate::backend::circuit::{CircuitBuild, PlonkParameters, PublicInput, PublicOutput};
use crate::backend::function::ProofRequest;
use crate::backend::prover::service::{ProofRequestStatus, ProofService};

/// A prover that generates proofs remotely on another machine.
#[derive(Debug, Clone)]
pub struct RemoteProver {
    pub client: Client,
}

impl Prover for RemoteProver {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        input: &PublicInput<L, D>,
    ) -> Result<(
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        debug!("prove: circuit_id={}", circuit.id());

        // Initialize the proof service.
        let service_url = env::var("PROOF_SERVICE_URL").unwrap();
        let service = ProofService::new(service_url);

        // Submit the proof request.
        let request = ProofRequest::new(circuit, input);
        let proof_id = service
            .submit::<L, D>(request)
            .await
            .expect("failed to submit proof request");

        // Wait for the proof to be generated.
        const MAX_RETRIES: usize = 500;
        let mut status = ProofRequestStatus::Pending;
        for i in 0..MAX_RETRIES {
            sleep(Duration::from_secs(60)).await;
            let request = service.get::<L, D>(proof_id).await?;
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
                    return Ok(request.result.unwrap().as_proof_and_output())
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

    async fn batch_prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        inputs: &[PublicInput<L, D>],
    ) -> Result<(
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<PublicOutput<L, D>>,
    )>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        debug!(
            "batch_prove: circuit_id={}, nb_inputs={}",
            circuit.id(),
            inputs.len()
        );

        // Create a proof request for each input in parallel.
        let futures = inputs
            .iter()
            .map(|input| self.prove(circuit, input))
            .collect_vec();

        // Wait for all proofs to be generated.
        let results = join_all(futures).await;

        // Unzip the results.
        Ok(results.into_iter().map(|r| r.unwrap()).unzip())
    }
}
