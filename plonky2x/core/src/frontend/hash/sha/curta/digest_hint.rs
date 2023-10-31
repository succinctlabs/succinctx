use core::marker::PhantomData;

use curta::math::prelude::PrimeField64;
use serde::{Deserialize, Serialize};

use super::SHA;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

/// Provides the SHA of a message usign the algorithm specified by `S`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SHADigestHint<S, const CYCLE_LEN: usize> {
    _marker: PhantomData<S>,
}

impl<L: PlonkParameters<D>, S: SHA<L, D, CYCLE_LEN>, const D: usize, const CYCLE_LEN: usize>
    Hint<L, D> for SHADigestHint<S, CYCLE_LEN>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let length = input_stream.read_value::<Variable>().as_canonical_u64() as usize;
        // Read the padded chunks from the input stream.
        let message = input_stream.read_vec::<ByteVariable>(length);
        let padded_chunks = S::pad(&message);

        // Initialize the hash state.
        let mut current_state = S::INITIAL_HASH;
        for chunk in padded_chunks.chunks_exact(16) {
            let pre_processed = S::pre_process(chunk);
            current_state = S::process(current_state, &pre_processed);
        }
        // Write the digest to the output stream.
        output_stream.write_value::<[S::IntVariable; 8]>(current_state)
    }
}

impl<S, const CYCLE_LEN: usize> SHADigestHint<S, CYCLE_LEN> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<S, const CYCLE_LEN: usize> Default for SHADigestHint<S, CYCLE_LEN> {
    fn default() -> Self {
        Self::new()
    }
}
