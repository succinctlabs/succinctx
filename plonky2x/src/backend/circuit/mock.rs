use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::generate_partial_witness;
use plonky2::iop::witness::{PartialWitness, PartitionWitness};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::{
    Buffer, GateSerializer, IoResult, Read, WitnessGeneratorSerializer, Write,
};

/// A mock circuit which can compute any function in the form `f(x)=y`.
#[derive(Debug)]
pub struct MockCircuit<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub data: MockCircuitData<F, C, D>,
    pub io: CircuitIO<D>,
}

impl MockCircuit<GoldilocksField, FastConfig, 2> {
    /// Creates a new mock circuit.
    pub fn fill_witness(&mut pw: PartialWitness<F>) -> Self {
        let res = fill_witness(pw, &self.data.prover_data, &self.data.common_data);
        match res {
            Ok(witness) => {
                let mut io = CircuitIO::new();
                io.set_witness(witness);
                Self { data, io }
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
