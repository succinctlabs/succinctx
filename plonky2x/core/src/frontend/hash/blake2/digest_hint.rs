use curta::machine::hash::blake::blake2b;
use curta::machine::hash::blake::blake2b::pure::BLAKE2BPure;
use curta::machine::hash::blake::blake2b::utils::BLAKE2BUtil;
use curta::math::prelude::PrimeField64;
use serde::{Deserialize, Serialize};

use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::*;

/// Provides the BLAKE2B of a message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BLAKE2BDigestHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BLAKE2BDigestHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let length = input_stream.read_value::<Variable>().as_canonical_u64() as usize;
        // Read the padded chunks from the input stream.
        let message = input_stream.read_vec::<ByteVariable>(length);
        let mut num_message_chunks = (message.len() as u64 / 128) + 1;
        if num_message_chunks % 128 == 0 {
            num_message_chunks -= 1;
        }

        let padded_chunks = BLAKE2BUtil::pad(&message, num_message_chunks);

        println!("padded chunks is {:?}", padded_chunks);
        println!("num_message_chunks is {:?}", num_message_chunks);
        println!("msg len is {:?}", message.len());

        // Initialize the hash state.
        let mut state = blake2b::IV;
        let mut t = 0;
        for (i, chunk) in padded_chunks.chunks_exact(128).enumerate() {
            let at_last_chunk = i as u64 == num_message_chunks - 1;
            if at_last_chunk {
                t = message.len() as u64;
            } else {
                t += 128;
            }
            BLAKE2BPure::compress(chunk, &mut state, t, at_last_chunk);
        }

        // Write the digest to the output stream.
        let mut digest: [u64; 4] = Default::default();
        println!("state: {:?}", state);
        digest.copy_from_slice(&state[0..4]);
        output_stream.write_value::<[U64Variable; 4]>(digest)
    }
}

impl BLAKE2BDigestHint {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BLAKE2BDigestHint {
    fn default() -> Self {
        Self::new()
    }
}
