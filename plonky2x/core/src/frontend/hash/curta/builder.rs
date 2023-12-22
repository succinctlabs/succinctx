use curta::chip::Chip;
use curta::plonky2::Plonky2Air;

use super::accelerator::HashAccelerator;
use super::digest_hint::HashDigestHint;
use super::proof_hint::HashProofHint;
use super::request::HashRequest;
use super::Hash;
use crate::frontend::hint::synchronous::Async;
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// The constraints for an accelerated hash computation using Curta.
    pub(crate) fn curta_constrain_hash<
        S: Hash<L, D, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>,
        const CYCLE_LEN: usize,
        const USE_T_VALUES: bool,
        const DIGEST_LEN: usize,
    >(
        &mut self,
        accelerator: HashAccelerator<S::IntVariable, DIGEST_LEN>,
    ) where
        Chip<S::AirParameters>: Plonky2Air<L::Field, D>,
    {
        // Get all the digest values using the digest hint.
        for (request, response) in accelerator
            .hash_requests
            .iter()
            .zip(accelerator.hash_responses.iter())
        {
            let digest_hint = HashDigestHint::<S, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN>::new();
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
            let digest = output_stream.read::<[S::IntVariable; DIGEST_LEN]>(self);
            self.assert_is_equal(digest, *response);
        }

        // Prove correctness of the digest using the proof hint.

        // Initialize the corresponding stark and hint.
        let hash_data = S::get_hash_data(self, accelerator);
        let parameters = hash_data.parameters();
        let hash_stark = S::stark(parameters);
        let proof_hint: HashProofHint<S, CYCLE_LEN, USE_T_VALUES, DIGEST_LEN> =
            HashProofHint::new(parameters);
        let mut input_stream = VariableStream::new();
        input_stream.write_hash_input(&hash_data);

        // Read the stark proof and public inputs from the hint's output stream.
        let output_stream = self.async_hint(input_stream, Async(proof_hint));
        let proof = output_stream.read_byte_stark_proof(self, &hash_stark.stark);
        let num_public_inputs = hash_stark.stark.air_data.num_public_inputs;
        let public_inputs = output_stream.read_vec(self, num_public_inputs);

        // Verify the proof.
        hash_stark.verify_proof(self, proof, &public_inputs, hash_data)
    }
}
