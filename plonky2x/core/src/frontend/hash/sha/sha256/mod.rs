use ::curta::machine::hash::sha::algorithm::SHAPure;
use ::curta::machine::hash::sha::sha256::SHA256;
use array_macro::array;
/// Implementation of sha256
/// reference: https://github.com/thomdixon/pysha2/blob/master/sha2/sha256.py
use itertools::Itertools;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::hash::common::{and_arr, not_arr, xor2_arr, xor3_arr};
use crate::frontend::vars::{BoolVariable, ByteVariable, Bytes32Variable, CircuitVariable};
use crate::prelude::{U32Variable, Variable};

pub mod curta;

/// Implements SHA256 implementation for CircuitBuilder
impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Pad the given input according to the SHA-256 spec.
    /// The last chunk (each chunk is 64 bytes = 512 bits) gets padded.
    pub(crate) fn pad_message_sha256(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
        let mut bits = input
            .iter()
            .flat_map(|b| b.as_bool_targets().to_vec())
            .collect_vec();
        bits.push(self.api._true());

        let l = bits.len() - 1;
        let k = 512 - (l + 1 + 64) % 512; // number of padding bits you need
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

        let bit_targets = bits.iter().map(|b| b.target).collect_vec();

        // Combine the bits into ByteVariable
        (0..bit_targets.len() / 8)
            .map(|i| ByteVariable::from_targets(&bit_targets[i * 8..(i + 1) * 8]))
            .collect_vec()
    }

    /// Pad the given variable length input according to the SHA-256 spec.
    ///
    /// It is assumed that `input` has length MAX_NUM_CHUNKS * 64.
    /// The true number of non-zero bytes in `input` is given by input_byte_length.
    ///
    /// 'last_chunk' is the index of the last chunk in `input`. Its value should be given by
    /// (input_byte_length + 9)/64 It is assumed that the last chunk is calculated correctly.
    fn pad_message_sha256_variable(
        &mut self,
        input: &[ByteVariable],
        input_byte_length: U32Variable,
        last_chunk: U32Variable,
    ) -> (Vec<ByteVariable>, Variable) {
        let max_number_of_chunks = input.len() / 64;
        assert_eq!(
            max_number_of_chunks * 64,
            input.len(),
            "input length must be a multiple of 64 bytes"
        );
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

        assert_eq!(padded_bytes.len(), max_number_of_chunks * 64);
        (padded_bytes, last_chunk.variable)
    }

    fn const_be_bits(&mut self, u: u32) -> [BoolVariable; 32] {
        u.to_be_bytes()
            .iter()
            .flat_map(|b| self.constant::<ByteVariable>(*b).as_be_bits().to_vec())
            .collect::<Vec<BoolVariable>>()
            .try_into()
            .unwrap()
    }

    fn get_inital_hash(&mut self) -> [[BoolVariable; 32]; 8] {
        SHA256::INITIAL_HASH.map(|x| self.const_be_bits(x))
    }

    fn get_round_constants(&mut self) -> [[BoolVariable; 32]; 64] {
        SHA256::ROUND_CONSTANTS.map(|x| self.const_be_bits(x))
    }

    fn process_padded_message(&mut self, msg_input: &[ByteVariable]) -> Vec<BoolVariable> {
        let msg_input_bits = msg_input
            .iter()
            .flat_map(|b| b.as_be_bits().to_vec())
            .collect_vec();
        let mut sha256_hash = self.get_inital_hash();
        let round_constants = self.get_round_constants();

        // Process the input with 512 bit chunks aka 64 byte chunks
        for chunk_start in (0..msg_input_bits.len()).step_by(512) {
            let chunk = msg_input_bits[chunk_start..chunk_start + 512].to_vec();
            let mut u: Vec<BoolVariable> = Vec::new();

            for bit in chunk.iter().take(512) {
                // 0 .. 16 chunk size * 32 bits7
                u.push(*bit);
            }
            for _ in 512..64 * 32 {
                // 16 * 8 ... 64 * 8 because of L
                u.push(self._false());
            }

            let mut w = self.reshape(u);

            for i in 16..64 {
                let s0 = xor3_arr(
                    self._right_rotate(w[i - 15], 7),
                    self._right_rotate(w[i - 15], 18),
                    self._shr(w[i - 15], 3),
                    self,
                );

                let s1 = xor3_arr(
                    self._right_rotate(w[i - 2], 17),
                    self._right_rotate(w[i - 2], 19),
                    self._shr(w[i - 2], 10),
                    self,
                );

                let inter1 = self.add_arr(w[i - 16], s0);
                let inter2 = self.add_arr(inter1, w[i - 7]);
                w[i] = self.add_arr(s1, inter2);
            }
            let mut a = sha256_hash[0];
            let mut b = sha256_hash[1];
            let mut c = sha256_hash[2];
            let mut d = sha256_hash[3];
            let mut e = sha256_hash[4];
            let mut f = sha256_hash[5];
            let mut g = sha256_hash[6];
            let mut h = sha256_hash[7];

            for i in 0..64 {
                let sum1 = xor3_arr(
                    self._right_rotate(e, 6),
                    self._right_rotate(e, 11),
                    self._right_rotate(e, 25),
                    self,
                );
                let ch = xor2_arr(
                    and_arr(e, f, self),
                    and_arr(not_arr(e, self), g, self),
                    self,
                );
                let temp1 = self.add_arr(h, sum1);
                let temp2 = self.add_arr(temp1, ch);
                let temp3 = self.add_arr(temp2, round_constants[i]);
                let temp4 = self.add_arr(temp3, w[i]);
                let final_temp1 = temp4;

                let sum0 = xor3_arr(
                    self._right_rotate(a, 2),
                    self._right_rotate(a, 13),
                    self._right_rotate(a, 22),
                    self,
                );

                let maj = xor3_arr(
                    and_arr(a, b, self),
                    and_arr(a, c, self),
                    and_arr(b, c, self),
                    self,
                );
                let final_temp2 = self.add_arr(sum0, maj);

                h = g;
                g = f;
                f = e;
                e = self.add_arr(d, final_temp1);
                d = c;
                c = b;
                b = a;
                a = self.add_arr(final_temp1, final_temp2);
            }

            sha256_hash = self.zip_add(sha256_hash, [a, b, c, d, e, f, g, h]);
        }

        sha256_hash.iter().flat_map(|x| x.to_vec()).collect()
    }

    pub fn sha256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let padded = self.pad_message_sha256(input);
        let bool_digest: Vec<BoolVariable> = self.process_padded_message(&padded);
        assert_eq!(bool_digest.len(), 256);
        // Ok to use `from_variables_unsafe` as we know `process_padded_message` returns 256 bits
        Bytes32Variable::from_variables_unsafe(
            &bool_digest.iter().map(|b| b.variable).collect_vec(),
        )
    }

    pub fn sha256_pair(
        &mut self,
        left: Bytes32Variable,
        right: Bytes32Variable,
    ) -> Bytes32Variable {
        let mut left_bytes = left.as_bytes().to_vec();
        left_bytes.extend(&right.as_bytes());
        self.sha256(&left_bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::prelude::{ByteVariable, CircuitBuilder, DefaultParameters};
    use crate::utils::hash::sha256;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_bench() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let messages = [
            vec![2u8],
            vec![4u8, 19u8, 100u8, 45u8],
            vec![
                4u8, 19u8, 100u8, 45u8, 4u8, 19u8, 100u8, 45u8, 4u8, 19u8, 100u8, 45u8, 4u8, 19u8,
                100u8, 45u8, 4u8, 19u8, 100u8, 45u8,
            ],
        ];
        for message in messages.iter() {
            let expected_result = sha256(message);

            let mut builder = CircuitBuilder::<L, D>::new();
            let mut message_variables = Vec::new();
            for m in message.iter() {
                message_variables.push(builder.constant::<ByteVariable>(*m));
            }
            let result = builder.sha256(&message_variables);
            let expected_digest = builder.constant::<Bytes32Variable>(expected_result.into());
            builder.assert_is_equal(result, expected_digest);

            let circuit = builder.build();
            let input = circuit.input();
            let (proof, output) = circuit.prove(&input);
            circuit.verify(&proof, &input, &output);
            circuit.test_default_serializers();
        }
    }

    #[test]
    #[should_panic]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_failure() {
        todo!();
    }
}
