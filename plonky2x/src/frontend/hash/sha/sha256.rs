use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder;

use crate::frontend::hash::bit_operations::util::{_right_rotate, _shr, uint32_to_bits};
use crate::frontend::hash::bit_operations::{
    add_arr, and_arr, not_arr, xor2_arr, xor3_arr, zip_add,
};

pub struct Sha256Target {
    pub message: Vec<BoolTarget>,
    pub digest: Vec<BoolTarget>,
}

pub const CHUNK_64_BYTES: usize = 64;

pub const SINGLE_CHUNK_MAX_MESSAGE_BYTES: usize = CHUNK_64_BYTES - 9;

fn get_initial_hash<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> [[BoolTarget; 32]; 8] {
    let initial_hash = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut res = [None; 8];
    for i in 0..8 {
        res[i] = Some(uint32_to_bits(initial_hash[i], builder));
    }
    res.map(|x| x.unwrap())
}

fn get_round_constants<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> [[BoolTarget; 32]; 64] {
    let round_constants: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut res = [None; 64];
    for i in 0..64 {
        res[i] = Some(uint32_to_bits(round_constants[i], builder));
    }
    res.map(|x| x.unwrap())
}

fn reshape(u: Vec<BoolTarget>) -> Vec<[BoolTarget; 32]> {
    let l = u.len() / 32;
    let mut res = Vec::new();
    for i in 0..l {
        let mut arr = [None; 32];
        for j in 0..32 {
            arr[j] = Some(u[i * 32 + j]);
        }
        res.push(arr.map(|x| x.unwrap()));
    }
    res
}

// Compute the SHA256 hash of variable length message that fits into a single chunk
pub fn sha256_variable_length_single_chunk<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    message: &[BoolTarget],
    // Length in bits
    length: Target,
) -> Vec<BoolTarget> {
    let padded_message = pad_single_sha256_chunk::<F, D>(builder, message, length);

    process_sha256(builder, &padded_message)
}

// Pad a variable length, single SHA256 chunk from a message
pub fn pad_single_sha256_chunk<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    message: &[BoolTarget],
    // Length in bits (assumes less than SINGLE_CHUNK_MAX_MESSAGE_BYTES * 8)
    length: Target,
) -> [BoolTarget; CHUNK_64_BYTES * 8] {
    assert!(message.len() <= SINGLE_CHUNK_MAX_MESSAGE_BYTES * 8);
    // 1) Adds all message bits before idx = length
    // 2) Adds padding bit when idx = length
    // 3) Add padding 0s when idx > length before length BE bits

    let mut msg_input = Vec::new();

    let mut select_bit = builder.constant_bool(true);

    for i in 0..message.len() {
        let idx_t = builder.constant(F::from_canonical_usize(i));
        let idx_length_eq_t = builder.is_equal(idx_t, length);

        // select_bit AND NOT(idx_length_eq_t)
        let not_idx_length_eq_t = builder.not(idx_length_eq_t);
        select_bit = builder.and(select_bit, not_idx_length_eq_t);

        // Set bit to push: (select_bit && message[i]) || idx_length_eq_t
        let bit_to_push = builder.and(select_bit, message[i]);
        let bit_to_push = builder.or(idx_length_eq_t, bit_to_push);
        msg_input.push(bit_to_push);
    }

    // Adds the padding bit if it has not been included so far
    msg_input.push(select_bit);
    for _ in 0..7 {
        msg_input.push(builder.constant_bool(false));
    }

    // Additional padding if necessary
    for _ in 0..((SINGLE_CHUNK_MAX_MESSAGE_BYTES * 8) - message.len()) {
        msg_input.push(builder.constant_bool(false));
    }

    let mut length_bits = builder.split_le(length, 64);

    // Convert length to BE bits
    length_bits.reverse();
    for i in 0..CHUNK_64_BYTES {
        msg_input.push(length_bits[i]);
    }

    let mut padded_msg = [builder._false(); CHUNK_64_BYTES * 8];

    padded_msg[..(CHUNK_64_BYTES * 8)].copy_from_slice(&msg_input[..(CHUNK_64_BYTES * 8)]);

    padded_msg
}

// Process SHA256 on padded chunks
// reference: https://github.com/thomdixon/pysha2/blob/master/sha2/sha256.py
fn process_sha256<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    msg_input: &[BoolTarget],
) -> Vec<BoolTarget> {
    let mut sha256_hash = get_initial_hash(builder);
    let round_constants = get_round_constants(builder);

    // Process the input with 512 bit chunks aka 64 byte chunks
    for chunk_start in (0..msg_input.len()).step_by(512) {
        let chunk = msg_input[chunk_start..chunk_start + 512].to_vec();
        let mut u = Vec::new();

        for bit in chunk.iter().take(512) {
            // 0 .. 16 chunk size * 32 bits7
            u.push(*bit);
        }
        for _ in 512..64 * 32 {
            // 16 * 8 ... 64 * 8 because of L
            u.push(builder.constant_bool(false));
        }

        let mut w = reshape(u);
        for i in 16..64 {
            let s0 = xor3_arr(
                _right_rotate(w[i - 15], 7),
                _right_rotate(w[i - 15], 18),
                _shr(w[i - 15], 3, builder),
                builder,
            );
            let s1 = xor3_arr(
                _right_rotate(w[i - 2], 17),
                _right_rotate(w[i - 2], 19),
                _shr(w[i - 2], 10, builder),
                builder,
            );
            let inter1 = add_arr(w[i - 16], s0, builder);
            let inter2 = add_arr(inter1, w[i - 7], builder);
            w[i] = add_arr(inter2, s1, builder);
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
                _right_rotate(e, 6),
                _right_rotate(e, 11),
                _right_rotate(e, 25),
                builder,
            );
            let ch = xor2_arr(
                and_arr(e, f, builder),
                and_arr(not_arr(e, builder), g, builder),
                builder,
            );
            let temp1 = add_arr(h, sum1, builder);
            let temp2 = add_arr(temp1, ch, builder);
            let temp3 = add_arr(temp2, round_constants[i], builder);
            let temp4 = add_arr(temp3, w[i], builder);
            let final_temp1 = temp4;

            let sum0 = xor3_arr(
                _right_rotate(a, 2),
                _right_rotate(a, 13),
                _right_rotate(a, 22),
                builder,
            );

            let maj = xor3_arr(
                and_arr(a, b, builder),
                and_arr(a, c, builder),
                and_arr(b, c, builder),
                builder,
            );
            let final_temp2 = add_arr(sum0, maj, builder);

            h = g;
            g = f;
            f = e;
            e = add_arr(d, final_temp1, builder);
            d = c;
            c = b;
            b = a;
            a = add_arr(final_temp1, final_temp2, builder);
        }

        sha256_hash = zip_add(sha256_hash, [a, b, c, d, e, f, g, h], builder);
    }

    let mut digest = Vec::new();
    for word in sha256_hash.iter() {
        for bit in word {
            digest.push(*bit);
        }
    }

    // Constrain the output of the hash to the correct length.
    let digest_len_const = builder.constant(F::from_canonical_usize(digest.len()));
    let hash_len_const = builder.constant(F::from_canonical_usize(32 * 8_usize));
    builder.connect(digest_len_const, hash_len_const);

    digest
}

// Generate the 32-byte SHA-256 hash of the message.
// reference: https://github.com/thomdixon/pysha2/blob/master/sha2/sha256.py
pub fn sha256<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    message: &[BoolTarget],
) -> Vec<BoolTarget> {
    let mut msg_input = Vec::new();

    msg_input.extend_from_slice(message);

    let mdi = (message.len() / 8) % 64;
    let length = (message.len() / 8) << 3; // length in bytes
    let padlen = if mdi < 56 { 55 - mdi } else { 119 - mdi };

    msg_input.push(builder.constant_bool(true));
    for _ in 0..7 {
        msg_input.push(builder.constant_bool(false));
    }

    for _ in 0..padlen * 8 {
        msg_input.push(builder.constant_bool(false));
    }

    for i in (0..64).rev() {
        // big endian binary representation of length
        msg_input.push(builder.constant_bool((length >> i) & 1 == 1));
    }

    process_sha256(builder, &msg_input)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hex::decode;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    use super::*;

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
            println!("{} step, time elapsed {}", i, now.elapsed().as_millis());
        }

        Ok(())
    }

    #[test]
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
    fn test_sha256_single_chunk_variable() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());

        let msg = decode(
            "00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d",
        )
        .unwrap();
        let mut msg_bits = to_bits(msg.to_vec());

        // Length of the message in bits (should be less than SINGLE_CHUNK_MAX_MESSAGE_BYTES * 8)
        let length = builder.constant(F::from_canonical_usize(msg_bits.len()));

        msg_bits.extend(vec![
            false;
            SINGLE_CHUNK_MAX_MESSAGE_BYTES * 8 - msg_bits.len()
        ]);

        // dbg!(&msg_bits);
        let expected_digest = "84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e";
        dbg!(decode(expected_digest).unwrap());
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        let targets = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();

        let msg_hash = sha256_variable_length_single_chunk(&mut builder, &targets, length);

        let mut pw = PartialWitness::new();

        for i in 0..msg_hash.len() {
            pw.set_bool_target(msg_hash[i], digest_bits[i]);
        }

        dbg!(builder.num_gates());
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    #[should_panic]
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
