use curta::chip::hash::sha::sha256::builder_gadget::{SHA256Builder, SHA256BuilderGadget};
use curta::chip::hash::sha::sha256::generator::SHA256HintGenerator;
use curta::math::field::Field;
use itertools::Itertools;
use plonky2::iop::target::Target;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use crate::backend::config::PlonkParameters;
use crate::frontend::hash::bit_operations::util::u64_to_bits;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, CircuitVariable};

/// Pad the given input according to the SHA-256 spec.
pub fn sha256_pad<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    input: &[ByteVariable],
) -> Vec<ByteVariable> {
    let mut bits = input
        .iter()
        .flat_map(|b| b.as_bool_targets().to_vec())
        .collect_vec();
    bits.push(builder.api._true());

    let l = bits.len() - 1;
    let mut k = 0;
    while (l + 1 + k + 64) % 512 != 0 {
        k += 1;
    }
    for _ in 0..k {
        bits.push(builder.api._false());
    }

    let be_bits = u64_to_bits(l as u64, &mut builder.api);
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
/// MAX_NUM_CHUNKS is the maximum number of output chunks that this function will pad.
pub fn sha256_pad_variable_length<
    L: PlonkParameters<D>,
    const D: usize,
    const MAX_NUM_CHUNKS: usize,
>(
    builder: &mut CircuitBuilder<L, D>,
    // Input should be length MAX_NUM_CHUNKS * 64.
    input: &[ByteVariable],
    last_chunk: U32Variable,
    // Input byte length should be at most MAX_NUM_CHUNKS * 64 - 9.
    input_byte_length: U32Variable,
) -> Vec<ByteVariable> {
    // Compute the length bytes (big-endian representation of the length in bits).
    let bits_per_byte = builder.constant::<U32Variable>(8);
    let input_bit_length = builder.mul(input_byte_length, bits_per_byte);
    let mut length_bits = builder.api.split_le(input_bit_length.0 .0, 64);
    length_bits.reverse();
    let length_bytes = length_bits
        .chunks(8)
        .map(|chunk| {
            let targets = chunk.iter().map(|b| b.target).collect_vec();
            ByteVariable::from_targets(&targets)
        })
        .collect_vec();

    // TODO: Use fixed size array instead of Vec. (MAX_NUM_CHUNKS * 64)
    let mut padded_bytes = Vec::new();

    let mut message_byte_selector = builder.constant::<BoolVariable>(true);
    for i in 0..MAX_NUM_CHUNKS {
        let chunk_offset = 64 * i;
        let curr_chunk = builder.constant::<U32Variable>(i as u32);

        let is_last_chunk = builder.is_equal(curr_chunk, last_chunk);

        for j in 0..64 {
            let idx = chunk_offset + j;
            let idx_t = builder.constant::<U32Variable>(idx as u32);
            let is_last_msg_byte = builder.is_equal(idx_t, input_byte_length);
            let not_last_msg_byte = builder.not(is_last_msg_byte);

            message_byte_selector = builder.select(
                message_byte_selector,
                not_last_msg_byte,
                message_byte_selector,
            );

            let padding_start_byte = builder.constant::<ByteVariable>(0x80);
            let zero_byte = builder.constant::<ByteVariable>(0x00);

            // If message_byte_selector is true, select the message byte.
            let mut byte = builder.select(message_byte_selector, input[idx], zero_byte);
            // If idx == length_bytes, select the padding start byte.
            byte = builder.select(is_last_msg_byte, padding_start_byte, byte);
            if j >= 64 - 8 {
                // If in last chunk, select the length byte.
                byte = builder.select(is_last_chunk, length_bytes[j % 8], byte);
            }

            padded_bytes.push(byte);
        }
    }

    padded_bytes
}

pub fn bytes_to_target<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    input: &[ByteVariable],
) -> Vec<Target> {
    let mut bytes = Vec::new();
    for i in 0..input.len() {
        let mut byte = builder.api.zero();
        let targets = input[i].targets();
        for j in 0..8 {
            let bit = targets[j];
            byte = builder
                .api
                .mul_const_add(L::Field::from_canonical_u8(1 << (7 - j)), bit, byte);
        }
        bytes.push(byte);
    }
    bytes
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Executes a SHA256 hash on the given input.
    /// input should be length MAX_NUM_CHUNKS * 64.
    /// input_byte_length should be at most MAX_NUM_CHUNKS * 64 - 9.
    pub fn sha256_curta_variable<const MAX_NUM_CHUNKS: usize>(
        &mut self,
        input: &[ByteVariable],
        last_chunk: U32Variable,
        input_byte_length: U32Variable,
    ) -> Bytes32Variable {
        // TODO: Currently, Curta does not support no-ops over SHA chunks. Until Curta SHA-256 supports no-ops, last_chunk should always be equal to MAX_NUM_CHUNKS - 1.
        let expected_last_chunk = self.constant::<U32Variable>((MAX_NUM_CHUNKS - 1) as u32);
        self.assert_is_equal(expected_last_chunk, last_chunk);

        let padded_input = sha256_pad_variable_length::<L, D, MAX_NUM_CHUNKS>(
            self,
            input,
            last_chunk,
            input_byte_length,
        );

        let bytes = bytes_to_target(self, &padded_input);

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

    /// Executes a SHA256 hash on the given input. (Assumes it's not padded)
    pub fn sha256_curta(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let padded_input = sha256_pad(self, input);

        let bytes = bytes_to_target(self, &padded_input);

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

    pub fn constraint_sha256_curta(&mut self)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut nb_chunks = 0;
        let mut curr_rq = 0;
        let mut num_rqs = self.sha256_requests.len();

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
                    let padded_input = sha256_pad(self, &zero_chunk);
                    let bytes = bytes_to_target(self, &padded_input);

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
            nb_chunks += curr_rq;
            curr_rq += 1;
        }

        // If the number of chunks is not a multiple of 1024, pad the gadget with dummy chunks.
        while nb_chunks % 1024 != 0 {
            self.sha256_curta(&zero_chunk);
            nb_chunks += 1;
        }

        // Allocate Curta SHA-256 gadgets according to the number of chunks across all requests.
        let gadgets: Vec<SHA256BuilderGadget<<L as PlonkParameters<D>>::Field, L::CubicParams, D>> =
            (0..nb_chunks / 1024)
                .map(|_| self.api.init_sha256())
                .collect_vec();

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

            self.api.constrain_sha256_gadget::<L::Config>(gadget);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::backend::config::DefaultParameters;
    use crate::frontend::uint::uint32::U32Variable;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{ByteVariable, BytesVariable, CircuitBuilder};
    use crate::utils::{bytes, bytes32};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.constant::<ByteVariable>(0u8);
        let result = builder.sha256_curta(&[zero; 1]);
        builder.watch(&result, "result");
        builder.constraint_sha256_curta();

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        // TODO: Add back once curta serializes as intended.
        // circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_variable_single() {
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

        let msg_hash = builder.sha256_curta_variable::<1>(&msg.0, last_chunk, bytes_length);
        builder.watch(&msg_hash, "msg_hash");

        builder.assert_is_equal(msg_hash, expected_digest);

        builder.constraint_sha256_curta();

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
