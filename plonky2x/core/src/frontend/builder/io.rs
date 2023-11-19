use log::debug;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{CommonCircuitData, VerifierCircuitTarget};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;

use serde::{Deserialize, Serialize};

use super::CircuitBuilder;
use crate::backend::circuit::{PlonkParameters, PublicInput};
use crate::frontend::vars::EvmVariable;
use crate::prelude::{ByteVariable, CircuitVariable, Variable};
use crate::utils::serde::{
    deserialize_proof_with_pis_target_option, deserialize_proof_with_pis_target_vec,
    deserialize_verifier_circuit_target_option, serialize_proof_with_pis_target_option,
    serialize_proof_with_pis_target_vec, serialize_verifier_circuit_target_option,
};

/// A schema for a circuit that uses bytes for input and output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesIO {
    pub input: Vec<ByteVariable>,
    pub output: Vec<ByteVariable>,
}

/// A schema for a circuit that uses field elements for input and output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementsIO {
    pub input: Vec<Variable>,
    pub output: Vec<Variable>,
}

/// A schema for a circuit that uses recursive proofs for inputs and field elements for outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveProofsIO<const D: usize> {
    #[serde(serialize_with = "serialize_proof_with_pis_target_vec")]
    #[serde(deserialize_with = "deserialize_proof_with_pis_target_vec")]
    pub input: Vec<ProofWithPublicInputsTarget<D>>,
    pub output: Vec<Variable>,
}

/// A schema for a circuit that uses recursive proofs for inputs and field elements for outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclicProofIO<const D: usize> {
    pub input: Vec<Variable>,
    #[serde(serialize_with = "serialize_proof_with_pis_target_option")]
    #[serde(deserialize_with = "deserialize_proof_with_pis_target_option")]
    pub proof: Option<ProofWithPublicInputsTarget<D>>,
    #[serde(serialize_with = "serialize_verifier_circuit_target_option")]
    #[serde(deserialize_with = "deserialize_verifier_circuit_target_option")]
    pub verifier_data: Option<VerifierCircuitTarget>,
    pub output: Vec<Variable>,
    #[serde(skip)]
    pub closed: bool,
}

/// A schema for what the inputs and outputs are for a circuit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircuitIO<const D: usize> {
    Bytes(BytesIO),
    Elements(ElementsIO),
    RecursiveProofs(RecursiveProofsIO<D>),
    CyclicProof(CyclicProofIO<D>),
    None(),
}

impl<const D: usize> CircuitIO<D> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::None()
    }

    pub fn input(&self) -> Vec<Variable> {
        match self {
            Self::Bytes(io) => io.input.iter().flat_map(|b| b.variables()).collect(),
            Self::Elements(io) => io.input.clone(),
            Self::RecursiveProofs(_) => todo!(),
            Self::CyclicProof(_) => todo!(),
            Self::None() => vec![],
        }
    }

    pub fn output(&self) -> Vec<Variable> {
        match self {
            Self::Bytes(io) => io.output.iter().flat_map(|b| b.variables()).collect(),
            Self::Elements(io) => io.output.clone(),
            Self::RecursiveProofs(io) => io.output.clone(),
            Self::CyclicProof(_) => todo!(),
            Self::None() => vec![],
        }
    }

    pub fn set_witness<L: PlonkParameters<D>>(
        &self,
        pw: &mut PartialWitness<L::Field>,
        input: &PublicInput<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        match self {
            CircuitIO::Bytes(io) => {
                let variables = &io.input;
                if let PublicInput::Bytes(input) = input {
                    for i in 0..variables.len() {
                        variables[i].set(pw, input[i]);
                    }
                } else {
                    panic!("circuit io type is bytes but circuit input is not")
                }
            }
            CircuitIO::Elements(io) => {
                let variables = &io.input;
                if let PublicInput::Elements(input) = input {
                    for i in 0..variables.len() {
                        variables[i].set(pw, input[i]);
                    }
                } else {
                    panic!("circuit io type is elements but circuit input is not")
                }
            }
            CircuitIO::RecursiveProofs(io) => {
                let proof_with_pis_targets = &io.input;
                if let PublicInput::RecursiveProofs(input) = input {
                    for i in 0..proof_with_pis_targets.len() {
                        pw.set_proof_with_pis_target(&proof_with_pis_targets[i], &input[i]);
                    }
                } else {
                    panic!("circuit io type is recursive proofs but circuit input is not")
                }
            }
            CircuitIO::CyclicProof(io) => {
                let variables = &io.input;
                if let PublicInput::CyclicProof(input, proof, verifier_data) = input {
                    for i in 0..variables.len() {
                        variables[i].set(pw, input[i]);
                    }
                    let proof_contents = proof.as_ref().unwrap();
                    let proof = io.proof.as_ref().unwrap();
                    pw.set_proof_with_pis_target(proof, proof_contents);
                    let verifier_data = verifier_data.clone().unwrap().materialize();
                    let verifier_data_target = io.verifier_data.as_ref().unwrap();
                    debug!(
                        "setting verifier data target {:?}",
                        &verifier_data.verifier_only
                    );
                    pw.set_verifier_data_target(
                        &verifier_data_target,
                        &verifier_data.verifier_only,
                    );
                } else {
                    panic!("circuit io type is cyclic but circuit input is not")
                }
            }
            CircuitIO::None() => {}
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn try_init_field_io(&mut self) {
        match self.io {
            CircuitIO::None() => {
                self.io = CircuitIO::Elements(ElementsIO {
                    input: Vec::new(),
                    output: Vec::new(),
                })
            }
            CircuitIO::Elements(_) => {}
            CircuitIO::CyclicProof(_) => {}
            _ => panic!("already set io type"),
        };
    }

    fn try_init_evm_io(&mut self) {
        match self.io {
            CircuitIO::None() => {
                self.io = CircuitIO::Bytes(BytesIO {
                    input: Vec::new(),
                    output: Vec::new(),
                })
            }
            CircuitIO::Bytes(_) => {}
            _ => panic!("already set io type"),
        };
    }

    fn try_init_proof_io(&mut self) {
        match self.io {
            CircuitIO::None() => {
                self.io = CircuitIO::RecursiveProofs(RecursiveProofsIO {
                    input: Vec::new(),
                    output: Vec::new(),
                })
            }
            CircuitIO::RecursiveProofs(_) => {}
            CircuitIO::CyclicProof(_) => {}
            _ => panic!("already set io type"),
        };
    }

    pub fn use_cyclic_recursion(&mut self) {
        match self.io {
            CircuitIO::None() => {
                self.io = CircuitIO::CyclicProof(CyclicProofIO {
                    input: Vec::new(),
                    output: Vec::new(),
                    proof: None,
                    verifier_data: None,
                    closed: false,
                })
            }
            CircuitIO::CyclicProof(_) => {}
            _ => panic!("other io used already"),
        };
    }

    pub fn close_cyclic_io(&mut self) {
        let verifier_data: Option<VerifierCircuitTarget>;
        match self.io {
            CircuitIO::CyclicProof(ref mut io) => {
                if io.closed {
                    panic!("cyclic io already closed");
                }
                io.closed = true;
                let input = io
                    .input
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                let output = io
                    .output
                    .iter()
                    .flat_map(|b| b.variables())
                    .collect::<Vec<_>>();
                self.register_public_inputs(input.as_slice());
                self.register_public_inputs(output.as_slice());
                verifier_data = Some(self.api.add_verifier_data_public_inputs());
            }
            _ => panic!("not using cyclic io"),
        }
        match self.io {
            CircuitIO::CyclicProof(ref mut io) => {
                io.verifier_data = verifier_data;
            }
            _ => panic!("not using cyclic io"),
        }
    }

    // @audit
    pub fn read<V: CircuitVariable>(&mut self) -> V {
        self.try_init_field_io();
        let variable = self.init::<V>();
        match self.io {
            CircuitIO::Elements(ref mut io) => io.input.extend(variable.variables()),
            CircuitIO::CyclicProof(ref mut io) => io.input.extend(variable.variables()),
            _ => panic!("field io is not enabled"),
        }
        variable
    }

    // @audit
    pub fn evm_read<V: EvmVariable>(&mut self) -> V {
        self.try_init_evm_io();
        let nb_bytes = V::nb_bytes::<L, D>();
        let mut bytes = Vec::new();
        for _ in 0..nb_bytes {
            bytes.push(self.init::<ByteVariable>());
        }
        let variable = V::decode(self, bytes.as_slice());
        match self.io {
            CircuitIO::Bytes(ref mut io) => io.input.extend(bytes),
            _ => panic!("evm io is not enabled"),
        }
        variable
    }

    // @audit
    pub fn proof_read(
        &mut self,
        data: &CommonCircuitData<L::Field, D>,
    ) -> ProofWithPublicInputsTarget<D> {
        self.try_init_proof_io();
        let proof = self.add_virtual_proof_with_pis(&data);
        match self.io {
            CircuitIO::RecursiveProofs(ref mut io) => {
                io.input.push(proof.clone());
            }
            CircuitIO::CyclicProof(ref mut io) => {
                if let Some(_) = io.proof {
                    panic!("proof already set");
                } else {
                    io.proof = Some(proof.clone());
                }
            }
            _ => panic!("proof io is not enabled"),
        }
        proof
    }

    pub fn read_verifier_data(&mut self) -> VerifierCircuitTarget {
        match self.io {
            CircuitIO::CyclicProof(ref mut io) => io.verifier_data.as_ref().unwrap().clone(),
            _ => panic!("cyclic proof io is not enabled"),
        }
    }

    // @audit
    pub fn write<V: CircuitVariable>(&mut self, variable: V) {
        self.try_init_field_io();
        match self.io {
            CircuitIO::Elements(ref mut io) => io.output.extend(variable.variables()),
            CircuitIO::CyclicProof(ref mut io) => io.output.extend(variable.variables()),
            _ => panic!("field io is not enabled"),
        }
    }

    // @audit
    pub fn evm_write<V: EvmVariable>(&mut self, variable: V) {
        self.try_init_evm_io();
        let bytes = variable.encode(self);
        match self.io {
            CircuitIO::Bytes(ref mut io) => io.output.extend(bytes),
            _ => panic!("evm io is not enabled"),
        }
    }

    // @audit
    pub fn proof_write<V: CircuitVariable>(&mut self, variable: V) {
        self.try_init_proof_io();
        match self.io {
            CircuitIO::RecursiveProofs(ref mut io) => io.output.extend(variable.variables()),
            _ => panic!("proof io is not enabled"),
        }
    }
}
