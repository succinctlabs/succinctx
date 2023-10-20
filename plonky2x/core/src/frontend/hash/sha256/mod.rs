/// Implementation of sha256
/// reference: https://github.com/thomdixon/pysha2/blob/master/sha2/sha256.py
use itertools::Itertools;
use num::traits::ToBytes;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::hash::common::{and_arr, not_arr, xor2_arr, xor3_arr};
use crate::frontend::vars::{BoolVariable, ByteVariable, Bytes32Variable, CircuitVariable};

mod consts;
pub mod curta;

/// Implements SHA256 implementation for CircuitBuilder
impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Pad the given input according to the SHA-256 spec.
    fn pad_message_sha256(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
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

        l.to_be_bytes()
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

    fn const_be_bits(&mut self, u: u32) -> [BoolVariable; 32] {
        u.to_be_bytes()
            .iter()
            .flat_map(|b| self.constant::<ByteVariable>(*b).as_be_bits().to_vec())
            .collect::<Vec<BoolVariable>>()
            .try_into()
            .unwrap()
    }

    fn get_inital_hash(&mut self) -> [[BoolVariable; 32]; 8] {
        consts::INITIAL_HASH.map(|x| self.const_be_bits(x))
    }

    fn get_round_constants(&mut self) -> [[BoolVariable; 32]; 64] {
        consts::ROUND_CONSTANTS.map(|x| self.const_be_bits(x))
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
                w[i] = self.add_arr(inter2, s1);
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

        let mut digest = Vec::new();
        for word in sha256_hash.iter() {
            for bit in word {
                digest.push(*bit);
            }
        }

        digest
    }

    pub fn sha256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let padded = self.pad_message_sha256(input);
        let bool_digest = self.process_padded_message(&padded);
        assert_eq!(bool_digest.len(), 256);
        // Ok to use `from_variables_unsafe` as we know `process_padded_message` returns 256 bits
        Bytes32Variable::from_variables_unsafe(&bool_digest.iter().map(|b| b.0).collect_vec())
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
    use anyhow::Result;
    use hex::decode;
    use log::debug;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    use super::*;
    use crate::utils;

    fn to_bits(msg: Vec<u8>) -> Vec<bool> {
        let mut res = Vec::new();
        for bit in msg {
            let char = bit;
            for j in 0..8 {
                if (char & (1 << (7 - j))) != 0 {
                    res.push(true);
                } else {
                    res.push(false);
                }
            }
        }
        res
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_bench() -> Result<()> {
        utils::setup_logger();
        let mut msg = String::new();
        for _ in 0..8 {
            msg.push_str("abcdefghij");
        }
        let msg_bits = to_bits(msg.as_bytes().to_vec());
        let expected_digest = "d68d62c262c2ec08961c1104188cde86f51695878759666ad61490c8ec66745c";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

        let targets = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder, &targets);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(msg_hash[i].target);
            } else {
                builder.assert_zero(msg_hash[i].target);
            }
        }

        let data = builder.build::<C>();

        for i in 0..10 {
            let mut pw = PartialWitness::new();

            for i in 0..msg_bits.len() {
                pw.set_bool_target(targets[i], msg_bits[i]);
            }
            let now = std::time::Instant::now();
            let _proof = data.prove(pw).unwrap();
            debug!("{} step, time elapsed {}", i, now.elapsed().as_millis());
        }

        Ok(())
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_empty() -> Result<()> {
        let msg = b"";
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

        let targets = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder, &targets);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(msg_hash[i].target);
            } else {
                builder.assert_zero(msg_hash[i].target);
            }
        }

        let mut pw = PartialWitness::new();

        for i in 0..msg_bits.len() {
            pw.set_bool_target(targets[i], msg_bits[i]);
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_small_msg() -> Result<()> {
        let msg = b"plonky2";
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "8943a85083f16e93dc92d6af455841daacdae5081aa3125b614a626df15461eb";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
        let targets = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder, &targets);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(msg_hash[i].target);
            } else {
                builder.assert_zero(msg_hash[i].target);
            }
        }

        let mut pw = PartialWitness::new();

        for i in 0..msg_bits.len() {
            pw.set_bool_target(targets[i], msg_bits[i]);
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_large_msg() -> Result<()> {
        let msg = decode(
            "00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d",
        )
        .unwrap();
        let msg_bits = to_bits(msg.to_vec());
        // dbg!(&msg_bits);
        let expected_digest = "84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e";
        dbg!(decode(expected_digest).unwrap());
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());

        let targets = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder, &targets);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(msg_hash[i].target);
            } else {
                builder.assert_zero(msg_hash[i].target);
            }
        }

        let mut pw = PartialWitness::new();

        for i in 0..msg_bits.len() {
            pw.set_bool_target(targets[i], msg_bits[i]);
        }

        dbg!(builder.num_gates());
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    #[should_panic]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_failure() {
        let msg = decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273").unwrap();
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "9fcee6fbeadc123c38d5a97dbe58f8257b4906820d627425af668b94b795e74e";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

        let targets = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder, &targets);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(msg_hash[i].target);
            } else {
                builder.assert_zero(msg_hash[i].target);
            }
        }
        let mut pw = PartialWitness::new();

        for i in 0..msg_bits.len() {
            pw.set_bool_target(targets[i], msg_bits[i]);
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof).expect("sha256 error");
    }
}
