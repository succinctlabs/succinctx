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
#[serde(tag = "type")]
pub enum FunctionResult {
    #[serde(rename = "res_bytes")]
    Bytes(BytesResult),
    #[serde(rename = "res_elements")]
    Elements(ElementsResult),
    #[serde(rename = "res_recursiveProofs")]
    RecursiveProofs(RecursiveProofsResult),
}

impl From<BytesResult> for FunctionResult {
    fn from(result: BytesResult) -> Self {
        FunctionResult::Bytes(result)
    }
}

impl From<ElementsResult> for FunctionResult {
    fn from(result: ElementsResult) -> Self {
        FunctionResult::Elements(result)
    }
}

impl From<RecursiveProofsResult> for FunctionResult {
    fn from(result: RecursiveProofsResult) -> Self {
        FunctionResult::RecursiveProofs(result)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::backend::function::request::FunctionRequest;

    #[test]
    fn test_function_request_deserialize() {
        let json_str = r#"
        {
            "type": "req_recursiveProofs",
            "data": {
                "proofs": "0xab",
                "input": ["1", "2"]
            }
        }
        "#;
        let deserialized: FunctionRequest = serde_json::from_str(json_str).unwrap();
        println!("{:?}", deserialized);
    }
}
