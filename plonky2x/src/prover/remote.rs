use core::time::Duration;
use std::collections::HashMap;
use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use super::Prover;
use crate::mapreduce::serialize::CircuitDataSerializable;
use crate::vars::CircuitVariable;

#[derive(Serialize)]
pub struct CreateProofPayload {
    release_id: String,
    input: String,
    context: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProofResponse {
    pub proof_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetProofResponse {
    id: String,
    status: String,
    result: Option<HashMap<String, String>>,
}

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
        input: Vec<F>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let release_id = env::var("RELEASE_ID").unwrap();
        let circuit_id = circuit.id();

        let input = input
            .iter()
            .map(|x| format!("{}", x.to_canonical_u64()))
            .collect::<Vec<String>>()
            .join(",");
        let context = format!("map ./build/{}.circuit {}", circuit_id, input);

        let payload = CreateProofPayload {
            release_id,
            input,
            context,
        };
        let create_response: CreateProofResponse = self
            .client
            .post("https://platform.succinct.xyz:8080/api/proof/new")
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let mut response: GetProofResponse;
        loop {
            let proof_id = create_response.proof_id.clone();
            println!("Proof ID: {}", proof_id);
            response = self
                .client
                .get(&format!(
                    "https://platform.succinct.xyz:8080/api/proof/{}",
                    proof_id
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            if response.status == "success" {
                break;
            } else if response.status == "failure" {
                panic!("Proof failed");
            }
            sleep(Duration::from_secs(1)).await;
        }

        let proof_bytes = hex::decode(response.result.unwrap().get("bytes").unwrap()).unwrap();
        let proof =
            ProofWithPublicInputs::<F, C, D>::from_bytes(proof_bytes, &circuit.common).unwrap();
        proof
    }

    async fn prove_reduce<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        input: Vec<ProofWithPublicInputs<F, C, D>>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let release_id = env::var("RELEASE_ID").unwrap();
        let circuit_id = circuit.id();

        let input = input
            .iter()
            .map(|x| format!("{}", hex::encode(x.to_bytes())))
            .collect::<Vec<String>>()
            .join(",");
        let context = format!("reduce ./build/{}.circuit {}", circuit_id, input);

        let payload = CreateProofPayload {
            release_id,
            input,
            context,
        };
        let create_response: CreateProofResponse = self
            .client
            .post("https://platform.succinct.xyz:8080/api/proof/new")
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let mut response: GetProofResponse;
        loop {
            let proof_id = create_response.proof_id.clone();
            println!("Proof ID: {}", proof_id);
            response = self
                .client
                .get(&format!(
                    "https://platform.succinct.xyz:8080/api/proof/{}",
                    proof_id
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            if response.status == "success" {
                break;
            } else if response.status == "failure" {
                panic!("Proof failed");
            }
            sleep(Duration::from_secs(1)).await;
        }

        let proof_bytes = hex::decode(response.result.unwrap().get("bytes").unwrap()).unwrap();
        let proof =
            ProofWithPublicInputs::<F, C, D>::from_bytes(proof_bytes, &circuit.common).unwrap();
        proof
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use super::*;
    use crate::builder::CircuitBuilder;

    #[tokio::test]
    async fn test_create_proof() {
        // dotenv::dotenv().ok();

        // type F = GoldilocksField;
        // type C = PoseidonGoldilocksConfig;
        // const D: usize = 2;

        // let mut builder = CircuitBuilder::<F, D>::new();
        // let zero = builder.zero();
        // let one = builder.one();
        // let sum = builder.add(zero, one);
        // builder.assert_is_equal(sum, one);

        // let pw = PartialWitness::new();
        // let data = builder.build::<C>();

        // let remote_prover = RemoteProver::new();
        // let proof = remote_prover.prove(&data, pw).await;
        // println!("{:#?}", proof);
        // let result = client
        //     .create_proof(
        //         "56655a48-15c6-46dc-aec0-36c9fb47c4cb".to_string(),
        //         "0x".to_string(),
        //         "map-0xc47cba1a4dedd0a3e0fe.circuit map-0xc47cba1a4dedd0a3e0fe.target".to_string(),
        //     )
        //     .await
        //     .unwrap();
    }
}
