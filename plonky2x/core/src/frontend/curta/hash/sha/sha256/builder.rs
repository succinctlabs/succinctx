// use core::marker::PhantomData;

// use curta::chip::hash::sha::sha256::builder_gadget::SHA256BuilderGadget;
// use curta::chip::hash::sha::sha256::generator::SHA256StarkData;
// use curta::chip::hash::sha::sha256::SHA256PublicData;

// use super::hint::Sha256ProofHint;
// use crate::frontend::hint::synchronous::Async;
// use crate::prelude::{PlonkParameters, *};

// impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
//     // Verifies and constrains a STARK proof from Curta's SHA256 gadget.
//     pub fn constrain_sha256_gadget(
//         &mut self,
//         gadget: &SHA256BuilderGadget<L::Field, L::CubicParams, D>,
//     ) {
//         let mut input_stream = VariableStream::new();

//         let hint = Sha256ProofHint::<L, D> {
//             num_messages: gadget.chunk_sizes.len(),
//             _phantom: PhantomData,
//         };

//         let SHA256StarkData { stark, config, .. } = Sha256ProofHint::<L, D>::stark_data();

//         let padded_messages = gadget
//             .padded_messages
//             .iter()
//             .map(|x| Variable(*x))
//             .collect::<Vec<_>>();

//         let chunk_sizes = gadget
//             .chunk_sizes
//             .iter()
//             .map(|x| self.constant::<Variable>(L::Field::from_canonical_usize(*x)))
//             .collect::<Vec<_>>();

//         input_stream.write_slice(&padded_messages);

//         input_stream.write_slice(&chunk_sizes);

//         let outputs = self.async_hint(input_stream, Async(hint));

//         let proof = outputs.read_stark_proof(self, &stark, &config);

//         let public_sha_targets =
//             SHA256PublicData::add_virtual(&mut self.api, &gadget.digests, &gadget.chunk_sizes);

//         let public_input_target = public_sha_targets.public_input_targets(&mut self.api);

//         let public_input_variable = public_input_target
//             .iter()
//             .map(|target| Variable(*target))
//             .collect::<Vec<_>>();

//         self.verify_stark_proof(&config, &stark, &proof, &public_input_variable);

//         // Read the public inputs from the proof.
//         let public_w = outputs.read_exact(self, public_sha_targets.public_w.len() * 4);

//         // Read the hash state of each chunk of the SHA256 gadget at the end of the proof.
//         let hash_state = outputs.read_exact(self, public_sha_targets.hash_state.len() * 4);

//         for (target, variable) in public_sha_targets
//             .public_w
//             .iter()
//             .flatten()
//             .zip(public_w.iter())
//         {
//             // Constrain the public inputs to the proof.
//             self.api.connect(*target, variable.0);
//         }

//         for (target, variable) in public_sha_targets
//             .hash_state
//             .iter()
//             .flatten()
//             .zip(hash_state.iter())
//         {
//             // Constrain the hash state of each chunk of the SHA256 gadget at the end of the proof.
//             self.api.connect(*target, variable.0);
//         }
//     }
// }
