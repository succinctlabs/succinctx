use array_macro::array;

use crate::prelude::*;

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
        let k = 512 - (l + 1 + 64) % 512;
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
    ///
    /// It is assumed that `input` has length MAX_NUM_CHUNKS * 64.
    /// The true number of non-zero bytes in `input` is given by input_byte_length.
    pub(crate) fn pad_message_sha256_variable(
        &mut self,
        input: &[ByteVariable],
        input_byte_length: U32Variable,
    ) -> Vec<ByteVariable> {
        let true_t = self._true();
        let false_t = self._false();

        let max_number_of_chunks = input.len() / 64;
        assert_eq!(
            max_number_of_chunks * 64,
            input.len(),
            "input length must be a multiple of 64 bytes"
        );
        let last_chunk = self.compute_sha256_last_chunk(input_byte_length);

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
                .collect::<Vec<_>>(),
        );

        let mut padded_bytes = Vec::new();

        let mut message_byte_selector = true_t;
        for i in 0..max_number_of_chunks {
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
        self.is_equal(message_byte_selector, false_t);

        assert_eq!(padded_bytes.len(), max_number_of_chunks * 64);
        padded_bytes
    }
}
