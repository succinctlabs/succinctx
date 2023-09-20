use curta::chip::hash::sha::sha256::builder_gadget::{SHA256Builder, SHA256BuilderGadget};
use curta::chip::hash::sha::sha256::generator::SHA256HintGenerator;
use curta::math::field::Field;
use itertools::Itertools;
use log::debug;
use plonky2::iop::target::Target;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hash::bit_operations::util::u64_to_bits;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, CircuitVariable};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn bytes_to_target(&mut self, input: &[ByteVariable]) -> Vec<Target> {
        let mut bytes = Vec::new();
        for i in 0..input.len() {
            let mut byte = self.api.zero();
            let targets = input[i].targets();
            for j in 0..8 {
                let bit = targets[j];
                byte = self
                    .api
                    .mul_const_add(L::Field::from_canonical_u8(1 << (7 - j)), bit, byte);
            }
            bytes.push(byte);
        }
        bytes
    }

    /// Pad the given input according to the SHA-256 spec.
    pub fn curta_sha256_pad(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
        let mut bits = input
            .iter()
            .flat_map(|b| b.as_bool_targets().to_vec())
            .collect_vec();
        bits.push(self.api._true());

        let l = bits.len() - 1;
        let mut k = 0;
        while (l + 1 + k + 64) % 512 != 0 {
            k += 1;
        }
        for _ in 0..k {
            bits.push(self.api._false());
        }

        let be_bits = u64_to_bits(l as u64, &mut self.api);
        for i in 0..be_bits.len() {
            bits.push(be_bits[i]);
        }

        let bit_targets = bits.iter().map(|b| b.target).collect_vec();

        // Combine the bits into ByteVariable
        (0..bit_targets.len() / 8)
            .map(|i| ByteVariable::from_targets(&bit_targets[i * 8..(i + 1) * 8]))
            .collect_vec()
    }

    /// Pad the given variable length input according to the SHA-256 spec.
    /// Input should be length MAX_NUM_CHUNKS * 64.
    /// Input byte length should be at most MAX_NUM_CHUNKS * 64 - 9.
    pub fn curta_sha256_pad_variable_length<const MAX_NB_CHUNKS: usize>(
        &mut self,
        input: &[ByteVariable],
        last_chunk: U32Variable,
        input_byte_length: U32Variable,
    ) -> Vec<ByteVariable> {
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

        // TODO: Use fixed size array instead of Vec. (MAX_NUM_CHUNKS * 64)
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

        padded_bytes
    }

    /// Executes a SHA256 hash on the given input. (Assumes it's not padded)
    pub fn curta_sha256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let padded_input = self.curta_sha256_pad(input);

        let bytes = self.bytes_to_target(&padded_input);

        self.sha256_requests.push(bytes);
        let digest = self.api.add_virtual_target_arr::<32>();
        self.sha256_responses.push(digest);
        Bytes32Variable::from_targets(
            &digest
                .into_iter()
                .flat_map(|byte| {
                    let mut bits = self
                        .api
                        .low_bits(byte, 8, 8)
                        .into_iter()
                        .map(|b| b.target)
                        .collect_vec();
                    bits.reverse();
                    bits
                })
                .collect_vec(),
        )
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

        let padded_input = self.curta_sha256_pad_variable_length::<MAX_NUM_CHUNKS>(
            input,
            last_chunk,
            input_byte_length,
        );

        let bytes = self.bytes_to_target(&padded_input);

        self.sha256_requests.push(bytes);
        let digest = self.api.add_virtual_target_arr::<32>();
        self.sha256_responses.push(digest);
        Bytes32Variable::from_targets(
            &digest
                .into_iter()
                .flat_map(|byte| {
                    let mut bits = self
                        .api
                        .low_bits(byte, 8, 8)
                        .into_iter()
                        .map(|b| b.target)
                        .collect_vec();
                    bits.reverse();
                    bits
                })
                .collect_vec(),
        )
    }

    pub fn curta_constrain_sha256(&mut self) {
        let mut nb_chunks = 0;
        let mut curr_rq = 0;
        let mut num_rqs = self.sha256_requests.len();
        debug!("num_rqs: {}", num_rqs);

        let zero = self.constant::<ByteVariable>(0u8);
        let zero_chunk = [zero; 1];

        // If a request crosses over the 1024-chunk boundary from Curta, insert dummy chunks.
        // This is because Curta does not support requests split over multiple gadgets.

        // Loop over all requests (including dummy requests).
        while curr_rq < num_rqs {
            let curr_rq_nb_chunks = self.sha256_requests[curr_rq].len() / 64;

            let temp_nb_chunks = nb_chunks + curr_rq_nb_chunks;

            // If curr_rq crosses over the 1024-chunk boundary, insert dummy chunks.
            if (temp_nb_chunks / 1024 != nb_chunks / 1024) && temp_nb_chunks % 1024 != 0 {
                while nb_chunks % 1024 != 0 {
                    let padded_input = self.curta_sha256_pad(&zero_chunk);
                    let bytes = self.bytes_to_target(&padded_input);

                    // Insert a dummy request and response.
                    self.sha256_requests.insert(curr_rq, bytes);
                    let digest = self.api.add_virtual_target_arr::<32>();
                    self.sha256_responses.insert(curr_rq, digest);

                    // Increment the number of requests and chunks accordingly.
                    curr_rq += 1;
                    num_rqs += 1;

                    nb_chunks += 1;
                }
            }
            nb_chunks += curr_rq_nb_chunks;
            curr_rq += 1;
        }

        // If the number of chunks is not a multiple of 1024, pad the gadget with dummy chunks.
        while nb_chunks % 1024 != 0 {
            self.curta_sha256(&zero_chunk);
            nb_chunks += 1;
        }

        // Allocate Curta SHA-256 gadgets according to the number of chunks across all requests.
        let gadgets: Vec<SHA256BuilderGadget<<L as PlonkParameters<D>>::Field, L::CubicParams, D>> =
            (0..nb_chunks / 1024)
                .map(|_| self.api.init_sha256())
                .collect_vec();
        debug!("allocated {} curta sha256 gadgets", gadgets.len());

        let mut rq_idx = 0;
        for i in 0..gadgets.len() {
            let mut gadget = gadgets[i].to_owned();

            // Fill the gadget with 1024 padded chunks.
            let mut num_chunks_so_far = 0;
            while num_chunks_so_far < 1024 {
                gadget
                    .padded_messages
                    .extend_from_slice(&self.sha256_requests[rq_idx]);
                let hint = SHA256HintGenerator::new(
                    &self.sha256_requests[rq_idx],
                    self.sha256_responses[rq_idx],
                );
                self.add_simple_generator(hint);
                gadget
                    .digests
                    .extend_from_slice(&self.sha256_responses[rq_idx]);
                gadget
                    .chunk_sizes
                    .push(self.sha256_requests[rq_idx].len() / 64);

                num_chunks_so_far += self.sha256_requests[rq_idx].len() / 64;
                rq_idx += 1;
            }

            self.api.constrain_sha256_gadget::<L::CurtaConfig>(gadget);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::backend::circuit::{CircuitBuild, DefaultParameters};
    use crate::frontend::uint::uint32::U32Variable;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{
        ByteVariable, BytesVariable, CircuitBuilder, GateRegistry, WitnessGeneratorRegistry,
    };
    use crate::utils::{bytes, bytes32};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta() {
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
        let generator_serializer = WitnessGeneratorRegistry::<L, D>::new();
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
        let generator_serializer = WitnessGeneratorRegistry::<L, D>::new();
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
}
