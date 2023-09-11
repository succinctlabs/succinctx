use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::backend::circuit::input::{BytesInput, ElementsInput, RecursiveProofsInput};
use crate::backend::config::PlonkParameters;

/// Common fields in all `FunctionRequest` types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRequestWrapper<D> {
    #[serde(rename = "releaseId")]
    pub release_id: String,
    pub data: D,
}

/// A request to generate a proof for a function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(bound = "")]
pub enum FunctionRequest<L: PlonkParameters<D>, const D: usize> {
    #[serde(rename = "req_bytes")]
    Bytes(FunctionRequestWrapper<BytesInput>),
    #[serde(rename = "req_elements")]
    Elements(FunctionRequestWrapper<ElementsInput<L, D>>),
    #[serde(rename = "req_recursiveProofs")]
    RecursiveProofs(FunctionRequestWrapper<RecursiveProofsInput<L, D>>),
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;

    use crate::backend::config::DefaultParameters;
    use crate::backend::function::io::FunctionRequest;
    use crate::prelude::{CircuitBuilderX, Variable};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_deserialize_function_request_bytes() {
        let json_str = r#"
        {
            "type": "req_bytes",
            "releaseId": "test",
            "data": {
                "input": "1234"
            }
        }
        "#;
        let deserialized: FunctionRequest<L, D> = serde_json::from_str(json_str).unwrap();
        println!("{:?}", deserialized);
    }

    #[test]
    fn test_deserialize_function_request_elements() {
        let json_str = r#"
        {
            "type": "req_elements",
            "releaseId": "test",
            "data": {
                "input": ["1234", "5678"]
            }
        }
        "#;
        let deserialized: FunctionRequest<L, D> = serde_json::from_str(json_str).unwrap();
        println!("{:?}", deserialized);
    }

    #[test]
    fn test_deserialize_function_request_recursive_proofs() {
        let mut builder = CircuitBuilderX::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        let circuit = builder.build();

        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        let (proof, _) = circuit.prove(&input);

        let proof_bytes = hex::encode(bincode::serialize(&proof).unwrap());
        let json_str = r#"
        {
            "type": "req_recursiveProofs",
            "releaseId": "test",
            "data": {
                "input": ["1234", "5678"],
                "proofs": ["PROOF_BYTES"]
            }
        }
        "#;
        let binding = &json_str.replace("PROOF_BYTES", &proof_bytes);

        let deserialized: FunctionRequest<L, D> = serde_json::from_str(binding).unwrap();
        println!("{:?}", deserialized);
    }
}
