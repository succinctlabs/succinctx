use std::collections::HashMap;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::generate_partial_witness;
use plonky2::iop::witness::{PartialWitness, PartitionWitness};
use plonky2::plonk::circuit_data::MockCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::{
    Buffer, GateSerializer, IoResult, Read, WitnessGeneratorSerializer, Write,
};

use super::witness::{fill_witness, FillWitnessError};
use crate::backend::circuit::CircuitInput;
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
    pub fn mock_prove_witness(
        &self,
        pw: PartialWitness<L::Field>,
    ) -> Result<PartitionWitness<L::Field>, FillWitnessError> {
        let res = fill_witness(pw, &self.data.prover_only, &self.data.common);
        match res {
            Ok(witness) => Ok(witness),
            Err(e) => {
                println!("failed to fill witness");
                // TODO: Use the debug information
                Err(e)
            }
        }
    }

    /// Returns an input instance for the circuit.
    pub fn input(&self) -> CircuitInput<L, D> {
        CircuitInput {
            io: self.io.clone(),
            buffer: Vec::new(),
        }
    }

    // TODO: below should take in `inputs` and return (witness, output)
    // It should be implemented using `mock_prove_witness`
    pub fn mock_prove(&mut self) {
        todo!()
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

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use crate::prelude::*;

    #[test]
    fn test_mock_circuit_mock_prove_witness() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        // Build your circuit.
        let mock_circuit = builder.mock_build();

        let mut pw = PartialWitness::new();
        a.set(&mut pw, GoldilocksField::TWO);
        b.set(&mut pw, GoldilocksField::TWO);
        let witness = mock_circuit.mock_prove_witness(pw).unwrap();

        let c_value = c.get(&witness);
        println!("{}", c_value);
    }

    #[test]
    fn test_mock_circuit_with_field_io() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);

        // Build your circuit.
        let mock_circuit = builder.mock_build();

        // Write to the circuit input.
        let mut input = mock_circuit.input();
        input.write::<Variable>(GoldilocksField::TWO);
        input.write::<Variable>(GoldilocksField::TWO);

        // // Generate a proof.
        // let (proof, mut output) = mock_circuit.prove(&input);

        // // Read output.
        // let sum = output.read::<Variable>();
        // println!("{}", sum.0);
    }

    #[test]
    fn test_simple_circuit_with_evm_io() {
        // Define your circuit.
        let mut builder = CircuitBuilderX::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(7u8);

        // // Generate a proof.
        // let (proof, mut output) = circuit.prove(&input);

        // // Read output.
        // let xor = output.evm_read::<ByteVariable>();
        // println!("{}", xor);
    }
}
