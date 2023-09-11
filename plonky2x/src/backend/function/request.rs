use core::fmt::Debug;

use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::{Circuit, PlonkParameters, PublicInput};
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
pub struct ElementsRequestData<L: PlonkParameters<D>, const D: usize> {
    #[serde(serialize_with = "serialize_elements")]
    #[serde(deserialize_with = "deserialize_elements")]
    pub input: Vec<L::Field>,
}

/// Fields for a function request that uses recursive proofs io.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveProofsRequestData<L: PlonkParameters<D>, const D: usize> {
    pub subfunction: Option<String>,
    #[serde(serialize_with = "serialize_proof_with_pis_vec")]
    #[serde(deserialize_with = "deserialize_proof_with_pis_vec")]
    pub input: Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
}

/// Common fields for all function requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRequestBase<D> {
    #[serde(rename = "releaseId")]
    pub release_id: String,
    pub data: D,
}

/// The standard request format for running "functions".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(bound = "")]
pub enum FunctionRequest<L: PlonkParameters<D>, const D: usize> {
    #[serde(rename = "req_bytes")]
    Bytes(FunctionRequestBase<BytesRequestData>),
    #[serde(rename = "req_elements")]
    Elements(FunctionRequestBase<ElementsRequestData<L, D>>),
    #[serde(rename = "req_recursiveProofs")]
    RecursiveProofs(FunctionRequestBase<RecursiveProofsRequestData<L, D>>),
}

impl<L: PlonkParameters<D>, const D: usize> FunctionRequest<L, D> {
    /// Creates a new function request from a circuit and public input.
    pub fn new(circuit: &Circuit<L, D>, input: PublicInput<L, D>) -> Self {
        let release_id = circuit.id();
        match input {
            PublicInput::Bytes(input) => FunctionRequest::Bytes(FunctionRequestBase {
                release_id,
                data: BytesRequestData { input },
            }),
            PublicInput::Elements(input) => FunctionRequest::Elements(FunctionRequestBase {
                release_id,
                data: ElementsRequestData { input },
            }),
            PublicInput::RecursiveProofs(input) => {
                FunctionRequest::RecursiveProofs(FunctionRequestBase {
                    release_id,
                    data: RecursiveProofsRequestData {
                        subfunction: None,
                        input,
                    },
                })
            }
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
            FunctionRequest::Bytes(FunctionRequestBase { data, .. }) => {
                PublicInput::Bytes(data.input.clone())
            }
            FunctionRequest::Elements(FunctionRequestBase { data, .. }) => {
                PublicInput::Elements(data.input.clone())
            }
            FunctionRequest::RecursiveProofs(FunctionRequestBase { data, .. }) => {
                PublicInput::RecursiveProofs(data.input.clone())
            }
        }
    }
}
