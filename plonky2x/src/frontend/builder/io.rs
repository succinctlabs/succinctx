use plonky2::plonk::proof::ProofWithPublicInputsTarget;

use super::CircuitBuilder;
use crate::backend::config::PlonkParameters;
use crate::frontend::vars::EvmVariable;
use crate::prelude::{ByteVariable, CircuitVariable, Variable};

/// Stores circuit variables used for reading and writing data to the EVM.
#[derive(Debug, Clone)]
pub struct EvmIO {
    pub input: Vec<ByteVariable>,
    pub output: Vec<ByteVariable>,
}

/// Stores circuit variable used for reading and writing data using field elements.
#[derive(Debug, Clone)]
pub struct FieldIO {
    pub input: Vec<Variable>,
    pub output: Vec<Variable>,
}

/// Stores circuit variables used for circuits with recursive proof inputs.
#[derive(Debug, Clone)]
pub struct RecursiveProofIO<const D: usize> {
    pub input: Vec<ProofWithPublicInputsTarget<D>>,
    pub output: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub enum CircuitIO<const D: usize> {
    Evm(EvmIO),
    Field(FieldIO),
    RecursiveProof(RecursiveProofIO<D>),
    None(),
}

impl<const D: usize> CircuitIO<D> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::None()
    }

    pub fn input(&self) -> Vec<Variable> {
        match self {
            Self::Evm(io) => io.input.iter().flat_map(|b| b.variables()).collect(),
            Self::Field(io) => io.input.clone(),
            Self::RecursiveProof(_) => todo!(),
            Self::None() => vec![],
        }
    }

    pub fn output(&self) -> Vec<Variable> {
        match self {
            Self::Evm(io) => io.output.iter().flat_map(|b| b.variables()).collect(),
            Self::Field(io) => io.output.clone(),
            Self::RecursiveProof(io) => io.output.clone(),
            Self::None() => vec![],
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn try_init_field_io(&mut self) {
        match self.io {
            CircuitIO::None() => {
                self.io = CircuitIO::Field(FieldIO {
                    input: Vec::new(),
                    output: Vec::new(),
                })
            }
            CircuitIO::Field(_) => {}
            _ => panic!("already set io type"),
        };
    }

    fn try_init_evm_io(&mut self) {
        match self.io {
            CircuitIO::None() => {
                self.io = CircuitIO::Evm(EvmIO {
                    input: Vec::new(),
                    output: Vec::new(),
                })
            }
            CircuitIO::Evm(_) => {}
            _ => panic!("already set io type"),
        };
    }

    pub fn read<V: CircuitVariable>(&mut self) -> V {
        self.try_init_field_io();
        let variable = self.init::<V>();
        match self.io {
            CircuitIO::Field(ref mut io) => io.input.extend(variable.variables()),
            _ => panic!("field io is not enabled"),
        }
        variable
    }

    pub fn evm_read<V: EvmVariable>(&mut self) -> V {
        self.try_init_evm_io();
        let nb_bytes = V::nb_bytes::<L, D>();
        let mut bytes = Vec::new();
        for _ in 0..nb_bytes {
            bytes.push(self.init::<ByteVariable>());
        }
        let variable = V::decode(self, bytes.as_slice());
        match self.io {
            CircuitIO::Evm(ref mut io) => io.input.extend(bytes),
            _ => panic!("evm io is not enabled"),
        }
        variable
    }

    pub fn write<V: CircuitVariable>(&mut self, variable: V) {
        self.try_init_field_io();
        match self.io {
            CircuitIO::Field(ref mut io) => io.output.extend(variable.variables()),
            _ => panic!("field io is not enabled"),
        }
    }

    pub fn evm_write<V: EvmVariable>(&mut self, variable: V) {
        self.try_init_evm_io();
        let bytes = variable.encode(self);
        match self.io {
            CircuitIO::Evm(ref mut io) => io.output.extend(bytes),
            _ => panic!("evm io is not enabled"),
        }
    }
}
