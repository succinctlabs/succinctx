use core::time::Duration;
use std::env;

use futures::future::join_all;
use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use super::service::{GetProofResponse, SuccinctService};
use super::{Prover, ProverInputTargets, ProverInputValues};
use crate::mapreduce::serialize::CircuitDataSerializable;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextData {
    pub circuit_id: String,
    pub input: Vec<String>,
    pub tag: String,
}

/// A prover that uses the Succinct remote prover to generate proofs. The built circuit must
/// already be uploaded to Succinct and be referenced via the enviroment variable `RELEASE_ID`.
pub struct RemoteProver {
    pub client: Client,
}

impl Prover for RemoteProver {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn prove<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        _: ProverInputTargets<D>,
        values: ProverInputValues<F, C, D>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        // Calculate create proof payload.
        let release_id = env::var("RELEASE_ID").expect("enviroment variable RELEASE_ID is not set");
        let circuit_id = circuit.id();
        let context = match values {
            ProverInputValues::Bytes(_) => {
                todo!()
            }
            ProverInputValues::FieldElements(elements) => {
                let elements = elements
                    .iter()
                    .map(|x| x.to_canonical_u64().to_string())
                    .collect_vec();
                let context = serde_json::to_string_pretty(&ContextData {
                    circuit_id: circuit_id.clone(),
                    input: elements,
                    tag: "map".to_string(),
                })
                .unwrap();
                context
            }
            ProverInputValues::Proofs(proofs) => {
                let proofs_base64 = proofs
                    .iter()
                    .map(|x| base64::encode(x.to_bytes()))
                    .collect_vec();
                let context = serde_json::to_string_pretty(&ContextData {
                    circuit_id: circuit_id.clone(),
                    input: proofs_base64,
                    tag: "reduce".to_string(),
                })
                .unwrap();
                context
            }
        };

        // Call the service to create a proof.
        let succinct = SuccinctService::new();
        let proof_id = succinct
            .create_proof(release_id, "0x".to_string(), context)
            .await;

        /// Wait up to 120 seconds for the proof to finish generating.
        const MAX_RETRIES: usize = 120;
        let mut response: GetProofResponse = GetProofResponse {
            id: "".to_string(),
            status: "".to_string(),
            result: None,
        };
        for _ in 0..MAX_RETRIES {
            response = succinct.get_proof(proof_id.clone()).await;
            if response.status == "success" {
                break;
            } else if response.status == "failure" {
                panic!("proof generation failed proof_id={}", response.id);
            }
            sleep(Duration::from_secs(1)).await;
        }

        // Check if the proof was generated successfully.
        if response.status != "success" {
            panic!("proof generation timed out proof_id={}", response.id);
        }

        println!("Proof generated successfully proof_id={}", response.id);

        // Deserialize the proof.
        let bytes = base64::decode(response.result.unwrap().get("bytes").unwrap()).unwrap();
        let proof = ProofWithPublicInputs::<F, C, D>::from_bytes(bytes, &circuit.common).unwrap();
        proof
    }

    async fn prove_batch<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        targets: ProverInputTargets<D>,
        values: Vec<ProverInputValues<F, C, D>>,
    ) -> Vec<ProofWithPublicInputs<F, C, D>>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let mut futures = Vec::new();
        for i in 0..values.len() {
            println!("Starting proof {}/{}.", i + 1, values.len());
            let future = self.prove(circuit, targets.clone(), values[i].clone());
            futures.push(future);
        }
        join_all(futures).await
    }
}
