use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{Target, BoolTarget};
use plonky2::plonk::circuit_builder::CircuitBuilder;

use crate::hash::bit_operations::util::{_right_rotate, _shr, uint64_to_bits};
use crate::hash::bit_operations::{add_arr, and_arr, not_arr, xor2_arr, xor3_arr, zip_add};

fn select_chunk<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    b: BoolTarget,
    x: [BoolTarget; 64],
    y: [BoolTarget; 64]
) -> [BoolTarget; 64] {
    let mut res = [None; 64];
    for i in 0..64 {
        res[i] = Some(builder.select(b, x[i].target, y[i].target));
    }
    res.map(|x| BoolTarget::new_unsafe(x.unwrap()))
}

const CHUNK_128_BYTES: usize = 128;

pub struct Sha512Target {
    pub message: Vec<BoolTarget>,
    pub message_len: Target,
    pub digest: Vec<BoolTarget>,
}

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

pub fn sha512<F: RichField + Extendable<D>, const D: usize, const MAX_MESSAGE_LENGTH: usize>(
    builder: &mut CircuitBuilder<F, D>,
    // In bytes
    message_len_bytes: usize,
) -> Sha512Target {
    assert!(MAX_MESSAGE_LENGTH > 0 && MAX_MESSAGE_LENGTH % CHUNK_128_BYTES == 0);

    let length = builder.add_virtual_target();
    // Compress all full message chunks (128-bytes)

    // let length_bytes_target = builder.le_sum(length_bits.to_vec().iter());
    // let message_len_bytes_target = builder.constant(F::from_canonical_usize(message_len_bytes));
    // builder.connect(length_bytes_target, message_len_bytes_target);

    // The bits [7 .. 64] will be the total number of chunks
    let last_block_num = builder.constant(F::from_canonical_usize((message_len_bytes * 8) + 129 / 1024));

    let mut msg_input = Vec::new();
    // msg_input.extend_from_slice(&message[..message_len]);

    let max_msg_bit_len: u128 = MAX_MESSAGE_LENGTH as u128 * 8;
    let msg_bit_len: u128 = message_len_bytes as u128 * 8;

    for _i in 0..msg_bit_len {
        msg_input.push(builder.add_virtual_bool_target_safe());
    }

    // minimum_padding = 1 + 128 (min 1 bit for the pad, and 128 bit for the msg size)
    let msg_with_min_padding_len = msg_bit_len + 129;

    let additional_padding_len = 1024 - (msg_with_min_padding_len % 1024);

    msg_input.push(builder.constant_bool(true));
    for _i in 0..additional_padding_len {
        msg_input.push(builder.constant_bool(false));
    }

    for i in (0..128).rev() {
        let has_bit = (msg_bit_len & (1 << i)) != 0;
        msg_input.push(builder.constant_bool(has_bit));
    }

    // No-op bits
    for _i in msg_with_min_padding_len + additional_padding_len..max_msg_bit_len {
        msg_input.push(builder.constant_bool(false));
    }

    let mut sha512_hash = get_initial_hash(builder);
    let round_constants = get_round_constants(builder);

    let mut do_noop = builder.constant_bool(false);

    // Process the input with 1024 bit chunks
    for chunk_start in (0..msg_input.len()).step_by(1024) {
        let i_target = builder.constant(F::from_canonical_usize(chunk_start / 1024));
        let is_last_block = builder.is_equal(i_target, last_block_num);

        let chunk = msg_input[chunk_start..chunk_start + 1024].to_vec();
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

        let new_sha512_hash = zip_add(sha512_hash, [a, b, c, d, e, f, g, h], builder);
        for i in 0..8 {
            sha512_hash[i] = select_chunk(builder, do_noop, sha512_hash[i], new_sha512_hash[i]);
        }

        do_noop = BoolTarget::new_unsafe(
            builder.select(
                    do_noop,
                    do_noop.target,
                    is_last_block.target)
        );
    }

    let mut digest = Vec::new();
    for word in sha512_hash.iter() {
        for d in word {
            digest.push(*d);
        }
    }

    Sha512Target { message: msg_input, message_len: length, digest }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::field::types::{Field, PrimeField64};
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
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

    fn run_test<const MAX_MESSAGE_LENGTH: usize>(
        msg: &[u8],
        expected_digest: &str,
    ) -> Result<()> {
        let msg_bits = to_bits(msg.to_vec());
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut builder: CircuitBuilder<plonky2::field::goldilocks_field::GoldilocksField, 2> = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

        let sha512_targets = sha512::<F, D, MAX_MESSAGE_LENGTH>(&mut builder, msg_bits.len() / 8);
        builder.register_public_inputs(&sha512_targets.digest.iter().map(|x| x.target).collect::<Vec<_>>());

        let mut pw = PartialWitness::new();
        for i in 0..msg_bits.len() {
            pw.set_bool_target(sha512_targets.message[i], msg_bits[i]);
        }
        pw.set_target(sha512_targets.message_len, F::from_canonical_u64(msg.len() as u64));

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        for (expected_bit, proof_bit) in digest_bits.iter().zip(proof.public_inputs.iter()) {
            assert_eq!(*expected_bit as u64, proof_bit.to_canonical_u64());
        }

        data.verify(proof)
    }

    #[test]
    fn test_sha512_empty() -> Result<()> {
        let msg = b"";
        let expected_digest = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";

        run_test::<CHUNK_128_BYTES>(msg, expected_digest)
    }

    #[test]
    fn test_sha512_small_msg() -> Result<()> {
        let msg = b"plonky2";
        let expected_digest = "7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87";

        run_test::<CHUNK_128_BYTES>(msg, expected_digest)
    }

    #[test]
    fn test_sha512_large_msg() -> Result<()> {
        let msg = decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273").unwrap();
        let expected_digest = "4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39";

        run_test::<CHUNK_128_BYTES>(&msg, expected_digest)
    }

    #[test]
    #[should_panic]
    fn test_sha512_failure() {
        let msg = decode("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273").unwrap();
        let expected_digest = "3388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39";

        run_test::<CHUNK_128_BYTES>(&msg, expected_digest).expect("should fail")
    }
}
