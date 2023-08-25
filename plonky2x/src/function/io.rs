use plonky2::hash::hash_types::RichField;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInput {
    io: String,
    bytes: Option<String>,
    elements: Option<Vec<u64>>,
}

impl FunctionInput {
    pub fn bytes(&self) -> Vec<u8> {
        let bytes = self.bytes.as_ref().unwrap();
        hex::decode(bytes).unwrap()
    }

    pub fn elements<F: RichField>(&self) -> Vec<F> {
        let elements = self.elements.as_ref().unwrap();
        elements.iter().map(|e| F::from_canonical_u64(*e)).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionOutput {
    pub io: String,
    pub bytes: Option<String>,
    pub elements: Option<Vec<u64>>,
    pub proof: Vec<u8>,
}
