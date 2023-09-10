use std::collections::HashMap;

use itertools::Itertools;
use plonky2::iop::witness::{PartialWitness, PartitionWitness};
use plonky2::plonk::circuit_data::MockCircuitData;

use super::witness::{generate_witness, GenerateWitnessError};
use crate::backend::circuit::{CircuitInput, CircuitOutput};
use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitIO;
use crate::frontend::vars::CircuitVariable;

/// A compiled circuit which can compute any function in the form `f(x)=y`.
#[derive(Debug)]
pub struct MockCircuit<L: PlonkParameters<D>, const D: usize> {
    pub data: MockCircuitData<L::Field, L::Config, D>,
    pub io: CircuitIO<D>,
    pub debug_variables: HashMap<usize, String>,
}

impl<L: PlonkParameters<D>, const D: usize> MockCircuit<L, D> {
    /// Returns an input instance for the circuit.
    pub fn input(&self) -> CircuitInput<L, D> {
        CircuitInput {
            io: self.io.clone(),
            buffer: Vec::new(),
        }
    }

    pub fn mock_prove(
        &self,
        input: &CircuitInput<L, D>,
    ) -> (PartitionWitness<L::Field>, CircuitOutput<L, D>) {
        // Get input variables from io.
        let input_variables = self.io.input();
        assert_eq!(input_variables.len(), input.buffer.len());

        // Assign input variables.
        let mut pw = PartialWitness::new();
        for i in 0..input_variables.len() {
            input_variables[i].set(&mut pw, input.buffer[i]);
        }

        let result = generate_witness(pw, &self.data.prover_only, &self.data.common);
        if let Err(e) = result {
            match e {
                GenerateWitnessError::GeneratorsNotRun(targets) => {
                    panic!("generators not run: {:?}", targets)
                }
            }
        };
        let filled_witness = result.unwrap();

        let output_variables = self.io.output();
        let output_elements = output_variables
            .iter()
            .map(|v| v.get(&filled_witness))
            .collect_vec();

        (
            filled_witness,
            CircuitOutput {
                io: self.io.clone(),
                buffer: output_elements,
            },
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use crate::prelude::*;

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
        let (_witness, mut output) = mock_circuit.mock_prove(&input);

        // // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);
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
        let mock_circuit = builder.mock_build();

        // Write to the circuit input.
        let mut input = mock_circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(7u8);

        // // Generate a proof.
        let (_witness, mut output) = mock_circuit.mock_prove(&input);

        // // Read output.
        let xor = output.evm_read::<ByteVariable>();
        println!("{}", xor);
    }
}
