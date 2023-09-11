use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::backend::circuit::{BytesInput, ElementsInput, PlonkParameters, RecursiveProofsInput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRequestWrapper<D> {
    #[serde(rename = "releaseId")]
    pub release_id: String,
    pub data: D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FunctionRequest<L: PlonkParameters<D>, const D: usize> {
    #[serde(rename = "req_bytes")]
    Bytes(FunctionRequestWrapper<BytesInput>),
    #[serde(rename = "req_elements")]
    Elements(FunctionRequestWrapper<ElementsInput<L, D>>),
    #[serde(rename = "req_recursiveProofs")]
    RecursiveProofs(FunctionRequestWrapper<RecursiveProofsInput<L, D>>),
}
