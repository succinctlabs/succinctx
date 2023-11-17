use plonky2::iop::target::BoolTarget;
use plonky2::util::ceil_div_usize;

use crate::prelude::*;

pub const SELECT_CHUNK_SIZE_64: usize = 64;
pub const LENGTH_BITS_128: usize = 128;
pub const SHA512_CHUNK_SIZE_BYTES_128: usize = 128;
pub const CHUNK_BITS_1024: usize = 1024;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub(crate) fn pad_message_sha512(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
        let bits = input
            .iter()
            .flat_map(|b| b.as_bool_targets().map(|x| x.target).to_vec())
            .collect::<Vec<_>>();

        let mut bit_targets = Vec::new();
        bit_targets.extend_from_slice(&bits);

        // TODO: Range check size of msg_bit_len?
        // Cast to u128 for bitmask
        let msg_bit_len: u128 = bits.len().try_into().expect("message too long");

        // minimum_padding = 1 + 128 (min 1 bit for the pad, and 128 bit for the msg size)
        let msg_with_min_padding_len = msg_bit_len as usize + LENGTH_BITS_128 + 1;

        let additional_padding_len = CHUNK_BITS_1024 - (msg_with_min_padding_len % CHUNK_BITS_1024);

        bit_targets.push(self.api.constant_bool(true).target);
        for _i in 0..additional_padding_len {
            bit_targets.push(self.api.constant_bool(false).target);
        }

        for i in (0..128).rev() {
            let has_bit = (msg_bit_len & (1 << i)) != 0;
            bit_targets.push(self.api.constant_bool(has_bit).target);
        }

        // Combine the bits into ByteVariable
        (0..bit_targets.len() / 8)
            .map(|i| ByteVariable::from_targets(&bit_targets[i * 8..(i + 1) * 8]))
            .collect::<Vec<_>>()
    }

    /// Calculates the last valid SHA512 chunk of an input_byte_length long message.
    /// This is useful for padding the message correctly for variable length inputs.
    pub(crate) fn compute_sha512_last_chunk(
        &mut self,
        input_byte_length: U32Variable,
    ) -> U32Variable {
        // 17 is the number of bytes added by the padding and LE length representation. Subtract 1
        // to account for the case where input.len() + 17 % 128 == 0, in which case an extra chunk is
        // not needed. Divide by 128 (chunk size in bytes) to get the number of chunks.
        let padding_and_length = self.constant::<U32Variable>((17 - 1) as u32);
        let chunk_size = self.constant::<U32Variable>(128);

        let total_length = self.add(input_byte_length, padding_and_length);
        self.div(total_length, chunk_size)
    }

    /// Pads the input according to the SHA512 specification.
    pub(crate) fn pad_sha512_variable_length(
        &mut self,
        input: &[ByteVariable],
        input_byte_length: U32Variable,
    ) -> Vec<ByteVariable> {
        let last_chunk = self.compute_sha512_last_chunk(input_byte_length);

        // Calculate the number of chunks needed to store the input. 17 is the number of bytes added
        // by the padding and LE length representation.
        let max_num_chunks = ceil_div_usize(input.len() + 17, SHA512_CHUNK_SIZE_BYTES_128);

        // Extend input to size max_num_chunks * 128 before padding.
        let mut padded_input = input.to_vec();
        padded_input.resize(max_num_chunks * SHA512_CHUNK_SIZE_BYTES_128, self.zero());

        let message = padded_input
            .iter()
            .flat_map(|b| b.as_bool_targets().to_vec())
            .collect::<Vec<_>>();

        let mut msg_input = Vec::new();

        let eight = self.constant::<U32Variable>(8);
        let hash_msg_length_bits = self.mul(input_byte_length, eight).variable.0;

        let mut length_bits = self.api.split_le(hash_msg_length_bits, 128);
        // Convert length to BE bits
        length_bits.reverse();

        let last_chunk = last_chunk.variable.0;
        let mut add_message_bit_selector = self.api.constant_bool(true);
        for i in 0..max_num_chunks {
            let chunk_offset = CHUNK_BITS_1024 * i;
            let curr_chunk_t = self.api.constant(L::Field::from_canonical_usize(i));
            // Check if this is the chunk where length should be added
            let add_length_bit_selector = self.api.is_equal(last_chunk, curr_chunk_t);
            // Always message || padding || nil
            for j in 0..CHUNK_BITS_1024 - LENGTH_BITS_128 {
                let idx = chunk_offset + j;

                let idx_t = self.api.constant(L::Field::from_canonical_usize(idx));
                let idx_length_eq_t = self.api.is_equal(idx_t, hash_msg_length_bits);

                // select_bit AND NOT(idx_length_eq_t)
                let not_idx_length_eq_t = self.api.not(idx_length_eq_t);
                add_message_bit_selector = BoolTarget::new_unsafe(self.api.select(
                    add_message_bit_selector,
                    not_idx_length_eq_t.target,
                    add_message_bit_selector.target,
                ));

                // Set bit to push: (select_bit && message[i]) || idx_length_eq_t
                let bit_to_push = self.api.and(add_message_bit_selector, message[idx]);
                let bit_to_push = self.api.or(idx_length_eq_t, bit_to_push);
                msg_input.push(bit_to_push);
            }

            // Message || padding || length || nil
            for j in CHUNK_BITS_1024 - LENGTH_BITS_128..CHUNK_BITS_1024 {
                let idx = chunk_offset + j;

                // Only true if in the last valid chunk
                let length_bit = self
                    .api
                    .and(length_bits[j % LENGTH_BITS_128], add_length_bit_selector);

                // TODO: add_length_bit_selector && (add_message_bit_selector || length_bit) should never be true concurrently -> add constraint for this?

                let idx_t = self.api.constant(L::Field::from_canonical_usize(idx));
                let idx_length_eq_t = self.api.is_equal(idx_t, hash_msg_length_bits);

                // select_bit AND NOT(idx_length_eq_t)
                let not_idx_length_eq_t = self.api.not(idx_length_eq_t);
                add_message_bit_selector = BoolTarget::new_unsafe(self.api.select(
                    add_message_bit_selector,
                    not_idx_length_eq_t.target,
                    add_message_bit_selector.target,
                ));

                // Set bit to push: (select_bit && message[i]) || idx_length_eq_t
                let bit_to_push = self.api.and(add_message_bit_selector, message[idx]);
                let bit_to_push = self.api.or(idx_length_eq_t, bit_to_push);

                let bit_to_push = self.api.or(length_bit, bit_to_push);

                // Either length bit || (message[i] || idx_length_eq_t)
                msg_input.push(bit_to_push);
            }
        }

        // Combine the bits into ByteVariable
        let output_bits = msg_input.iter().map(|b| b.target).collect::<Vec<_>>();
        (0..output_bits.len() / 8)
            .map(|i| ByteVariable::from_targets(&output_bits[i * 8..(i + 1) * 8]))
            .collect::<Vec<_>>()
    }
}
