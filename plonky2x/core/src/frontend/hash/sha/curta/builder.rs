use curta::chip::Chip;
use curta::plonky2::Plonky2Air;

use super::accelerator::SHAAccelerator;
use super::digest_hint::SHADigestHint;
use super::proof_hint::SHAProofHint;
use super::request::SHARequest;
use super::SHA;
use crate::frontend::hint::synchronous::Async;
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// The constraints for an accelerated SHA computation using Curta.
    pub(crate) fn curta_constrain_sha<S: SHA<L, D, CYCLE_LEN>, const CYCLE_LEN: usize>(
        &mut self,
        accelerator: SHAAccelerator<S::IntVariable>,
    ) where
        Chip<S::AirParameters>: Plonky2Air<L::Field, D>,
    {
        // Get all the digest values using the digest hint.
        for (request, response) in accelerator
            .sha_requests
            .iter()
            .zip(accelerator.sha_responses.iter())
        {
            let digest_hint = SHADigestHint::<S, CYCLE_LEN>::new();
            let mut input_stream = VariableStream::new();

            match &request {
                SHARequest::Fixed(msg) => {
                    let len = self.constant::<Variable>(L::Field::from_canonical_usize(msg.len()));
                    input_stream.write(&len);
                    input_stream.write_slice(msg);
                }
                SHARequest::Variable(msg, len, _) => {
                    input_stream.write(len);
                    input_stream.write_slice(msg);
                }
            }

            let output_stream = self.hint(input_stream, digest_hint);
            let digest = output_stream.read::<[S::IntVariable; 8]>(self);
            self.assert_is_equal(digest, *response);
        }

        // Prove correctness of the digest using the proof hint.

        // Initialize the corresponding stark and hint.
        let sha_data = S::get_sha_data(self, accelerator);
        let parameters = sha_data.parameters();
        let sha_stark = S::stark(parameters);
        let proof_hint = SHAProofHint::<S, CYCLE_LEN>::new(parameters);
        let mut input_stream = VariableStream::new();
        input_stream.write_sha_input(&sha_data);

        // Read the stark proof and public inputs from the hint's output stream.
        let output_stream = self.async_hint(input_stream, Async(proof_hint));
        let proof = output_stream.read_byte_stark_proof(self, &sha_stark.stark);
        let num_public_inputs = sha_stark.stark.air_data.num_public_inputs;
        let public_inputs = output_stream.read_vec(self, num_public_inputs);

        // Verify the proof.
        sha_stark.verify_proof(self, proof, &public_inputs, sha_data)
    }
}
