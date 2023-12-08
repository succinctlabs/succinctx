use serde::{Deserialize, Serialize};

use super::data::BLAKE2BInputParameters;
use super::stark::stark;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

/// A hint for Curta proof of a BLAKE2B stark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BLAKE2BProofHint {
    parameters: BLAKE2BInputParameters,
}

impl BLAKE2BProofHint {
    pub fn new(parameters: BLAKE2BInputParameters) -> Self {
        Self { parameters }
    }
}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BLAKE2BProofHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let inputs = input_stream.read_blake2b_input_values(self.parameters);
        let stark = stark(self.parameters);

        println!("inputs are {:?}", inputs);
        println!("parameters are {:?}", self.parameters);

        // Generate the proof with public inputs and write them to the output stream.
        let (proof, public_inputs) = stark.prove(inputs);

        output_stream.write_byte_stark_proof(proof);
        output_stream.write_slice(&public_inputs);
    }
}
