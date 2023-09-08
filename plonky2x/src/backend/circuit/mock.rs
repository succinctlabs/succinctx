use itertools::Itertools;
use std::collections::HashMap;


use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::generate_partial_witness;
use plonky2::iop::witness::{PartialWitness, PartitionWitness};
use plonky2::plonk::circuit_data::{MockCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::{
    Buffer, GateSerializer, IoResult, Read, WitnessGeneratorSerializer, Write,
};

use super::witness::fill_witness;
use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitIO;
/// A compiled circuit which can compute any function in the form `f(x)=y`.
#[derive(Debug)]
pub struct MockCircuit<L: PlonkParameters<D>, const D: usize> {
    pub data: MockCircuitData<L::Field, L::Config, D>,
    pub io: CircuitIO<D>,
    pub debug_variables: HashMap<usize, String>,
}

impl<L: PlonkParameters<D>, const D: usize> MockCircuit<L, D> {
    /// Creates a new mock circuit.
    pub fn fill_witness(&mut self, pw: PartialWitness<L::Field>) {
        let res = fill_witness(pw, &self.data.prover_only, &self.data.common);
        match res {
            Ok(witness) => {
                let mut io: CircuitIO<D> = CircuitIO::new();
                // io.set_witness(witness);
                // Self { data, io }
            }
            Err(e) => {
                println!("failed to fill witness");
                // Use the debug mode
            }
        }
    }
}

//     let witness = generate_partial_witness(pw, &self.data.prover_only, &self.data.common);
//     let output_variables = if self.io.evm.is_some() {
//         self.io
//             .evm
//             .clone()
//             .unwrap()
//             .output_bytes
//             .into_iter()
//             .flat_map(|b| b.variables())
//             .collect()
//     } else if self.io.field.is_some() {
//         self.io.field.clone().unwrap().output_variables
//     } else {
//         vec![]
//     };
//     let output_buffer = output_variables
//         .iter()
//         .map(|v| v.get(&witness))
//         .collect_vec();
//     let output = CircuitOutput {
//         io: self.io.clone(),
//         buffer: output_buffer,
//     };
