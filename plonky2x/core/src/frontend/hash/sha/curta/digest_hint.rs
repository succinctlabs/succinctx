use core::marker::PhantomData;

use curta::math::prelude::PrimeField64;
use serde::{Deserialize, Serialize};

use super::request::SHARequestType;
use super::SHA;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SHADigestHint<S, const CYCLE_LEN: usize> {
    pub input_length: usize,
    pub request_type: SHARequestType,
    _marker: PhantomData<S>,
}

impl<L: PlonkParameters<D>, S: SHA<L, D, CYCLE_LEN>, const D: usize, const CYCLE_LEN: usize>
    Hint<L, D> for SHADigestHint<S, CYCLE_LEN>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        // Read the padded chunks from the input stream.
        let message = input_stream.read_vec::<ByteVariable>(self.input_length);
        let padded_chunks = S::pad(&message);
        // Read the desired digest from the output stream. This is the value that we want to ouput
        // from the circuit and matches the hash state after `num_chunks` hashed.
        let num_chunks = match self.request_type {
            SHARequestType::Fixed => padded_chunks.len() / 16,
            SHARequestType::Variable => {
                input_stream.read_value::<Variable>().as_canonical_u64() as usize
            }
        };
        // Initialize the hash state.
        let mut current_state = S::INITIAL_HASH;
        for chunk in padded_chunks.chunks_exact(16).take(num_chunks) {
            let pre_processed = S::pre_process(chunk);
            current_state = S::process(current_state, &pre_processed);
        }
        // Write the digest to the output stream.
        output_stream.write_value::<[S::IntVariable; 8]>(current_state)
    }
}

impl<S, const CYCLE_LEN: usize> SHADigestHint<S, CYCLE_LEN> {
    pub fn new(input_length: usize, request_type: SHARequestType) -> Self {
        Self {
            input_length,
            request_type,
            _marker: PhantomData,
        }
    }
}
