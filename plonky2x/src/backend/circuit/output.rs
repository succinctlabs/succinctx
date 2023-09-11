use itertools::Itertools;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::{Deserialize, Serialize};

use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::{EvmVariable, ValueStream};
use crate::prelude::{ByteVariable, CircuitVariable};
use crate::utils::deserialize::{deserialize_elements, deserialize_hex};

/// An output to the circuit in the form of bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BytesOutput {
    #[serde(deserialize_with = "deserialize_hex")]
    pub output: Vec<u8>,
}

/// An output to the circuit in the form of field elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementsOutput<L: PlonkParameters<D>, const D: usize> {
    #[serde(deserialize_with = "deserialize_elements")]
    pub output: Vec<L::Field>,
}

/// An input to the circuit in the form of field elements and child proofs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecursiveProofsOutput<L: PlonkParameters<D>, const D: usize> {
    #[serde(deserialize_with = "deserialize_elements")]
    pub output: Vec<L::Field>,
}

/// An output from the circuit. Can either be in the form of bytes, field elements, or child proofs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PublicOutput<L: PlonkParameters<D>, const D: usize> {
    Bytes(BytesOutput),
    Elements(ElementsOutput<L, D>),
    RecursiveProofs(RecursiveProofsOutput<L, D>),
}

impl<L: PlonkParameters<D>, const D: usize> PublicOutput<L, D> {
    /// Gets the circuit output from the circuit io schema and the proof with public inputs.
    pub fn from_proof_with_pis(
        io: &CircuitIO<D>,
        proof_with_pis: &ProofWithPublicInputs<L::Field, L::Config, D>,
    ) -> Self {
        match io {
            CircuitIO::Bytes(io) => {
                let offset = ByteVariable::nb_elements() * io.input.len();
                let elements = proof_with_pis.public_inputs[offset..].to_vec();
                let mut stream = ValueStream::<L, D>::from_values(elements);
                let bytes = (0..io.output.len())
                    .map(|_| stream.read_value::<ByteVariable>())
                    .collect_vec();
                PublicOutput::Bytes(BytesOutput { output: bytes })
            }
            CircuitIO::Elements(io) => {
                let offset = io.input.len();
                let elements = proof_with_pis.public_inputs[offset..].to_vec();
                PublicOutput::Elements(ElementsOutput { output: elements })
            }
            CircuitIO::RecursiveProofs(_) => {
                todo!()
            }
            CircuitIO::None() => todo!(),
        }
    }

    /// Gets the circuit output from the circuit io schema and the filled witness.
    pub fn from_witness(io: &CircuitIO<D>, witness: &PartitionWitness<L::Field>) -> Self {
        match io {
            CircuitIO::Bytes(io) => {
                let output = io.output.iter().map(|b| b.get(witness)).collect_vec();
                PublicOutput::Bytes(BytesOutput { output })
            }
            CircuitIO::Elements(io) => {
                let output = io.output.iter().map(|v| v.get(witness)).collect_vec();
                PublicOutput::Elements(ElementsOutput { output })
            }
            CircuitIO::RecursiveProofs(_) => todo!(),
            CircuitIO::None() => todo!(),
        }
    }

    /// Reads a value from the public circuit output using field-based serialization.
    pub fn read<V: CircuitVariable>(&mut self) -> V::ValueType<L::Field> {
        match self {
            PublicOutput::Elements(c) => {
                let elements = c.output.drain(0..V::nb_elements()).collect_vec();
                V::from_elements::<L, D>(&elements)
            }
            _ => panic!("field io is not enabled"),
        }
    }

    /// Reads the entire stream of field elements from the public circuit output.
    pub fn read_all(&self) -> Vec<L::Field> {
        match self {
            PublicOutput::Elements(c) => c.output.clone(),
            _ => panic!("field io is not enabled"),
        }
    }

    /// Reads a value from the public circuit output using byte-based serialization.
    pub fn evm_read<V: EvmVariable>(&mut self) -> V::ValueType<L::Field> {
        match self {
            PublicOutput::Bytes(c) => {
                let nb_bytes = V::nb_bytes::<L, D>();
                let bytes = c.output.drain(0..nb_bytes).collect_vec();
                V::decode_value(bytes.as_slice())
            }
            _ => panic!("evm io is not enabled"),
        }
    }

    /// Reads the entire stream of bytes from the public circuit output.
    pub fn evm_read_all(&self) -> Vec<u8> {
        match self {
            PublicOutput::Bytes(c) => c.output.clone(),
            _ => panic!("evm io is not enabled"),
        }
    }

    /// Reads a value from the circuit output. It also can access the value of any intermediate
    /// variable in the circuit.
    pub fn get<V: CircuitVariable>(&self, _: V) -> V::ValueType<L::Field> {
        todo!()
    }
}
