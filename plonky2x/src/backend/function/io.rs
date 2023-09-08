use core::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesData {
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementsData {
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveProofsData {
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
    Bytes(FunctionRequestWrapper<BytesData>),
    #[serde(rename = "req_elements")]
    Elements(FunctionRequestWrapper<ElementsData>),
    #[serde(rename = "req_recursiveProofs")]
    RecursiveProofs(FunctionRequestWrapper<RecursiveProofsData>),
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::backend::function::io::FunctionRequest;

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
