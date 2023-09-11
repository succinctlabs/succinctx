use core::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesResult {
    pub proof: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementsResult {
    pub proof: String,
    pub output: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveProofsResult {
    pub proof: String,
    pub output: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResultWrapper<D> {
    pub data: D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FunctionResult {
    #[serde(rename = "res_bytes")]
    Bytes(FunctionResultWrapper<BytesResult>),
    #[serde(rename = "res_elements")]
    Elements(FunctionResultWrapper<ElementsResult>),
    #[serde(rename = "res_recursiveProofs")]
    RecursiveProofs(FunctionResultWrapper<RecursiveProofsResult>),
}
