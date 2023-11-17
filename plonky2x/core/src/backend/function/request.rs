use core::fmt::Debug;
use std::env;

use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::{PlonkParameters, PublicInput};
use crate::backend::prover::ProofId;
use crate::utils::serde::{
    deserialize_elements, deserialize_hex, deserialize_proof_with_pis_vec, serialize_elements,
    serialize_hex, serialize_proof_with_pis_vec,
};

/// Fields for a function request that uses bytes io.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesRequestData {
    #[serde(serialize_with = "serialize_hex")]
    #[serde(deserialize_with = "deserialize_hex")]
    pub input: Vec<u8>,
}

/// Fields for a function request that uses field elements io.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementsRequestData<L: PlonkParameters<D>, const D: usize> {
    pub circuit_id: String,
    #[serde(serialize_with = "serialize_elements")]
    #[serde(deserialize_with = "deserialize_elements")]
    pub input: Vec<L::Field>,
}

/// Fields for a function request that uses recursive proofs io.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecursiveProofsRequestData<L: PlonkParameters<D>, const D: usize> {
    pub circuit_id: String,
    #[serde(serialize_with = "serialize_proof_with_pis_vec")]
    #[serde(deserialize_with = "deserialize_proof_with_pis_vec")]
    pub proofs: Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
}

/// Fields for a function request that uses recursive proofs io but with remote proofs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteRecursiveProofsRequestData {
    pub circuit_id: String,
    pub proof_ids: Vec<ProofId>,
}

/// Common fields for all function requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofRequestBase<D> {
    pub release_id: String,
    pub files: Option<Vec<String>>,
    pub data: D,
}

/// The standard request format for running "functions".
///
/// Note that this is a standard enforced by the remote provers. Locally, you can just use
/// `let (proof, output) = circuit.prove(input)` to run your circuit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(bound = "")]
pub enum ProofRequest<L: PlonkParameters<D>, const D: usize> {
    #[serde(rename = "req_bytes")]
    Bytes(ProofRequestBase<BytesRequestData>),
    #[serde(rename = "req_elements")]
    Elements(ProofRequestBase<ElementsRequestData<L, D>>),
    #[serde(rename = "req_recursiveProofs")]
    RecursiveProofs(ProofRequestBase<RecursiveProofsRequestData<L, D>>),
    #[serde(rename = "req_remoteRecursiveProofs")]
    RemoteRecursiveProofs(ProofRequestBase<RemoteRecursiveProofsRequestData>),
}

impl<L: PlonkParameters<D>, const D: usize> ProofRequest<L, D> {
    /// Creates a new function request from a circuit and public input.
    pub fn new(circuit_id: &str, input: &PublicInput<L, D>) -> Self {
        let release_id = env::var("RELEASE_ID").unwrap();
        match input {
            PublicInput::Bytes(input) => ProofRequest::Bytes(ProofRequestBase {
                release_id,
                files: Some(vec![format!("main.circuit")]),
                data: BytesRequestData {
                    input: input.clone(),
                },
            }),
            PublicInput::Elements(input) => ProofRequest::Elements(ProofRequestBase {
                release_id,
                files: Some(vec![format!("{}.circuit", circuit_id)]),
                data: ElementsRequestData {
                    circuit_id: circuit_id.to_string(),
                    input: input.clone(),
                },
            }),
            PublicInput::RecursiveProofs(input) => {
                ProofRequest::RecursiveProofs(ProofRequestBase {
                    release_id,
                    files: Some(vec![format!("{}.circuit", circuit_id)]),
                    data: RecursiveProofsRequestData {
                        circuit_id: circuit_id.to_string(),
                        proofs: input.clone(),
                    },
                })
            }
            PublicInput::RemoteRecursiveProofs(input) => {
                ProofRequest::RemoteRecursiveProofs(ProofRequestBase {
                    release_id,
                    files: Some(vec![format!("{}.circuit", circuit_id)]),
                    data: RemoteRecursiveProofsRequestData {
                        circuit_id: circuit_id.to_string(),
                        proof_ids: input.clone(),
                    },
                })
            }
            PublicInput::CyclicProof(_) => todo!(),
            PublicInput::None() => todo!(),
        }
    }

    /// Loads a function request from a file.
    pub fn load(path: &String) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let rdr = std::io::BufReader::new(file);
        serde_json::from_reader(rdr).unwrap()
    }

    /// Gets the public input from the function request.
    pub fn input(&self) -> PublicInput<L, D> {
        match self {
            ProofRequest::Bytes(ProofRequestBase { data, .. }) => {
                PublicInput::Bytes(data.input.clone())
            }
            ProofRequest::Elements(ProofRequestBase { data, .. }) => {
                PublicInput::Elements(data.input.clone())
            }
            ProofRequest::RecursiveProofs(ProofRequestBase { data, .. }) => {
                PublicInput::RecursiveProofs(data.proofs.clone())
            }
            _ => panic!("invalid proof request type"),
        }
    }
}
