use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::iop::target::Target;

use crate::hash::bit_operations::util::{_right_rotate, _shr, uint64_to_bits};
use crate::hash::bit_operations::{add_arr, and_arr, not_arr, xor2_arr, xor3_arr, zip_add};

pub struct Sha512VariableTarget {
    pub message: Vec<BoolTarget>,
    pub hash_msg_length_bits: Target,
    pub digest: Vec<BoolTarget>,
}

pub const SELECT_CHUNK_SIZE_64: usize = 64;
pub const LENGTH_BITS_128: usize = 128;
pub const CHUNK_1024_BITS: usize = 1024;

pub fn get_initial_hash<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> [[BoolTarget; 64]; 8] {
    let initial_hash = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];
    let mut res = [None; 8];
    for i in 0..8 {
        res[i] = Some(uint64_to_bits(initial_hash[i], builder));
    }
    res.map(|x| x.unwrap())
}

pub fn get_round_constants<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> [[BoolTarget; 64]; 80] {
    let round_constants: [u64; 80] = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];
    let mut res = [None; 80];
    for i in 0..80 {
        res[i] = Some(uint64_to_bits(round_constants[i], builder));
    }
    res.map(|x| x.unwrap())
}

pub fn reshape(u: Vec<BoolTarget>) -> Vec<[BoolTarget; 64]> {
    let l = u.len() / 64;
    let mut res = Vec::new();
    for i in 0..l {
        let mut arr = [None; 64];
        for j in 0..64 {
            arr[j] = Some(u[i * 64 + j]);
        }
        res.push(arr.map(|x| x.unwrap()));
    }
    res
}

fn select_chunk<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    b: BoolTarget,
    x: [BoolTarget; SELECT_CHUNK_SIZE_64],
    y: [BoolTarget; SELECT_CHUNK_SIZE_64]
) -> [BoolTarget; SELECT_CHUNK_SIZE_64] {
    let mut res = [None; SELECT_CHUNK_SIZE_64];
    for i in 0..SELECT_CHUNK_SIZE_64 {
        res[i] = Some(builder.select(b, x[i].target, y[i].target));
    }
    res.map(|x| BoolTarget::new_unsafe(x.unwrap()))
}

const fn calculate_num_chunks(length: usize) -> usize {
    // Add 128 bits for the length and 1 bit for the padding bit
    let msg_with_min_padding_len = length + LENGTH_BITS_128 + 1;

    let additional_padding_len = CHUNK_1024_BITS - (msg_with_min_padding_len % CHUNK_1024_BITS);
    
    let msg_length_with_all_padding = msg_with_min_padding_len + additional_padding_len;

    msg_length_with_all_padding / CHUNK_1024_BITS
}


// Should be a multiple of CHUNK_1024_BITS
fn pad_sha512_variable<F: RichField + Extendable<D>, const D: usize, const MAX_NUM_CHUNKS: usize>(
    builder: &mut CircuitBuilder<F, D>,
    // Message should have length MAX_NUM_CHUNKS
    message: &[BoolTarget],
    // Pass in the last chunk number in the message as a target
    last_chunk: Target,
    // This should be less than (MAX_NUM_CHUNKS * 1024) - 129
    // Length in bits of the target
    hash_msg_length_bits: Target
) -> Vec<BoolTarget> {
    let mut msg_input = Vec::new();

    let mut length_bits = builder.split_le(hash_msg_length_bits, 64);
    // Convert length to BE bits
    length_bits.reverse();

    let mut add_message_bit_selector = builder.constant_bool(true);
    for i in 0..MAX_NUM_CHUNKS {
        let chunk_offset = CHUNK_1024_BITS * i;
        let curr_chunk_t = builder.constant(F::from_canonical_usize(i));
        // Check if this is the chunk where length should be added
        let add_length_bit_selector = builder.is_equal(last_chunk, curr_chunk_t);
        // Always message || padding || nil
        for j in 0..CHUNK_1024_BITS-LENGTH_BITS_128 {
            let idx = chunk_offset + j;

            let idx_t = builder.constant(F::from_canonical_usize(idx));
            let idx_length_eq_t = builder.is_equal(idx_t, hash_msg_length_bits);

            // select_bit AND NOT(idx_length_eq_t)
            let not_idx_length_eq_t = builder.not(idx_length_eq_t);
            add_message_bit_selector = BoolTarget::new_unsafe(
                builder.select(
                        add_message_bit_selector,
                        not_idx_length_eq_t.target,
                        add_message_bit_selector.target)
            );

            // Set bit to push: (select_bit && message[i]) || idx_length_eq_t
            let bit_to_push = builder.and(add_message_bit_selector, message[idx]);
            let bit_to_push = builder.or(idx_length_eq_t, bit_to_push);
            msg_input.push(bit_to_push);

        }

        // Message || padding || length || nil
        for j in CHUNK_1024_BITS-LENGTH_BITS_128..CHUNK_1024_BITS {
            let idx = chunk_offset + j;

            // Only true if in the last valid chunk
            let length_bit = builder.and(length_bits[j % LENGTH_BITS_128], add_length_bit_selector);

            // TODO: add_length_bit_selector && (add_message_bit_selector || length_bit) should never be true concurrently -> add constraint for this?

            let idx_t = builder.constant(F::from_canonical_usize(idx));
            let idx_length_eq_t = builder.is_equal(idx_t, hash_msg_length_bits);

            // select_bit AND NOT(idx_length_eq_t)
            let not_idx_length_eq_t = builder.not(idx_length_eq_t);
            add_message_bit_selector = BoolTarget::new_unsafe(
                builder.select(
                        add_message_bit_selector,
                        not_idx_length_eq_t.target,
                        add_message_bit_selector.target)
            );
            
            // Set bit to push: (select_bit && message[i]) || idx_length_eq_t
            let bit_to_push = builder.and(add_message_bit_selector, message[idx]);
            let bit_to_push = builder.or(idx_length_eq_t, bit_to_push);

            let bit_to_push = builder.or(length_bit, bit_to_push);

            // Either length bit || (message[i] || idx_length_eq_t)
            msg_input.push(bit_to_push);
        }
    }
    msg_input
}

fn process_sha512_variable<F: RichField + Extendable<D>, const D: usize, const MAX_NUM_CHUNKS: usize>(
    builder: &mut CircuitBuilder<F, D>,
    msg_input: &[BoolTarget],
    last_chunk: Target,
) -> Vec<BoolTarget> {
    let mut sha512_hash = get_initial_hash(builder);
    let round_constants = get_round_constants(builder);

    let mut noop_select = builder.constant_bool(false);
    // Process the input with 1024 bit chunks
    for chunk_start in (0..MAX_NUM_CHUNKS*CHUNK_1024_BITS).step_by(1024) {
        let chunk = msg_input[chunk_start..chunk_start + CHUNK_1024_BITS].to_vec();

        let new_sha512_hash = process_sha512_chunk(builder, round_constants, sha512_hash, chunk);
        for i in 0..8 {
            sha512_hash[i] = select_chunk(builder, noop_select, sha512_hash[i], new_sha512_hash[i]);
        }

        // Check if this is the last chunk
        let curr_chunk_t = builder.constant(F::from_canonical_usize(chunk_start / CHUNK_1024_BITS));
        let is_last_block = builder.is_equal(last_chunk, curr_chunk_t);

        noop_select = BoolTarget::new_unsafe(
            builder.select(
                    noop_select,
                    noop_select.target,
                    is_last_block.target)
        );
    }

    let mut digest = Vec::new();
    for word in sha512_hash.iter() {
        for d in word {
            digest.push(*d);
        }
    }
    digest

}

// Number of chunks in hash_msg_input
pub fn sha512_variable<F: RichField + Extendable<D>, const D: usize, const MAX_NUM_CHUNKS: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> Sha512VariableTarget {

    let hash_msg_length_bits = builder.add_virtual_target();

    let mut msg_input = Vec::new();
    for _i in 0..MAX_NUM_CHUNKS * CHUNK_1024_BITS {
        msg_input.push(builder.add_virtual_bool_target_safe());
    }

    let length_bits = builder.split_le(hash_msg_length_bits, 64);

    let last_block_num = builder.le_sum(length_bits[10..64].to_vec().iter());

    let hash_msg_input = pad_sha512_variable::<F, D, MAX_NUM_CHUNKS>(builder, &msg_input, last_block_num, hash_msg_length_bits);

    // Padding may require an additional chunk (should be computed outside of the circuit)
    let digest = process_sha512_variable::<F, D, MAX_NUM_CHUNKS>(builder, &hash_msg_input, last_block_num);

    return Sha512VariableTarget { message: msg_input, hash_msg_length_bits, digest }
}

fn process_sha512<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    msg_input: &[BoolTarget],
) -> Vec<BoolTarget> {
    let mut sha512_hash = get_initial_hash(builder);
    let round_constants = get_round_constants(builder);

    // Process the input with 1024 bit chunks
    for chunk_start in (0..msg_input.len()).step_by(CHUNK_1024_BITS) {
        let chunk = msg_input[chunk_start..chunk_start + CHUNK_1024_BITS].to_vec();

        sha512_hash = process_sha512_chunk(builder, round_constants, sha512_hash, chunk);
    }

    let mut digest = Vec::new();
    for word in sha512_hash.iter() {
        for d in word {
            digest.push(*d);
        }
    }
    digest
}

fn process_sha512_chunk<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>, 
    round_constants: [[BoolTarget; 64]; 80], 
    sha512_hash: [[BoolTarget; 64]; 8], 
    chunk: Vec<BoolTarget>
) -> [[BoolTarget; 64]; 8] {
    let mut u = Vec::new();

    for bit in chunk.iter().take(1024) {
        u.push(*bit);
    }
    for _ in 1024..80 * 64 {
        u.push(builder.constant_bool(false));
    }

    let mut w = reshape(u);
    for i in 16..80 {
        let s0 = xor3_arr(
            _right_rotate(w[i - 15], 1),
            _right_rotate(w[i - 15], 8),
            _shr(w[i - 15], 7, builder),
            builder,
        );
        let s1 = xor3_arr(
            _right_rotate(w[i - 2], 19),
            _right_rotate(w[i - 2], 61),
            _shr(w[i - 2], 6, builder),
            builder,
        );
        let inter1 = add_arr(w[i - 16], s0, builder);
        let inter2 = add_arr(inter1, w[i - 7], builder);
        w[i] = add_arr(inter2, s1, builder);
    }
    let mut a = sha512_hash[0];
    let mut b = sha512_hash[1];
    let mut c = sha512_hash[2];
    let mut d = sha512_hash[3];
    let mut e = sha512_hash[4];
    let mut f = sha512_hash[5];
    let mut g = sha512_hash[6];
    let mut h = sha512_hash[7];

    for i in 0..80 {
        let sum1 = xor3_arr(
            _right_rotate(e, 14),
            _right_rotate(e, 18),
            _right_rotate(e, 41),
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
            _right_rotate(a, 28),
            _right_rotate(a, 34),
            _right_rotate(a, 39),
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

    zip_add(sha512_hash, [a, b, c, d, e, f, g, h], builder)
}

pub fn sha512<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    message: &[BoolTarget],
) -> Vec<BoolTarget> {
    let mut msg_input = Vec::new();
    msg_input.extend_from_slice(message);

    // TODO: Range check size of msg_bit_len?
    let msg_bit_len: usize = message.len();

    // minimum_padding = 1 + 128 (min 1 bit for the pad, and 128 bit for the msg size)
    let msg_with_min_padding_len = msg_bit_len + LENGTH_BITS_128 + 1;

    let additional_padding_len = CHUNK_1024_BITS - (msg_with_min_padding_len % CHUNK_1024_BITS);

    msg_input.push(builder.constant_bool(true));
    for _i in 0..additional_padding_len {
        msg_input.push(builder.constant_bool(false));
    }

    for i in (0..128).rev() {
        let has_bit = (msg_bit_len & (1 << i)) != 0;
        msg_input.push(builder.constant_bool(has_bit));
    }

    process_sha512(builder, &msg_input)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use plonky2::iop::witness::WitnessWrite;
    use plonky2::field::types::Field;
    use subtle_encoding::hex::decode;

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
    fn test_sha512_empty() -> Result<()> {
        let msg = b"";
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
        let message = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let digest = sha512(&mut builder, &message);
        let pw = PartialWitness::new();

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(digest[i].target);
            } else {
                builder.assert_zero(digest[i].target);
            }
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    fn test_sha512_small_msg() -> Result<()> {
        let msg = b"plonky2";
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
        let message = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let digest = sha512(&mut builder, &message);
        let pw = PartialWitness::new();

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(digest[i].target);
            } else {
                builder.assert_zero(digest[i].target);
            }
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    fn test_sha512_large_msg() -> Result<()> {
        let msg = decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273").unwrap();
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let message = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let digest = sha512(&mut builder, &message);
        let pw = PartialWitness::new();

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(digest[i].target);
            } else {
                builder.assert_zero(digest[i].target);
            }
        }

        dbg!(builder.num_gates());
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    fn test_sha512_large_msg_variable() -> Result<()> {
        // This test tests both the variable length and the no-op skip for processing each chunk of the sha512
        // 77-byte message fits in one chunk, but we make MAX_NUM_CHUNKS 2 to test the no-op skip
        let msg = decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273").unwrap();
        let msg_bits = to_bits(msg.to_vec());

        let expected_digest = "4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());

        // Note: This should be computed from the maximum SHA512 size for the circuit
        const MAX_NUM_CHUNKS: usize = 2;

        let sha512_target = sha512_variable::<F, D, MAX_NUM_CHUNKS>(&mut builder);
        let mut pw = PartialWitness::new();

        // Pass in the bit length of the message to hash as a target
        pw.set_target(sha512_target.hash_msg_length_bits, F::from_canonical_usize(msg_bits.len()));

        // Add extra bool targets
        for i in 0..msg_bits.len() {
            pw.set_bool_target(sha512_target.message[i], msg_bits[i]);
        }

        // Add extra bool targets
        for i in msg_bits.len()..MAX_NUM_CHUNKS*CHUNK_1024_BITS {
            pw.set_bool_target(sha512_target.message[i], false);
        }

        for i in 0..sha512_target.digest.len() {
            if digest_bits[i] {
                builder.assert_one(sha512_target.digest[i].target);
            } else {
                builder.assert_zero(sha512_target.digest[i].target);
            }
        }

        dbg!(builder.num_gates());
        let data = builder.build::<C>();
        
        let circuit_digest = data.verifier_only.circuit_digest;
        println!("circuit_digest: {:?}", circuit_digest);

        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    #[test]
    #[should_panic]
    fn test_sha512_failure() {
        let msg = decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273").unwrap();
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "3388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
        let message = msg_bits
            .iter()
            .map(|b| builder.constant_bool(*b))
            .collect::<Vec<_>>();
        let digest = sha512(&mut builder, &message);
        let pw = PartialWitness::new();

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.assert_one(digest[i].target);
            } else {
                builder.assert_zero(digest[i].target);
            }
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof).expect("sha512 error");
    }
}
