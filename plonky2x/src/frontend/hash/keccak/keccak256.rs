// Much of this file is copied from Jump Crypto's plonky2-crypto repo:
// https://github.com/JumpCrypto/plonky2-crypto/blob/main/src/hash/keccak256.rs

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;

use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::frontend::num::u32::gadgets::interleaved_u32::CircuitBuilderB32;
use crate::frontend::vars::{ByteVariable, Bytes32Variable, CircuitVariable};
use crate::prelude::{BoolVariable, BytesVariable, CircuitBuilder};

const KECCAK256_C: usize = 1600;
pub const KECCAK256_R: usize = 1088;

// This function will input a bit vector, and output the vector in
// the opposite endian byte ordering. In other words, it takes 8-bit chunks
// of the input vector and reverses the ordering of those chunks.
// So for an BE byte ordered bit vector, it will output a LE byte ordered bit vector
// and vice-versa.
fn reverse_byte_ordering(input_vec: Vec<BoolTarget>) -> Vec<BoolTarget> {
    assert!(input_vec.len() % 8 == 0);

    let mut le_ordered_bits = Vec::new();
    for byte_chunk in input_vec.as_slice().chunks(8).rev() {
        le_ordered_bits.extend_from_slice(byte_chunk);
    }

    le_ordered_bits
}

// This function create a circuit to output a will accept a bit vector that is in little endian byte order
// and will output a BigUintTarget.
fn biguint_from_le_bytes<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    bits: Vec<BoolTarget>, // bits is in little-endian byte order, but big endian bit order
) -> BigUintTarget {
    assert!(bits.len() % 32 == 0);

    let be_byte_ordered_bits = reverse_byte_ordering(bits);

    // Convert to BigUintTarget.
    // Note that the limbs within the BigUintTarget are in little endian ordering, so
    // the least significant u32 should be processed first.
    let mut u32_targets = Vec::new();
    for u32_chunk in be_byte_ordered_bits.as_slice().chunks(32).rev() {
        // The chunk's bit ordering is in BE.  Need to reverse it for the le_sum function.
        u32_targets.push(U32Target(builder.api.le_sum(u32_chunk.iter().rev())));
    }

    BigUintTarget { limbs: u32_targets }
}

#[rustfmt::skip]
pub const KECCAKF_ROTC: [u8; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62,
    18, 39, 61, 20, 44
];

#[rustfmt::skip]
pub const KECCAKF_PILN: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20,
    14, 22, 9, 6, 1
];

#[rustfmt::skip]
pub const KECCAKF_RNDC: [[u32; 2]; 24] = [
    [0x00000001, 0x00000000], [0x00008082, 0x00000000],
    [0x0000808A, 0x80000000], [0x80008000, 0x80000000],
    [0x0000808B, 0x00000000], [0x80000001, 0x00000000],
    [0x80008081, 0x80000000], [0x00008009, 0x80000000],
    [0x0000008A, 0x00000000], [0x00000088, 0x00000000],
    [0x80008009, 0x00000000], [0x8000000A, 0x00000000],
    [0x8000808B, 0x00000000], [0x0000008B, 0x80000000],
    [0x00008089, 0x80000000], [0x00008003, 0x80000000],
    [0x00008002, 0x80000000], [0x00000080, 0x80000000],
    [0x0000800A, 0x00000000], [0x8000000A, 0x80000000],
    [0x80008081, 0x80000000], [0x00008080, 0x80000000],
    [0x80000001, 0x00000000], [0x80008008, 0x80000000],
];

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    fn _hash_keccak256_f1600(&mut self, s: &mut [[U32Target; 2]; 25]) {
        let zero = self.api.zero_u32();
        let mut bc = [[zero; 2]; 5];

        let mut keccakf_rndc = Vec::new();
        for item in &KECCAKF_RNDC {
            keccakf_rndc.push([
                self.api.constant_u32(item[0]),
                self.api.constant_u32(item[1]),
            ]);
        }

        // for round in 0..24 {
        for rndc in keccakf_rndc.iter().take(24) {
            // Theta
            for i in 0..5 {
                bc[i] = self.api.unsafe_xor_many_u64(&[
                    s[i],
                    s[i + 5],
                    s[i + 10],
                    s[i + 15],
                    s[i + 20],
                ]);
            }

            for i in 0..5 {
                let t1 = self.api.lrot_u64(&bc[(i + 1) % 5], 1);
                let t2 = self.api.xor_u64(&bc[(i + 4) % 5], &t1);
                for j in 0..5 {
                    s[5 * j + i] = self.api.xor_u64(&s[5 * j + i], &t2);
                }
            }

            // Rho Pi
            let mut t = s[1];
            for i in 0..24 {
                let j = KECCAKF_PILN[i];
                let tmp = s[j];
                s[j] = self.api.lrot_u64(&t, KECCAKF_ROTC[i]);
                t = tmp;
            }

            // Chi
            for j in 0..5 {
                for i in 0..5 {
                    bc[i] = s[5 * j + i];
                }
                for i in 0..5 {
                    let t1 = self.api.not_u64(&bc[(i + 1) % 5]);
                    let t2 = self.api.and_u64(&bc[(i + 2) % 5], &t1);
                    s[5 * j + i] = self.api.xor_u64(&s[5 * j + i], &t2);
                }
            }

            // Iota
            s[0] = self.api.xor_u64(&s[0], rndc);
        }
    }

    pub fn keccak256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let input_len_bits = input.len() * 8;
        let num_actual_blocks = 1 + input_len_bits / KECCAK256_R;
        let padded_len_bits = num_actual_blocks * KECCAK256_R;

        // Convert to bits.
        let mut bits = Vec::new();
        for byte in input.iter() {
            bits.extend(byte.to_le_bits());
        }

        // Add the padding
        bits.push(BoolVariable::constant(self, true));
        for _ in bits.len()..padded_len_bits - 1 {
            bits.push(BoolVariable::constant(self, false));
        }
        bits.push(BoolVariable::constant(self, true));

        // Convert the bits into bool targets since we are using a gadget using plonky2's circuit builder
        let bool_targets = bits
            .into_iter()
            .map(|b| BoolTarget::new_unsafe(b.0 .0))
            .collect::<Vec<_>>();

        let input_biguint = biguint_from_le_bytes(self, bool_targets);

        let chunks_len = KECCAK256_R / 64;
        let zero = self.api.zero_u32();
        let mut state = [[zero; 2]; KECCAK256_C / 64];
        let mut next_state = [[zero; 2]; KECCAK256_C / 64];

        // first block. xor = use input as initial state
        for (i, s) in state.iter_mut().enumerate().take(chunks_len) {
            s[0] = input_biguint.limbs[2 * i];
            s[1] = input_biguint.limbs[2 * i + 1];
        }

        self._hash_keccak256_f1600(&mut state);

        // other blocks
        for k in 0..num_actual_blocks - 1 {
            // xor
            let input_start = (k + 1) * chunks_len * 2;
            for (i, s) in state.iter().enumerate() {
                if i < chunks_len {
                    next_state[i][0] = self
                        .api
                        .xor_u32(s[0], input_biguint.limbs[input_start + i * 2]);
                    next_state[i][1] = self
                        .api
                        .xor_u32(s[1], input_biguint.limbs[input_start + i * 2 + 1]);
                } else {
                    next_state[i][0] = s[0];
                    next_state[i][1] = s[1];
                }
            }

            self._hash_keccak256_f1600(&mut next_state);

            // conditionally set old or new state, depending if block needs to be processed
            for (i, s) in next_state.iter().enumerate() {
                state[i] = *s;
            }
        }

        let output = self.api.add_virtual_biguint_target(8);

        // squeeze
        let output_len = output.num_limbs();
        for (i, s) in state.iter().enumerate().take(output_len / 2) {
            self.api.connect_u32(s[0], output.limbs[2 * i]);
            self.api.connect_u32(s[1], output.limbs[2 * i + 1]);
        }

        assert!(output.num_limbs() == 8);

        // Convert the bigint's u32 limbs into 8 bytes.
        let hash_bytes_vec = output
            .limbs
            .iter()
            .flat_map(|chunk| {
                let bit_list = self.api.split_le(chunk.0, 32);

                let hash_byte_vec = bit_list
                    .chunks(8)
                    .map(|chunk| ByteVariable(array![i => BoolVariable::from(chunk[i].target); 8]))
                    .collect::<Vec<_>>();

                hash_byte_vec
            })
            .collect::<Vec<_>>();

        let mut hash_bytes_array = [ByteVariable::init(self); 32];
        hash_bytes_array.copy_from_slice(&hash_bytes_vec);

        Bytes32Variable(BytesVariable(hash_bytes_array))
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use super::*;
    use crate::prelude::CircuitBuilder;
    use crate::utils::bytes32;

    #[test]
    fn test_keccak256() {
        env_logger::init();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let word = builder.constant::<Bytes32Variable>(bytes32!(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        ));
        let hash = builder.keccak256(&word.0 .0);
        builder.watch(&hash, "hi");

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (_, _) = circuit.prove(&input);
    }
}
