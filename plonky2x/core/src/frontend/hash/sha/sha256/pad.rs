use array_macro::array;
use plonky2::util::ceil_div_usize;

use crate::prelude::*;

pub const SHA256_CHUNK_SIZE_BYTES: usize = 64;
pub const SHA256_INPUT_LENGTH_BYTE_SIZE: usize = 8;

pub const SHA256_CHUNK_SIZE_BITS: usize = SHA256_CHUNK_SIZE_BYTES * 8;
pub const SHA256_INPUT_LENGTH_BIT_SIZE: usize = SHA256_INPUT_LENGTH_BYTE_SIZE * 8;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Pad the given input according to the SHA-256 spec.
    /// The last chunk (each chunk is 64 bytes = 512 bits) gets padded.
    pub(crate) fn pad_message_sha256(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
        let mut bits = input
            .iter()
            .flat_map(|b| b.as_bool_targets().to_vec())
            .collect::<Vec<_>>();
        bits.push(self.api._true());

        let l = bits.len() - 1;
        let k = SHA256_CHUNK_SIZE_BITS
            - (l + 1 + SHA256_INPUT_LENGTH_BIT_SIZE) % SHA256_CHUNK_SIZE_BITS;
        for _ in 0..k {
            bits.push(self.api._false());
        }

        (l as u64)
            .to_be_bytes()
            .iter()
            .map(|b| self.constant::<ByteVariable>(*b))
            .for_each(|b| {
                bits.extend_from_slice(&b.as_bool_targets());
            });

        let bit_targets = bits.iter().map(|b| b.target).collect::<Vec<_>>();

        // Combine the bits into ByteVariable
        (0..bit_targets.len() / 8)
            .map(|i| ByteVariable::from_targets(&bit_targets[i * 8..(i + 1) * 8]))
            .collect()
    }

    /// Calculates the last valid SHA256 chunk of an input_byte_length long message.
    /// This is useful for padding the message correctly for variable length inputs.
    pub(crate) fn compute_sha256_last_chunk(
        &mut self,
        input_byte_length: U32Variable,
    ) -> U32Variable {
        // 9 is the number of bytes added by the padding and LE length representation. Subtract 1
        // to account for the case where input.len() + 9 % 64 == 0, in which case an extra chunk is
        // not needed. Divide by 64 to get the number of chunks.
        let padding_and_length = self.constant::<U32Variable>((9 - 1) as u32);
        let chunk_size = self.constant::<U32Variable>(64);

        let total_length = self.add(input_byte_length, padding_and_length);
        self.div(total_length, chunk_size)
    }

    /// Pad the given variable length input according to the SHA-256 spec.
    /// input_byte_length gives the real length of the input in bytes.
    pub(crate) fn pad_sha256_variable_length(
        &mut self,
        input: &[ByteVariable],
        input_byte_length: U32Variable,
    ) -> Vec<ByteVariable> {
        let true_t = self._true();
        let false_t = self._false();

        // Check that input_byte_length <= input.len(). This is needed to ensure that users cannot
        // prove the hash of a longer message than they supplied.
        let supplied_input_length = self.constant::<U32Variable>(input.len() as u32);
        self.lte(input_byte_length, supplied_input_length);

        let last_chunk = self.compute_sha256_last_chunk(input_byte_length);

        // Calculate the number of chunks needed to store the input. 9 bytes are added by the
        // padding and LE length representation.
        let max_num_chunks = ceil_div_usize(
            input.len() + SHA256_INPUT_LENGTH_BYTE_SIZE + 1,
            SHA256_CHUNK_SIZE_BYTES,
        );

        // Extend input to size max_num_chunks * 64 before padding.
        let mut padded_input = input.to_vec();
        padded_input.resize(max_num_chunks * SHA256_CHUNK_SIZE_BYTES, self.zero());

        // Compute the length bytes (big-endian representation of the length in bits).
        let zero_byte = self.constant::<ByteVariable>(0x00);

        let bits_per_byte = self.constant::<U32Variable>(8);
        let input_bit_length = self.mul(input_byte_length, bits_per_byte);

        // Get the length bits in LE order, padded to 64 bits.
        let mut length_bits = self
            .api
            .split_le(input_bit_length.variable.0, SHA256_INPUT_LENGTH_BIT_SIZE);
        // Convert length to BE bits
        length_bits.reverse();

        let length_bytes = &length_bits
            .chunks(8)
            .map(|chunk| {
                let bits = array![x => BoolVariable::from_targets(&[chunk[x].target]); 8];
                ByteVariable(bits)
            })
            .collect::<Vec<_>>();

        let mut padded_bytes = Vec::new();

        // Set to true if the last chunk has been reached. This is used to verify that
        // input_byte_length is <= input.len().
        let mut reached_last_chunk = false_t;

        let mut message_byte_selector = true_t;
        for i in 0..max_num_chunks {
            let chunk_offset = SHA256_CHUNK_SIZE_BYTES * i;
            let curr_chunk = self.constant::<U32Variable>(i as u32);

            // Check if this is the chunk where length should be added.
            let is_last_chunk = self.is_equal(curr_chunk, last_chunk);
            reached_last_chunk = self.or(reached_last_chunk, is_last_chunk);

            for j in 0..SHA256_CHUNK_SIZE_BYTES {
                // First 64 - 8 bytes are either message | padding | nil bytes.
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
                let mut byte = self.select(message_byte_selector, padded_input[idx], zero_byte);
                // If idx == length_bytes, select the padding start byte.
                byte = self.select(is_last_msg_byte, padding_start_byte, byte);

                if j >= SHA256_CHUNK_SIZE_BYTES - SHA256_INPUT_LENGTH_BYTE_SIZE {
                    // If in last chunk, this is a length byte.
                    byte = self.select(
                        is_last_chunk,
                        length_bytes[j % SHA256_INPUT_LENGTH_BYTE_SIZE],
                        byte,
                    );
                }

                padded_bytes.push(byte);
            }
        }
        // These checks verify input_byte_length <= input.len().
        self.is_equal(message_byte_selector, false_t);
        self.is_equal(reached_last_chunk, true_t);

        padded_bytes
    }
}
