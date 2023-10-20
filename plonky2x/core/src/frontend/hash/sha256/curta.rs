use core::marker::PhantomData;

use curta::chip::hash::sha::sha256::builder_gadget::{SHA256Builder, SHA256BuilderGadget};
use curta::chip::hash::sha::sha256::generator::SHA256HintGenerator;
use itertools::Itertools;
use log::debug;
use plonky2::iop::target::Target;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, CircuitVariable, U64Variable};

#[derive(Debug, Clone)]
pub struct Sha256Accelerator<L: PlonkParameters<D>, const D: usize> {
    pub sha256_requests: Vec<Vec<Target>>,
    pub sha256_responses: Vec<[Target; 32]>,
    _marker: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> Sha256Accelerator<L, D> {
    pub fn build(&mut self, builder: &mut CircuitBuilder<L, D>) {
        builder.curta_constrain_sha256(self);
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Pad the given variable length input according to the SHA-256 spec.
    ///
    /// It is assumed that `input` has length MAX_NUM_CHUNKS * 64.
    /// The true number of non-zero bytes in `input` is given by input_byte_length.
    /// Input byte length should be at most MAX_NUM_CHUNKS * 64 - 9.
    /// last_chunk = (input_byte_length + 9) / 64, where 9 represents the 8 length bytes and 1 padding byte.
    /// It is assumed that the caller of this function has computed last_chunk correctly.
    fn pad_message_sha256_variable<const MAX_NB_CHUNKS: usize>(
        &mut self,
        input: &[ByteVariable],
        input_byte_length: U32Variable,
        last_chunk: U32Variable,
    ) -> Vec<ByteVariable> {
        assert_eq!(input.len(), MAX_NB_CHUNKS * 64);
        // Compute the length bytes (big-endian representation of the length in bits).
        let zero_byte = self.constant::<ByteVariable>(0x00);
        let mut length_bytes = vec![zero_byte; 4];

        let bits_per_byte = self.constant::<U32Variable>(8);
        let input_bit_length = self.mul(input_byte_length, bits_per_byte);

        let mut length_bits = self.to_le_bits(input_bit_length);
        length_bits.reverse();

        // Prepend 4 zero bytes to length_bytes as abi.encodePacked(U32Variable) is 4 bytes.
        length_bytes.extend_from_slice(
            &length_bits
                .chunks(8)
                .map(|chunk| {
                    let variables = chunk.iter().map(|b| b.0).collect_vec();
                    ByteVariable::from_variables_unsafe(&variables)
                })
                .collect_vec(),
        );

        let mut padded_bytes = Vec::new();

        let mut message_byte_selector = self.constant::<BoolVariable>(true);
        for i in 0..MAX_NB_CHUNKS {
            let chunk_offset = 64 * i;
            let curr_chunk = self.constant::<U32Variable>(i as u32);

            let is_last_chunk = self.is_equal(curr_chunk, last_chunk);

            for j in 0..64 {
                let idx = chunk_offset + j;
                let idx_t = self.constant::<U32Variable>(idx as u32);
                let is_last_msg_byte = self.is_equal(idx_t, input_byte_length);
                let not_last_msg_byte = self.not(is_last_msg_byte);

                message_byte_selector = self.select(
                    message_byte_selector,
                    not_last_msg_byte,
                    message_byte_selector,
                );

                let padding_start_byte = self.constant::<ByteVariable>(0x80);

                // If message_byte_selector is true, select the message byte.
                let mut byte = self.select(message_byte_selector, input[idx], zero_byte);
                // If idx == length_bytes, select the padding start byte.
                byte = self.select(is_last_msg_byte, padding_start_byte, byte);
                if j >= 64 - 8 {
                    // If in last chunk, select the length byte.
                    byte = self.select(is_last_chunk, length_bytes[j % 8], byte);
                }

                padded_bytes.push(byte);
            }
        }

        assert_eq!(padded_bytes.len(), MAX_NB_CHUNKS * 64);
        padded_bytes
    }

    /// Executes a SHA256 hash on the given input of fixed size.
    pub fn curta_sha256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let padded_input = self.pad_message_sha256(input);

        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(Sha256Accelerator::<L, D> {
                sha256_requests: Vec::new(),
                sha256_responses: Vec::new(),
                _marker: PhantomData,
            });
        }

        let bytes = padded_input
            .iter()
            .map(|x| x.to_variable(self).0)
            .collect::<Vec<_>>();

        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator.sha256_requests.push(bytes);
        let digest = self.api.add_virtual_target_arr::<32>();
        accelerator.sha256_responses.push(digest);

        let bytes: [ByteVariable; 32] = digest.map(|x| ByteVariable::from_target(self, x));
        bytes.into()
    }

    /// Executes a SHA256 hash on the given input. Note: input should be length MAX_NUM_CHUNKS * 64.
    pub fn curta_sha256_variable<const MAX_NUM_CHUNKS: usize>(
        &mut self,
        input: &[ByteVariable],
        last_chunk: U32Variable,
        input_byte_length: U32Variable,
    ) -> Bytes32Variable {
        // TODO: Currently, Curta does not support no-ops over SHA chunks. Until Curta SHA-256 supports no-ops, last_chunk should always be equal to MAX_NUM_CHUNKS - 1.
        let expected_last_chunk = self.constant::<U32Variable>((MAX_NUM_CHUNKS - 1) as u32);
        self.assert_is_equal(expected_last_chunk, last_chunk);

        let padded_input = self.pad_message_sha256_variable::<MAX_NUM_CHUNKS>(
            input,
            last_chunk,
            input_byte_length,
        );

        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(Sha256Accelerator::<L, D> {
                sha256_requests: Vec::new(),
                sha256_responses: Vec::new(),
                _marker: PhantomData,
            });
        }

        let bytes = padded_input
            .iter()
            .map(|x| x.to_variable(self).0)
            .collect::<Vec<_>>();

        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator.sha256_requests.push(bytes);
        let digest = self.api.add_virtual_target_arr::<32>();
        accelerator.sha256_responses.push(digest);

        let bytes: [ByteVariable; 32] = digest.map(|x| ByteVariable::from_target(self, x));
        bytes.into()
    }

    pub fn curta_sha256_pair(
        &mut self,
        left: Bytes32Variable,
        right: Bytes32Variable,
    ) -> Bytes32Variable {
        let mut input = Vec::new();
        input.extend(&left.as_bytes());
        input.extend(&right.as_bytes());
        self.curta_sha256(&input)
    }

    // TODO: needs to be cleaned up and significantly rewritten
    /// Takes the accelerator with all requests and constraints it
    fn curta_constrain_sha256(&mut self, accelerator: &mut Sha256Accelerator<L, D>) {
        todo!();
        // let mut nb_chunks = 0;
        // let mut curr_rq = 0;
        // let mut num_rqs = accelerator.sha256_requests.len();

        // let zero = self.constant::<ByteVariable>(0u8);
        // let zero_chunk = [zero; 1];

        // // If a request crosses over the 1024-chunk boundary from Curta, insert dummy chunks.
        // // This is because Curta does not support requests split over multiple gadgets.

        // // Loop over all requests (including dummy requests).
        // while curr_rq < num_rqs {
        //     let curr_rq_nb_chunks = accelerator.sha256_requests[curr_rq].len() / 64;

        //     let temp_nb_chunks = nb_chunks + curr_rq_nb_chunks;

        //     // If curr_rq crosses over the 1024-chunk boundary, insert dummy chunks.
        //     if (temp_nb_chunks / 1024 != nb_chunks / 1024) && temp_nb_chunks % 1024 != 0 {
        //         while nb_chunks % 1024 != 0 {
        //             let padded_input = self.pad_message_sha256(&zero_chunk);
        //             let bytes = padded_input
        //                 .iter()
        //                 .map(|x| x.to_variable(self).0)
        //                 .collect::<Vec<_>>();

        //             // Insert a dummy request and response.
        //             accelerator.sha256_requests.insert(curr_rq, bytes);
        //             let digest = self.api.add_virtual_target_arr::<32>();
        //             accelerator.sha256_responses.insert(curr_rq, digest);

        //             // Increment the number of requests and chunks accordingly.
        //             curr_rq += 1;
        //             num_rqs += 1;

        //             nb_chunks += 1;
        //         }
        //     }
        //     nb_chunks += curr_rq_nb_chunks;
        //     curr_rq += 1;
        // }

        // // If the number of chunks is not a multiple of 1024, pad the gadget with dummy chunks.
        // while nb_chunks % 1024 != 0 {
        //     // self.curta_sha256_with_accelerator(&zero_chunk, accelerator);
        //     self.curta_sha256(&zero_chunk);
        //     nb_chunks += 1;
        // }

        // // Allocate Curta SHA-256 gadgets according to the number of chunks across all requests.
        // let gadgets: Vec<SHA256BuilderGadget<<L as PlonkParameters<D>>::Field, L::CubicParams, D>> =
        //     (0..nb_chunks / 1024)
        //         .map(|_| self.api.init_sha256())
        //         .collect_vec();
        // debug!("allocated {} curta sha256 gadgets", gadgets.len());

        // let mut rq_idx = 0;
        // for i in 0..gadgets.len() {
        //     let mut gadget = gadgets[i].to_owned();

        //     // Fill the gadget with 1024 padded chunks.
        //     let mut num_chunks_so_far = 0;
        //     while num_chunks_so_far < 1024 {
        //         gadget
        //             .padded_messages
        //             .extend_from_slice(&accelerator.sha256_requests[rq_idx]);
        //         let hint = SHA256HintGenerator::new(
        //             &accelerator.sha256_requests[rq_idx],
        //             accelerator.sha256_responses[rq_idx],
        //         );

        //         self.add_simple_generator(hint);
        //         gadget
        //             .digests
        //             .extend_from_slice(&accelerator.sha256_responses[rq_idx]);
        //         gadget
        //             .chunk_sizes
        //             .push(accelerator.sha256_requests[rq_idx].len() / 64);

        //         num_chunks_so_far += accelerator.sha256_requests[rq_idx].len() / 64;
        //         rq_idx += 1;
        //     }

        //     self.constrain_sha256_gadget(gadget);
        // }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::backend::circuit::{CircuitBuild, DefaultParameters};
    use crate::frontend::uint::uint32::U32Variable;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{
        ByteVariable, BytesVariable, CircuitBuilder, DefaultBuilder, GateRegistry, HintRegistry,
    };
    use crate::utils::{bytes, bytes32};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_fixed_single() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.constant::<ByteVariable>(0u8);
        let result = builder.curta_sha256(&[zero; 1]);
        builder.watch(&result, "result");

        let expected_digest =
            bytes32!("0x6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        builder.assert_is_equal(result, expected_digest);

        let circuit = builder.build();
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_curta_allocation() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = DefaultBuilder::new();

        // Requires 2 chunks each.
        let short_msg = [1u8; 56];

        let short_msg_bytes = short_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();

        let mut msgs = (0..1024)
            .map(|_| short_msg_bytes.clone())
            .collect::<Vec<_>>();

        // Requires 3 chunks each.
        let long_msg = [1u8; 128];
        let long_msg_bytes = long_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();

        msgs.extend(
            (0..2048)
                .map(|_| long_msg_bytes.clone())
                .collect::<Vec<_>>(),
        );

        let mut builder = CircuitBuilder::<L, D>::new();
        let _ = msgs
            .iter()
            .map(|msg| builder.curta_sha256(msg))
            .collect::<Vec<_>>();

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_variable_single() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let msg = builder.constant::<BytesVariable<64>>(bytes!(
            "00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d00000000000000000000000000000000000000000000000000"
        ));

        let bytes_length = builder.constant::<U32Variable>(39);

        let expected_digest =
            bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        let last_chunk = builder.constant::<U32Variable>(0);

        let msg_hash = builder.curta_sha256_variable::<1>(&msg.0, last_chunk, bytes_length);
        builder.watch(&msg_hash, "msg_hash");

        builder.assert_is_equal(msg_hash, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        // TODO: Add back once curta serialization is implemented.
        // circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_serialized_sha256_curta() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.constant::<ByteVariable>(0u8);
        let result = builder.curta_sha256(&[zero; 1]);
        builder.watch(&result, "result");

        let expected_digest =
            bytes32!("0x6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        builder.assert_is_equal(result, expected_digest);

        let circuit = builder.build();
        let gate_serializer = GateRegistry::<L, D>::new();
        let hint_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &hint_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &hint_serializer).unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }
}
