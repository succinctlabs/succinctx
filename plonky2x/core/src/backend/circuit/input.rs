use itertools::Itertools;
use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::proof::ProofWithPublicInputs;
use serde::{Deserialize, Serialize};

use super::PlonkParameters;
use crate::backend::prover::ProofId;
use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::{EvmVariable, ValueStream};
use crate::prelude::{ByteVariable, CircuitVariable};

/// Public inputs to the circuit. In the form of bytes, field elements, or recursive proofs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInput<L: PlonkParameters<D>, const D: usize> {
    Bytes(Vec<u8>),
    Elements(Vec<L::Field>),
    RecursiveProofs(
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<L::Field>,
    ),
    RemoteRecursiveProofs(Vec<ProofId>),
    CyclicProof(
        Vec<L::Field>,
        Box<Option<ProofWithPublicInputs<L::Field, L::Config, D>>>,
        #[serde(skip)] Box<Option<VerifierCircuitData<L::Field, L::Config, D>>>,
    ),
    None(),
}

impl<L: PlonkParameters<D>, const D: usize> PublicInput<L, D> {
    /// Creates an empty public input instance.
    pub fn new(io: &CircuitIO<D>) -> Self {
        match io {
            CircuitIO::Bytes(_) => PublicInput::Bytes(vec![]),
            CircuitIO::Elements(_) => PublicInput::Elements(vec![]),
            CircuitIO::RecursiveProofs(_) => PublicInput::RecursiveProofs(vec![], vec![]),
            CircuitIO::CyclicProof(_) => {
                PublicInput::CyclicProof(vec![], Box::new(None), Box::new(None))
            }
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
            CircuitIO::CyclicProof(_) => {
                todo!()
            }
            CircuitIO::None() => PublicInput::None(),
        }
    }

    /// Writes a value to the public circuit input using field-based serialization.
    pub fn write<V: CircuitVariable>(&mut self, value: V::ValueType<L::Field>) {
        match self {
            PublicInput::Elements(input) => {
                input.extend(V::elements::<L::Field>(value));
            }
            PublicInput::RecursiveProofs(_, input) => {
                input.extend(V::elements::<L::Field>(value));
            }
            PublicInput::CyclicProof(input, _, _) => {
                input.extend(V::elements::<L::Field>(value));
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
            PublicInput::CyclicProof(input, _, _) => {
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

    /// Writes a proof to the public circuit input.
    pub fn proof_write(&mut self, proof: ProofWithPublicInputs<L::Field, L::Config, D>) {
        match self {
            PublicInput::RecursiveProofs(proof_input, _) => {
                proof_input.push(proof);
            }
            PublicInput::CyclicProof(_input, ref mut io_proof, ref _data) => {
                if io_proof.is_some() {
                    panic!("cyclic proof already has data");
                } else {
                    *io_proof = Box::new(Some(proof));
                }
            }
            _ => panic!("cyclic io is not enabled"),
        };
    }

    pub fn data_write(&mut self, data: VerifierCircuitData<L::Field, L::Config, D>) {
        match self {
            PublicInput::CyclicProof(_, _, ref mut io_data) => {
                if io_data.as_ref().is_some() {
                    panic!("cyclic proof already has data");
                } else {
                    let wrapped = VerifierCircuitData {
                        verifier_only: data.verifier_only,
                        common: data.common,
                    };
                    *io_data = Box::new(Some(wrapped));
                }
            }
            _ => panic!("cyclic io is not enabled"),
        };
    }

    /// Sets a value to the circuit input. This method only works if the circuit is using
    /// field element-based IO.
    pub fn set<V: CircuitVariable>(&mut self, _: V, _: V::ValueType<L::Field>) {
        todo!()
    }
}
