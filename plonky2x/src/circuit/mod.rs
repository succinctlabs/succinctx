pub mod io;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;

use self::io::{CircuitInput, CircuitOutput};
use crate::builder::CircuitIO;
use crate::prelude::CircuitVariable;

/// A compiled circuit which can compute any function in the form `f(x)=y`.
pub struct Circuit<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub data: CircuitData<F, C, D>,
    pub io: CircuitIO<D>,
}

impl<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> Circuit<F, C, D> {
    /// Returns an input instance for the circuit.
    pub fn input(&self) -> CircuitInput<F, D> {
        CircuitInput {
            io: self.io.clone(),
            buffer: Vec::new(),
        }
    }

    /// Generates a proof for the circuit. The proof can be verified using `verify`.
    pub fn prove(
        &self,
        input: &CircuitInput<F, D>,
    ) -> (ProofWithPublicInputs<F, C, D>, CircuitOutput<F, D>) {
        // Get input variables from io.
        let input_variables = if self.io.evm.is_some() {
            self.io
                .evm
                .clone()
                .unwrap()
                .input_bytes
                .into_iter()
                .flat_map(|b| b.variables())
                .collect()
        } else if self.io.field.is_some() {
            self.io.field.clone().unwrap().input_variables
        } else {
            todo!()
        };
        assert_eq!(input_variables.len(), input.buffer.len());

        // Assign input variables.
        let mut pw = PartialWitness::new();
        for i in 0..input_variables.len() {
            input_variables[i].set(&mut pw, input.buffer[i].into());
        }

        // Generate the proof.
        let proof = self.data.prove(pw).unwrap();

        // Slice the public inputs to reflect the output portion of the circuit.
        let output = CircuitOutput {
            io: self.io.clone(),
            buffer: proof.public_inputs[input_variables.len()..].to_vec(),
        };

        (proof.clone(), output)
    }

    /// Verifies a proof for the circuit.
    pub fn verify(
        &self,
        proof: &ProofWithPublicInputs<F, C, D>,
        input: &CircuitInput<F, D>,
        output: &CircuitOutput<F, D>,
    ) {
        let mut public_inputs = Vec::new();
        public_inputs.extend(input.buffer.clone());
        public_inputs.extend(output.buffer.clone());
        assert_eq!(public_inputs.len(), proof.public_inputs.len());
        for i in 0..public_inputs.len() {
            assert_eq!(public_inputs[i], proof.public_inputs[i]);
        }
        self.data.verify(proof.clone()).unwrap();
    }
}
