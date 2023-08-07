use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder;

use crate::hash::bit_operations::util::{_right_rotate, uint64_to_bits};
use crate::hash::bit_operations::{add_arr, not_arr, xor2_arr_slow, xor3_arr_slow};

const SIGMA_LEN: usize = 10;
const SIGMA: [[usize; 16]; SIGMA_LEN] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

const _WORDBITS: usize = 64;

fn get_iv<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
) -> [[BoolTarget; 64]; 8] {
    let iv: [u64; 8] = [
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
        res[i] = Some(uint64_to_bits(iv[i], builder));
    }
    res.map(|x| x.unwrap())
}

const ROT1: usize = 32;
const ROT2: usize = 24;
const ROT3: usize = 16;
const ROT4: usize = 63;

const CHUNK_128_BYTES: usize = 128;

pub struct Blake2bTarget {
    pub message: Vec<BoolTarget>,
    pub message_len: Target,
    pub digest: Vec<BoolTarget>,
}

fn select_chunk<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    b: BoolTarget,
    x: [BoolTarget; 64],
    y: [BoolTarget; 64],
) -> [BoolTarget; 64] {
    let mut res = [None; 64];
    for i in 0..64 {
        res[i] = Some(builder.select(b, x[i].target, y[i].target));
    }
    res.map(|x| BoolTarget::new_unsafe(x.unwrap()))
}

fn compress<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    h: &mut [[BoolTarget; 64]; 8],
    chunk: &[BoolTarget],
    t: Target,
    is_last_block: BoolTarget,
    do_noop: BoolTarget,
) {
    let mut ov = Vec::new();
    let iv = get_iv(builder);

    // Setup local work vector V
    ov.push(h[0]);
    ov.push(h[1]);
    ov.push(h[2]);
    ov.push(h[3]);
    ov.push(h[4]);
    ov.push(h[5]);
    ov.push(h[6]);
    ov.push(h[7]);

    ov.push(iv[0]);
    ov.push(iv[1]);
    ov.push(iv[2]);
    ov.push(iv[3]);
    // Mix the 128-bit counter t into V12:V13
    // Get t's big endian bit representation
    let mut t_bits = builder.split_le(t, 64);
    t_bits.reverse();
    ov.push(xor2_arr_slow(iv[4], t_bits.try_into().unwrap(), builder)); // assumes t is not more than u64
    ov.push(iv[5]); // assumes t is not more than u64 and Hi(t) == 0

    // If this is the last block then invert all the bits in V14
    let not_iv_6 = not_arr(iv[6], builder);
    ov.push(select_chunk(builder, is_last_block, not_iv_6, iv[6]));
    ov.push(iv[7]);

    let mut v = ov.try_into().unwrap();

    // Treat each 128-byte message chunk as sixteen 8-byte (64-bit) little-endian words m
    let mut m = reshape(chunk.to_vec());

    // Flip byte endian-ness
    for w in 0..16 {
        // words
        for b in 0..8 {
            for byte in 0..4 {
                (m[w][8 * byte + b], m[w][8 * (7 - byte) + b]) =
                    (m[w][8 * (7 - byte) + b], m[w][8 * byte + b]);
            }
        }
    }

    // Twelve rounds of cryptographic message mixing
    for r in 0..12 {
        let s = &SIGMA[r % SIGMA_LEN];

        mix(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]], builder);
        mix(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]], builder);
        mix(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]], builder);
        mix(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]], builder);

        mix(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]], builder);
        mix(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]], builder);
        mix(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]], builder);
        mix(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]], builder);
    }

    // Mix the upper and lower halves of V into ongoing state vector h
    let h0_xor = xor3_arr_slow(h[0], v[0], v[8], builder);
    let h1_xor = xor3_arr_slow(h[1], v[1], v[1 + 8], builder);
    let h2_xor = xor3_arr_slow(h[2], v[2], v[2 + 8], builder);
    let h3_xor = xor3_arr_slow(h[3], v[3], v[3 + 8], builder);
    let h4_xor = xor3_arr_slow(h[4], v[4], v[4 + 8], builder);
    let h5_xor = xor3_arr_slow(h[5], v[5], v[5 + 8], builder);
    let h6_xor = xor3_arr_slow(h[6], v[6], v[6 + 8], builder);
    let h7_xor = xor3_arr_slow(h[7], v[7], v[7 + 8], builder);

    h[0] = select_chunk(builder, do_noop, h[0], h0_xor);
    h[1] = select_chunk(builder, do_noop, h[1], h1_xor);
    h[2] = select_chunk(builder, do_noop, h[2], h2_xor);
    h[3] = select_chunk(builder, do_noop, h[3], h3_xor);
    h[4] = select_chunk(builder, do_noop, h[4], h4_xor);
    h[5] = select_chunk(builder, do_noop, h[5], h5_xor);
    h[6] = select_chunk(builder, do_noop, h[6], h6_xor);
    h[7] = select_chunk(builder, do_noop, h[7], h7_xor);
}

#[allow(clippy::too_many_arguments)]
fn mix<F: RichField + Extendable<D>, const D: usize>(
    v: &mut [[BoolTarget; 64]; 16],
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    x: [BoolTarget; 64],
    y: [BoolTarget; 64],
    builder: &mut CircuitBuilder<F, D>,
) {
    v[a] = add_arr(add_arr(v[a], v[b], builder), x, builder);
    v[d] = _right_rotate(xor2_arr_slow(v[d], v[a], builder), ROT1);
    v[c] = add_arr(v[c], v[d], builder);
    v[b] = _right_rotate(xor2_arr_slow(v[b], v[c], builder), ROT2);
    v[a] = add_arr(add_arr(v[a], v[b], builder), y, builder);
    v[d] = _right_rotate(xor2_arr_slow(v[d], v[a], builder), ROT3);
    v[c] = add_arr(v[c], v[d], builder);
    v[b] = _right_rotate(xor2_arr_slow(v[b], v[c], builder), ROT4);
}

fn reshape(u: Vec<BoolTarget>) -> Vec<[BoolTarget; 64]> {
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

// reference: https://github.com/ethereum/blake2b-py/blob/master/src/blake2b.rs
// reference: https://github.com/bryant/pyblake2/blob/master/pyblake2/__init__.py
// A zk-circuit implementation of the Blake2b hash function.
// Computes blake2b
pub fn blake2b<
    F: RichField + Extendable<D>,
    const D: usize,
    const MAX_MESSAGE_LENGTH: usize,
    const DIGEST_SIZE: usize,
>(
    builder: &mut CircuitBuilder<F, D>,
) -> Blake2bTarget {
    assert!(MAX_MESSAGE_LENGTH > 0 && MAX_MESSAGE_LENGTH % CHUNK_128_BYTES == 0);

    let length = builder.add_virtual_target();

    let mut msg_input = Vec::new();
    for _i in 0..MAX_MESSAGE_LENGTH * 8 {
        msg_input.push(builder.add_virtual_bool_target_safe());
    }

    // Initialize State vector h with iv
    let mut blake2b_hash = get_iv(builder);

    // Mix key size and hash length (64 0x40); p[0] = 0x0101kknn
    let p = 0x1010000 + (DIGEST_SIZE as u64);
    blake2b_hash[0] = xor2_arr_slow(blake2b_hash[0], uint64_to_bits(p, builder), builder);

    let mut do_noop = builder.constant_bool(false);

    // Compress all full message chunks (128-bytes)
    let chunk_bytes_target = builder.constant(F::from_canonical_u32(CHUNK_128_BYTES as u32));
    let length_bits = builder.split_le(length, 64);
    // The bits [7 .. 64] will be the total number of chunks
    let last_block_num = builder.le_sum(length_bits[7..64].to_vec().iter());
    let max_block_num = MAX_MESSAGE_LENGTH / CHUNK_128_BYTES;

    // Each time we Compress we record how many bytes have been compressed
    let mut t_target = builder.zero();
    for i in 0..max_block_num {
        let i_target = builder.constant(F::from_canonical_usize(i));
        let is_last_block = builder.is_equal(i_target, last_block_num);

        let chunk: Vec<BoolTarget> =
            msg_input[CHUNK_128_BYTES * i * 8..CHUNK_128_BYTES * (i + 1) * 8].to_vec();

        t_target = builder.add(t_target, chunk_bytes_target);
        let t_input = builder.select(is_last_block, length, t_target);

        compress(
            builder,
            &mut blake2b_hash,
            &chunk,
            t_input,
            is_last_block,
            do_noop,
        );

        if i != (max_block_num - 1) {
            do_noop = BoolTarget::new_unsafe(builder.select(
                do_noop,
                do_noop.target,
                is_last_block.target,
            ));
        }
    }

    let mut digest = Vec::new();
    'outer: for word in blake2b_hash.iter() {
        // Flip byte endian-ness
        for byte in (0..8).rev() {
            for b in 0..8 {
                digest.push(word[8 * byte + b]);
            }
            if digest.len() == DIGEST_SIZE * 8 {
                break 'outer;
            }
        }
    }

    Blake2bTarget {
        message: msg_input,
        message_len: length,
        digest,
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hex::decode;
    use plonky2::field::types::{Field, PrimeField64};
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    use crate::hash::blake2::blake2b::{blake2b, CHUNK_128_BYTES};

    fn to_bits(msg: Vec<u8>) -> Vec<bool> {
        let mut res = Vec::new();
        for i in 0..msg.len() {
            let char = msg[i];
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

    fn run_test<const MAX_MESSAGE_LENGTH: usize, const DIGEST_SIZE: usize>(
        msg: &[u8],
        expected_digest: &str,
    ) -> Result<()> {
        let msg_bits = to_bits(msg.to_vec());
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut builder: CircuitBuilder<plonky2::field::goldilocks_field::GoldilocksField, 2> =
            CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

        let blake2b_targets = blake2b::<F, D, MAX_MESSAGE_LENGTH, DIGEST_SIZE>(&mut builder);
        builder.register_public_inputs(
            &blake2b_targets
                .digest
                .iter()
                .map(|x| x.target)
                .collect::<Vec<_>>(),
        );

        let mut pw = PartialWitness::new();
        for i in 0..msg_bits.len() {
            pw.set_bool_target(blake2b_targets.message[i], msg_bits[i]);
        }
        for i in msg_bits.len()..MAX_MESSAGE_LENGTH * 8 {
            pw.set_bool_target(blake2b_targets.message[i], false);
        }
        pw.set_target(
            blake2b_targets.message_len,
            F::from_canonical_u64(msg.len() as u64),
        );

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        for (expected_bit, proof_bit) in digest_bits.iter().zip(proof.public_inputs.iter()) {
            assert_eq!(*expected_bit as u64, proof_bit.to_canonical_u64());
        }

        data.verify(proof)
    }

    #[test]
    fn test_blake2b() {
        println!("Running blake2b test #1");
        run_test::<CHUNK_128_BYTES, 32>(
            b"",
            "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8",
        )
        .expect("Failed test #1");

        println!("Running blake2b test #2");
        const MAX_MESSAGE_LENGTH: usize = CHUNK_128_BYTES * 2;
        run_test::<MAX_MESSAGE_LENGTH, 64>(b"abc", "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923").expect("Failed test #2");

        println!("Running blake2b test #3");
        run_test::<MAX_MESSAGE_LENGTH, 64>(&b"12345678".repeat(16 + 1), "e21ff4f8ce8ac25483d4532644d1dfc50469c90496a1343812d764c6322e13c9eabfb327f3c50647f30d237e2cc6326e5a1a3766247cbcc3e3f39aa039492554").expect("Failed test #3");
    }
}
