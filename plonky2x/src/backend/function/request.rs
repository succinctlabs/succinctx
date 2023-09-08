use core::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesInput {
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementsInput {
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveProofsInput {
    pub subfunction: Option<String>,
    pub input: Vec<String>,
    pub proofs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRequestWrapper<D> {
    #[serde(rename = "releaseId")]
    pub release_id: String,
    pub data: D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FunctionRequest {
    #[serde(rename = "req_bytes")]
    Bytes(FunctionRequestWrapper<BytesInput>),
    #[serde(rename = "req_elements")]
    Elements(FunctionRequestWrapper<ElementsInput>),
    #[serde(rename = "req_recursiveProofs")]
    RecursiveProofs(FunctionRequestWrapper<RecursiveProofsInput>),
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::backend::function::request::FunctionRequest;

    #[test]
    fn test_function_request_deserialize() {
        let json_str = r#"
        {
            "type": "req_recursiveProofs",
            "releaseId": "some_release_id",
            "data": {
                "subfunction?": "main",
                "proofs": ["proof1", "proof2"],
                "input": ["input1", "input2"]
            }
        }
        "#;
        let deserialized: FunctionRequest = serde_json::from_str(json_str).unwrap();
        println!("{:?}", deserialized);
    }
}
