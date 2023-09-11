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
/// MAX_NUM_CHUNKS is the maximum number of padded SHA-256 chunks that can be output.
pub fn sha256_pad_variable_length<
    L: PlonkParameters<D>,
    const D: usize,
    const MAX_NUM_CHUNKS: usize,
>(
    builder: &mut CircuitBuilder<L, D>,
    input: &[ByteVariable],
    last_chunk: U32Variable,
    input_byte_length: U32Variable,
) -> Vec<ByteVariable> {
    // TODO: Should be ArrayVariable<ByteVariable, MAX_NUM_CHUNKS * 64>
    let mut padded_bytes = Vec::new();

    let mut message_byte_selector = builder.constant::<BoolVariable>(true);
    for i in 0..MAX_NUM_CHUNKS {
        let chunk_offset = 64 * i;
        let curr_chunk = builder.constant::<U32Variable>(i as u32);

        let add_length_selector = builder.is_equal(curr_chunk, last_chunk);

        // Convert length_bytes into u64
        let mut length_bits = builder.api.split_le(input_byte_length.targets()[0], 64);
        length_bits.reverse();
        // Convert length_bits into [ByteVariable; 8]
        let length_bytes = length_bits
            .chunks(8)
            .map(|chunk| {
                let targets = chunk.iter().map(|b| b.target).collect_vec();
                ByteVariable::from_targets(&targets)
            })
            .collect_vec();

        for j in 0..64 {
            let idx = chunk_offset + j;
            let idx_t = builder.constant::<U32Variable>(idx as u32);
            let idx_length_eq = builder.is_equal(idx_t, input_byte_length);

            let not_idx_length_eq = builder.not(idx_length_eq);
            message_byte_selector = builder.select(
                message_byte_selector,
                not_idx_length_eq,
                message_byte_selector,
            );
            // If idx == length_bytes, then we want to select the length byte.
            let padding_start_byte = builder.constant::<ByteVariable>(0x80);
            let zero_byte = builder.constant::<ByteVariable>(0x00);

            let mut byte = builder.select(idx_length_eq, padding_start_byte, input[idx]);

            // If message_byte_selector is true, then we want to select the message byte.
            // If neither, then we want to select 0 byte.
            byte = builder.select(message_byte_selector, byte, zero_byte);

            if j >= 64 - 8 {
                // If add_length_selector is true, then we want to select the length byte.
                byte = builder.select(add_length_selector, length_bytes[j % 8], byte);
            }

            padded_bytes.push(byte);
        }
    }

    padded_bytes
}

/// Convert an array of ByteVariable to a Vec<Target>.
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
    /// Executes a SHA256 hash on the given input. Assumes the message is already padded.
    pub fn sha256_curta_variable<const MAX_NUM_CHUNKS: usize>(
        &mut self,
        input: &[ByteVariable],
        last_chunk: U32Variable,
        input_byte_length: U32Variable,
    ) -> Bytes32Variable {
        let expected_last_chunk = self.constant::<U32Variable>((MAX_NUM_CHUNKS - 1) as u32);
        // TODO: Currently, Curta does not support no-ops over SHA chunks.
        // Until Curta SHA-256 has no-ops, last_chunk should always be equal to MAX_NUM_CHUNKS - 1.
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
        let mut rq_idx = 0;
        let mut array_end_idx = self.sha256_requests.len();

        let zero = self.constant::<ByteVariable>(0u8);
        let zero_chunk = [zero; 1];

        while rq_idx < array_end_idx {
            let nb_chunks_in_rq = self.sha256_requests[rq_idx].len() / 64;

            let temp_nb_chunks = nb_chunks + nb_chunks_in_rq;

            // If temp_nb_chunks / 1024 != nb_chunks / 1024 & temp_nb_chunks % 1024 != 0 we need to zero pad
            if (temp_nb_chunks / 1024 != nb_chunks / 1024) && temp_nb_chunks % 1024 != 0 {
                while nb_chunks % 1024 != 0 {
                    // We want to insert a 0 chunk in here!
                    let padded_input = sha256_pad(self, &zero_chunk);
                    let bytes = bytes_to_target(self, &padded_input);

                    self.sha256_requests.insert(rq_idx, bytes);

                    let digest = self.api.add_virtual_target_arr::<32>();
                    self.sha256_responses.insert(rq_idx, digest);

                    // Increment request index because we've inserted, and also increment the end index
                    rq_idx += 1;
                    array_end_idx += 1;

                    nb_chunks += 1;
                }
            }
            nb_chunks += nb_chunks_in_rq;
            rq_idx += 1;
        }

        while nb_chunks % 1024 != 0 {
            self.sha256_curta(&zero_chunk);
            nb_chunks += 1;
        }

        let gadgets: Vec<SHA256BuilderGadget<<L as PlonkParameters<D>>::Field, L::CubicParams, D>> =
            (0..nb_chunks / 1024)
                .map(|_| self.api.init_sha256())
                .collect_vec();

        let mut rq_idx = 0;
        for i in 0..gadgets.len() {
            let mut gadget = gadgets[i].to_owned();

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
    use crate::prelude::{ByteVariable, CircuitBuilder};

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
}
