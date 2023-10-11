use core::fmt::Debug;
use core::panic;

use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::{PlonkParameters, PublicOutput};
use crate::utils::serde::{
    deserialize_elements, deserialize_hex, deserialize_proof_with_pis, serialize_elements,
    serialize_hex, serialize_proof_with_pis,
};

/// Fields for a function result that uses bytes io.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesResultData {
    #[serde(serialize_with = "serialize_hex")]
    #[serde(deserialize_with = "deserialize_hex")]
    pub output: Vec<u8>,
    #[serde(serialize_with = "serialize_hex")]
    #[serde(deserialize_with = "deserialize_hex")]
    pub proof: Vec<u8>,
}

/// Fields for a function result that uses field elements io.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementsResultData<L: PlonkParameters<D>, const D: usize> {
    pub output: Vec<L::Field>,
    #[serde(serialize_with = "serialize_proof_with_pis")]
    #[serde(deserialize_with = "deserialize_proof_with_pis")]
    pub proof: ProofWithPublicInputs<L::Field, L::Config, D>,
}

/// Fields for a function result that uses recursive proofs io.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveProofsResultData<L: PlonkParameters<D>, const D: usize> {
    #[serde(serialize_with = "serialize_elements")]
    #[serde(deserialize_with = "deserialize_elements")]
    pub output: Vec<L::Field>,
    #[serde(serialize_with = "serialize_proof_with_pis")]
    #[serde(deserialize_with = "deserialize_proof_with_pis")]
    pub proof: ProofWithPublicInputs<L::Field, L::Config, D>,
}

/// Common fields for all function results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResultBase<D> {
    pub data: D,
}

/// The standard result format for "functions".
///
/// Note that this is a standard enforced by the remote provers. Locally, you can just use
/// `let (proof, output) = circuit.prove(input)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(bound = "")]
pub enum ProofResult<L: PlonkParameters<D>, const D: usize> {
    #[serde(rename = "res_bytes")]
    Bytes(ProofResultBase<BytesResultData>),
    #[serde(rename = "res_elements")]
    Elements(ProofResultBase<ElementsResultData<L, D>>),
    #[serde(rename = "res_recursiveProofs")]
    RecursiveProofs(ProofResultBase<RecursiveProofsResultData<L, D>>),
}

impl<L: PlonkParameters<D>, const D: usize> ProofResult<L, D> {
    /// Creates a new function result from a proof and output.
    pub fn from_proof_output(
        proof: ProofWithPublicInputs<L::Field, L::Config, D>,
        output: PublicOutput<L, D>,
    ) -> Self {
        match output {
            PublicOutput::Bytes(output) => {
                let data = BytesResultData {
                    output,
                    proof: bincode::serialize(&proof).unwrap(),
                };
                ProofResult::Bytes(ProofResultBase { data })
            }
            PublicOutput::Elements(output) => {
                let data = ElementsResultData { output, proof };
                ProofResult::Elements(ProofResultBase { data })
            }
            PublicOutput::Proofs(output) => {
                let data = RecursiveProofsResultData { output, proof };
                ProofResult::RecursiveProofs(ProofResultBase { data })
            }
            PublicOutput::None() => todo!(),
        }
    }

    pub fn from_bytes(proof: Vec<u8>, output: Vec<u8>) -> Self {
        let data = BytesResultData { output, proof };
        ProofResult::Bytes(ProofResultBase { data })
    }

    pub fn as_proof_and_output(
        &self,
    ) -> (
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    ) {
        match self {
            ProofResult::Elements(result) => {
                let proof = &result.data.proof;
                let output = PublicOutput::Elements(result.data.output.clone());
                (proof.clone(), output)
            }
            ProofResult::RecursiveProofs(result) => {
                let proof = &result.data.proof;
                let output = PublicOutput::Proofs(result.data.output.clone());
                (proof.clone(), output)
            }
            _ => panic!("cannot convert to proof and output"),
        }
    }
}
