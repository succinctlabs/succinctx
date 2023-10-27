use core::marker::PhantomData;

use array_macro::array;
use itertools::Itertools;
use log::debug;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hash::sha::curta::request::SHARequest;
use crate::frontend::hash::sha::sha256::curta::SHA256Accelerator;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::{Bytes32Variable, EvmVariable};
use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, *};

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
        last_chunk: U32Variable,
        input_byte_length: U32Variable,
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
                    let bits = array![x => chunk[x]; 8];
                    ByteVariable(bits)
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
        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(SHA256Accelerator {
                sha_requests: Vec::new(),
                sha_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_value = digest
            .as_bytes()
            .chunks_exact(4)
            .map(|x| U32Variable::decode(self, x))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator
            .sha_requests
            .push(SHARequest::Fixed(input.to_vec()));
        accelerator.sha_responses.push(digest_value);

        digest
    }

    // /// Executes a SHA256 hash on the given input. Note: input should be length MAX_NUM_CHUNKS * 64.
    // pub fn curta_sha256_variable<const MAX_NUM_CHUNKS: usize>(
    //     &mut self,
    //     input: &[ByteVariable],
    //     input_byte_length: U32Variable,
    // ) -> Bytes32Variable {
    //     let nine = self.constant(9u32);
    //     let sixty_four = self.constant(64u32);
    //     let mut num_chunks = self.add(input_byte_length, nine);
    //     num_chunks = self.div(num_chunks, sixty_four);
    //     let padded_input = self.pad_message_sha256_variable::<MAX_NUM_CHUNKS>(
    //         input,
    //         num_chunks,
    //         input_byte_length,
    //     );

    //     if self.sha256_accelerator.is_none() {
    //         self.sha256_accelerator = Some(Sha256Accelerator::<L, D> {
    //             sha256_requests: Vec::new(),
    //             sha256_responses: Vec::new(),
    //             _marker: PhantomData,
    //         });
    //     }

    //     let accelerator = self
    //         .sha256_accelerator
    //         .as_mut()
    //         .expect("sha256 accelerator should exist");
    //     accelerator.sha256_requests.push(padded_input);
    //     let digest = self.api.add_virtual_target_arr::<32>();
    //     accelerator.sha256_responses.push(digest);

    //     let bytes: [ByteVariable; 32] = digest.map(|x| ByteVariable::from_target(self, x));
    //     bytes.into()
    // }

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

    // /// Takes the accelerator with all requests and constraints it
    // fn curta_constrain_sha256(&mut self, accelerator: &Sha256Accelerator<L, D>) {
    // let mut current_gadget = self.api.init_sha256();
    // let mut current_gadget_chunks = 0;
    // // Iterate through all requests to the accelerator. As we iterate, we allocate gadgets for
    // // groups of requests, keeping in mind that gadgets can only accomodate 1024 chunks.
    // for (i, req) in accelerator.sha256_requests.iter().enumerate() {
    //     // All the requests are chunk-aligned because of the padding we do.
    //     let request_chunks = req.len() / 64;

    //     // If this request would overflow the current_gadget, pad the gadget and constrain it.
    //     if current_gadget_chunks + request_chunks > 1024 {
    //         debug!("allocated curta sha256 gadget");
    //         self.pad_and_constrain_sha256_gadget(&mut current_gadget, current_gadget_chunks);

    //         // Then reset current_gadget and current_gadget_chunks
    //         current_gadget = self.api.init_sha256();
    //         current_gadget_chunks = 0;
    //     }

    //     // We add the current request to the current gadget and update current_gadget_chunks appropriately.
    //     current_gadget_chunks += request_chunks;
    //     current_gadget
    //         .padded_messages
    //         .extend_from_slice(&accelerator.sha256_requests[i]);
    //     let hint = SHA256HintGenerator::new(
    //         &accelerator.sha256_requests[i],
    //         accelerator.sha256_responses[i],
    //     );
    //     self.add_simple_generator(hint);
    //     current_gadget
    //         .digests
    //         .extend_from_slice(&accelerator.sha256_responses[i]);
    //     current_gadget.chunk_sizes.push(request_chunks);
    // }

    // // At the end, if there is a non-empty gadget, pad and constrain it.
    // if current_gadget_chunks > 0 {
    //     debug!("allocated curta sha256 gadget");
    //     self.pad_and_constrain_sha256_gadget(&mut current_gadget, current_gadget_chunks);
    // }
    // }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::backend::circuit::{CircuitBuild, DefaultParameters};
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{
        ByteVariable, CircuitBuilder, DefaultBuilder, GateRegistry, HintRegistry,
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
        // let gate_serializer = GateRegistry::<L, D>::new();
        // let generator_serializer = HintRegistry::<L, D>::new();
        // let bytes = circuit
        //     .serialize(&gate_serializer, &generator_serializer)
        //     .unwrap();
        // let circuit =
        //     CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
        //         .unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        // circuit.test_default_serializers();
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

    // #[test]
    // #[cfg_attr(feature = "ci", ignore)]
    // fn test_sha256_curta_variable_single() {
    //     env::set_var("RUST_LOG", "debug");
    //     env_logger::try_init().unwrap_or_default();
    //     dotenv::dotenv().ok();

    //     let mut builder = CircuitBuilder::<L, D>::new();

    //     let msg = builder.constant::<BytesVariable<64>>(bytes!(
    //         "00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d00000000000000000000000000000000000000000000000000"
    //     ));

    //     let bytes_length = builder.constant::<U32Variable>(39);

    //     let expected_digest =
    //         bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
    //     let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

    //     let last_chunk = builder.constant::<U32Variable>(0);

    //     let msg_hash = builder.curta_sha256_variable::<1>(&msg.0, last_chunk, bytes_length);
    //     builder.watch(&msg_hash, "msg_hash");
    //     builder.assert_is_equal(msg_hash, expected_digest);

    //     let circuit = builder.build();
    //     let input = circuit.input();
    //     let (proof, output) = circuit.prove(&input);
    //     circuit.verify(&proof, &input, &output);

    //     circuit.test_default_serializers();
    // }

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
