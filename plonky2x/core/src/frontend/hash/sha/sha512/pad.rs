use crate::prelude::*;

pub const SELECT_CHUNK_SIZE_64: usize = 64;
pub const LENGTH_BITS_128: usize = 128;
pub const CHUNK_BITS_1024: usize = 1024;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub(crate) fn pad_message_sha512(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
        let mut bits = input
            .iter()
            .flat_map(|b| b.as_bool_targets().map(|x| x.target).to_vec())
            .collect::<Vec<_>>();
        bits.push(self.api._true().target);

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
}
