use core::fmt::Debug;

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
pub struct FunctionResultBase<D> {
    pub data: D,
}

/// The standard result format for "functions".
///
/// Note that this is a standard enforced by the remote provers. Locally, you can just use
/// `let (proof, output) = circuit.prove(input)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(bound = "")]
pub enum FunctionResult<L: PlonkParameters<D>, const D: usize> {
    #[serde(rename = "res_bytes")]
    Bytes(FunctionResultBase<BytesResultData>),
    #[serde(rename = "res_elements")]
    Elements(FunctionResultBase<ElementsResultData<L, D>>),
    #[serde(rename = "res_recursiveProofs")]
    RecursiveProofs(FunctionResultBase<RecursiveProofsResultData<L, D>>),
}

impl<L: PlonkParameters<D>, const D: usize> FunctionResult<L, D> {
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
                FunctionResult::Bytes(FunctionResultBase { data })
            }
            PublicOutput::Elements(output) => {
                let data = ElementsResultData { output, proof };
                FunctionResult::Elements(FunctionResultBase { data })
            }
            PublicOutput::Proofs(output) => {
                let data = RecursiveProofsResultData { output, proof };
                FunctionResult::RecursiveProofs(FunctionResultBase { data })
            }
            PublicOutput::None() => todo!(),
        }
    }

    pub fn from_bytes(proof: Vec<u8>, output: Vec<u8>) -> Self {
        let data = BytesResultData { output, proof };
        FunctionResult::Bytes(FunctionResultBase { data })
    }
}
