use super::curta::BLAKE2BAccelerator;
use super::digest_hint::BLAKE2BDigestHint;
use super::proof_hint::BLAKE2BProofHint;
use super::stark::{get_blake2b_data, stark};
use crate::frontend::hash::curta::request::HashRequest;
use crate::frontend::hint::synchronous::Async;
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// The constraints for an accelerated BLAKE2B computation using Curta.
    pub(crate) fn curta_constrain_blake2b(&mut self, accelerator: BLAKE2BAccelerator) {
        // Get all the digest values using the digest hint.
        for (request, response) in accelerator
            .hash_requests
            .iter()
            .zip(accelerator.hash_responses.iter())
        {
            let digest_hint = BLAKE2BDigestHint::new();
            let mut input_stream = VariableStream::new();

            match &request {
                HashRequest::Fixed(msg) => {
                    let len = self.constant::<Variable>(L::Field::from_canonical_usize(msg.len()));
                    input_stream.write(&len);
                    input_stream.write_slice(msg);
                }
                HashRequest::Variable(msg, len, _) => {
                    input_stream.write(len);
                    input_stream.write_slice(msg);
                }
            }

            let output_stream = self.hint(input_stream, digest_hint);
            let digest = output_stream.read::<[U64Variable; 4]>(self);
            self.assert_is_equal(digest, *response);
        }

        // Prove correctness of the digest using the proof hint.

        // Initialize the corresponding stark and hint.
        let blake2b_data = get_blake2b_data(self, accelerator);
        let parameters = blake2b_data.parameters();
        let blake2b_stark = stark(parameters);
        let proof_hint = BLAKE2BProofHint::new(parameters);
        let mut input_stream = VariableStream::new();
        input_stream.write_blake2b_input(&blake2b_data);

        // Read the stark proof and public inputs from the hint's output stream.
        let output_stream = self.async_hint(input_stream, Async(proof_hint));
        let proof = output_stream.read_byte_stark_proof(self, &blake2b_stark.stark);
        let num_public_inputs = blake2b_stark.stark.air_data.num_public_inputs;
        let public_inputs = output_stream.read_vec(self, num_public_inputs);

        // Verify the proof.
        blake2b_stark.verify_proof(self, proof, &public_inputs, blake2b_data)
    }
}
