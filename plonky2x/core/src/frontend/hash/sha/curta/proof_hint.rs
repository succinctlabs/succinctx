use core::marker::PhantomData;

use curta::chip::Chip;
use curta::plonky2::Plonky2Air;
use serde::{Deserialize, Serialize};

use super::data::SHAInputParameters;
use super::SHA;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

/// A hint for Curta proof of a SHA stark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHAProofHint<S, const CYCLE_LEN: usize> {
    parameters: SHAInputParameters,
    _marker: PhantomData<S>,
}

impl<S, const CYCLE_LEN: usize> SHAProofHint<S, CYCLE_LEN> {
    pub fn new(parameters: SHAInputParameters) -> Self {
        Self {
            parameters,
            _marker: PhantomData,
        }
    }
}

impl<L: PlonkParameters<D>, S: SHA<L, D, CYCLE_LEN>, const D: usize, const CYCLE_LEN: usize>
    Hint<L, D> for SHAProofHint<S, CYCLE_LEN>
where
    Chip<S::AirParameters>: Plonky2Air<L::Field, D>,
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let inputs = input_stream.read_sha_input_values(self.parameters);
        let stark = S::stark(self.parameters);

        // Generate the proof with public inputs and write them to the output stream.
        let (proof, public_inputs) = stark.prove(inputs);

        output_stream.write_byte_stark_proof(proof);
        output_stream.write_slice(&public_inputs);
    }
}
