use std::collections::HashMap;

use plonky2::iop::witness::{PartialWitness, PartitionWitness};
use plonky2::plonk::circuit_data::MockCircuitData;

use super::input::PublicInput;
use super::output::PublicOutput;
use super::witness::{generate_witness, GenerateWitnessError};
use super::PlonkParameters;
use crate::frontend::builder::CircuitIO;

/// A mock circuit that can be used for testing.
///
/// Mock circuits are not meant to be used in production. It is only meant to be used for testing.
/// It skips a bunch of time-consuming steps in .build() and .prove().
#[derive(Debug)]
pub struct MockCircuit<L: PlonkParameters<D>, const D: usize> {
    pub data: MockCircuitData<L::Field, L::Config, D>,
    pub io: CircuitIO<D>,
    pub debug_variables: HashMap<usize, String>,
}

impl<L: PlonkParameters<D>, const D: usize> MockCircuit<L, D> {
    /// Returns an input instance for the circuit.
    pub fn input(&self) -> PublicInput<L, D> {
        PublicInput::new(&self.io)
    }

    pub fn mock_prove(
        &self,
        input: &PublicInput<L, D>,
    ) -> (PartitionWitness<L::Field>, PublicOutput<L, D>) {
        // Initialize the witness.
        let mut pw = PartialWitness::new();

        // Write the input to the witness.
        self.io.set_witness(&mut pw, input);

        // Generate the rest of witness.
        let witness = match generate_witness(pw, &self.data.prover_only, &self.data.common) {
            Ok(witness) => witness,
            Err(GenerateWitnessError::GeneratorsNotRun(targets)) => {
                panic!("generators not run: {:?}", targets)
            }
        };

        // Get the output from the witness.
        let output = PublicOutput::from_witness(&self.io, &witness);

        (witness, output)
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use crate::prelude::*;

    #[test]
    fn test_mock_circuit_with_field_io() {
        // Define your circuit.
        let mut builder = DefaultBuilder::new();
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

        // Generate a proof.
        let (_witness, mut output) = mock_circuit.mock_prove(&input);

        // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);
    }

    #[test]
    fn test_simple_circuit_with_evm_io() {
        // Define your circuit.
        let mut builder = DefaultBuilder::new();
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
