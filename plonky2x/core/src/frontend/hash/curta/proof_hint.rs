use core::marker::PhantomData;

use serde::{Deserialize, Serialize};
use starkyx::chip::Chip;
use starkyx::plonky2::Plonky2Air;

use super::data::HashInputParameters;
use super::Hash;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

/// A hint for Curta proof of a SHA stark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashProofHint<
    H,
    const CYCLE_LEN: usize,
    const USE_T_VALUES: bool,
    const DIGEST_LEN: usize,
> {
    parameters: HashInputParameters,
    _marker: PhantomData<H>,
}

impl<H, const CYCLE_LEN: usize, const USE_T_VALUES: bool, const DIGEST_LEN: usize>
    HashProofHint<H, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>
{
    pub fn new(parameters: HashInputParameters) -> Self {
        Self {
            parameters,
            _marker: PhantomData,
        }
    }
}

impl<
        L: PlonkParameters<D>,
        H: Hash<L, D, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>,
        const D: usize,
        const CYCLE_LEN: usize,
        const USE_T_VALUES: bool,
        const DIGEST_LEN: usize,
    > Hint<L, D> for HashProofHint<H, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>
where
    Chip<H::AirParameters>: Plonky2Air<L::Field, D>,
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let inputs = input_stream.read_hash_input_values(self.parameters);
        let stark = H::stark(self.parameters);

        // Generate the proof with public inputs and write them to the output stream.
        let (proof, public_inputs) = stark.prove(inputs);

        output_stream.write_byte_stark_proof(proof);
        output_stream.write_slice(&public_inputs);
    }
}
