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

#[cfg(test)]
pub(crate) mod tests {
    use crate::backend::function::result::FunctionResult;

    #[test]
    fn test_function_request_deserialize() {
        let json_str = r#"
        {
            "type": "res_recursiveProofs",
            "data": {
                "proof": "ab",
                "output": ["1", "2"]
            }
        }
        "#;
        let deserialized: FunctionResult = serde_json::from_str(json_str).unwrap();
        println!("{:?}", deserialized);
    }
}
