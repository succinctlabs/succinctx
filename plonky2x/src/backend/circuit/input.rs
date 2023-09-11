use itertools::Itertools;
use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::{Deserialize, Serialize};

use super::PlonkParameters;
use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::{EvmVariable, ValueStream};
use crate::prelude::{ByteVariable, CircuitVariable};

/// Public inputs to the circuit. In the form of bytes, field elements, or child proofs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInput<L: PlonkParameters<D>, const D: usize> {
    Bytes(Vec<u8>),
    Elements(Vec<L::Field>),
    RecursiveProofs(Vec<ProofWithPublicInputs<L::Field, L::Config, D>>),
    None(),
}

impl<L: PlonkParameters<D>, const D: usize> PublicInput<L, D> {
    /// Creates an empty public input instance.
    pub fn new(io: &CircuitIO<D>) -> Self {
        match io {
            CircuitIO::Bytes(_) => PublicInput::Bytes(vec![]),
            CircuitIO::Elements(_) => PublicInput::Elements(vec![]),
            CircuitIO::RecursiveProofs(_) => PublicInput::RecursiveProofs(vec![]),
            CircuitIO::None() => PublicInput::None(),
        }
    }

    /// Create a public input instance with data from the proof with public inputs.
    pub fn from_proof_with_pis(
        io: &CircuitIO<D>,
        proof_with_pis: &ProofWithPublicInputs<L::Field, L::Config, D>,
    ) -> Self {
        match io {
            CircuitIO::Bytes(io) => {
                let offset = ByteVariable::nb_elements() * io.input.len();
                let elements = proof_with_pis.public_inputs[..offset].to_vec();
                let mut stream = ValueStream::<L, D>::from_values(elements);
                let bytes = (0..io.input.len())
                    .map(|_| stream.read_value::<ByteVariable>())
                    .collect_vec();
                PublicInput::Bytes(bytes)
            }
            CircuitIO::Elements(io) => {
                let offset = io.input.len();
                let elements = proof_with_pis.public_inputs[..offset].to_vec();
                PublicInput::Elements(elements)
            }
            CircuitIO::RecursiveProofs(_) => {
                todo!()
            }
            CircuitIO::None() => PublicInput::None(),
        }
    }

    /// Writes a value to the public circuit input using field-based serialization.
    pub fn write<V: CircuitVariable>(&mut self, value: V::ValueType<L::Field>) {
        match self {
            PublicInput::Elements(input) => {
                input.extend(V::elements::<L, D>(value));
            }
            _ => panic!("field io is not enabled"),
        };
    }

    /// Writes a slice of field elements to the public circuit input.
    pub fn write_all(&mut self, value: &[L::Field]) {
        match self {
            PublicInput::Elements(input) => {
                input.extend(value);
            }
            _ => panic!("field io is not enabled"),
        };
    }

    /// Writes a value to the public circuit input using byte-based serialization (i.e., abi
    /// encoded types).
    pub fn evm_write<V: EvmVariable>(&mut self, value: V::ValueType<L::Field>) {
        match self {
            PublicInput::Bytes(input) => {
                let bytes = V::encode_value(value);
                input.extend(bytes);
            }
            _ => panic!("evm io is not enabled"),
        };
    }

    /// Writes a stream of bytes to the public circuit input. Assumes that the bytes can be
    /// properly deserialized.
    pub fn evm_write_all(&mut self, bytes: &[u8]) {
        match self {
            PublicInput::Bytes(input) => {
                input.extend(bytes);
            }
            _ => panic!("evm io is not enabled"),
        };
    }

    /// Sets a value to the circuit input. This method only works if the circuit is using
    /// field element-based IO.
    pub fn set<V: CircuitVariable>(&mut self, _: V, _: V::ValueType<L::Field>) {
        todo!()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;

    use crate::backend::circuit::input::PublicInput;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::{ByteVariable, DefaultBuilder, Variable};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_serialize_input_bytes() {
        let mut builder = DefaultBuilder::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        let circuit = builder.build();

        let mut input = circuit.inputs();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(7u8);

        let json = serde_json::to_string(&input).unwrap();
        assert_eq!(r#"{"bytes":"0007"}"#, json);

        let deserialized_input = serde_json::from_str::<PublicInput<L, D>>(&json).unwrap();
        assert_eq!(input, deserialized_input);
    }

    #[test]
    fn test_serialize_input_elements() {
        let mut builder = DefaultBuilder::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);
        let circuit = builder.build();

        let mut input = circuit.inputs();
        input.write::<Variable>(GoldilocksField::TWO);

        let json = serde_json::to_string(&input).unwrap();
        assert_eq!(r#"{"elements":{"input":["2"]}}"#, json);

        let deserialized_input = serde_json::from_str::<PublicInput<L, D>>(&json).unwrap();
        assert_eq!(input, deserialized_input);
    }
}
