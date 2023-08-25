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
    let one = builder.one();
    let length_minus_one = builder.sub(length, one);
    let length_bits = builder.split_le(length_minus_one, 64);
    // The bits [7 .. 64] will be the total number of chunks
    let mut last_block_idx = builder.le_sum(length_bits[7..64].to_vec().iter());

    let zero = builder.zero();
    let length_is_zero = builder.is_equal(length, zero);
    // Zero length mesage is a special case
    last_block_idx = builder.select(length_is_zero, zero, last_block_idx);

    let max_block_num = MAX_MESSAGE_LENGTH / CHUNK_128_BYTES;

    // Each time we Compress we record how many bytes have been compressed
    let mut t_target = builder.zero();
    for i in 0..max_block_num {
        let i_target = builder.constant(F::from_canonical_usize(i));
        let is_last_block = builder.is_equal(i_target, last_block_idx);

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

        do_noop =
            BoolTarget::new_unsafe(builder.select(do_noop, do_noop.target, is_last_block.target));
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
        println!("setting length to {:?}", msg.len());
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

    // #[test]
    // fn test_blake2b() {
    //     println!("Running blake2b test #1");
    //     run_test::<CHUNK_128_BYTES, 32>(
    //         b"",
    //         "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8",
    //     )
    //     .expect("Failed test #1");

    //     println!("Running blake2b test #2");
    //     const MAX_MESSAGE_LENGTH: usize = CHUNK_128_BYTES * 2;
    //     run_test::<MAX_MESSAGE_LENGTH, 64>(b"abc", "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923").expect("Failed test #2");

    //     println!("Running blake2b test #3");
    //     run_test::<MAX_MESSAGE_LENGTH, 64>(&b"12345678".repeat(16 + 1), "e21ff4f8ce8ac25483d4532644d1dfc50469c90496a1343812d764c6322e13c9eabfb327f3c50647f30d237e2cc6326e5a1a3766247cbcc3e3f39aa039492554").expect("Failed test #3");

    //     println!("Running blake2b test #4");
    //     let msg = hex::decode("092005a6f7a58a98df5f9b8d186b9877f12b603a").unwrap();
    //     run_test::<MAX_MESSAGE_LENGTH, 32>(
    //         msg.as_slice(),
    //         "51ce01415fbb9361d96df765be0d130361433ced03909dbe1a874e6791d80e5f",
    //     )
    //     .expect("Failed test #4");

    //     println!("Running blake2b test #5");
    //     let msg = hex::decode("092005a6f7a58a98df5f9b8d186b9877f12b603aa06c7debf0f610d5a49f9ed7262b5e095b309af2b0eae1c554e03b6cc4a5a0df207b662b329623f27fdce8d088554d82b1e63bedeb3fe9bd7754c7deccdfe277bcbfad4bbaff6302d3488bd2a8565f4f6e753fc7942fa29051e258da2e06d13b352220b9eadb31d8ead7f88b").unwrap();
    //     run_test::<MAX_MESSAGE_LENGTH, 32>(
    //         msg.as_slice(),
    //         "dad415aa819ebb585ce8ee1c1fa883804f405f6d8a6a0992628fb3bdaab5b42e",
    //     )
    //     .expect("Failed test #5");

    //     println!("Running blake2b test #6");
    //     const LONG_MESSAGE_LENGTH: usize = 2560;
    //     let msg = hex::decode("092005a6f7a58a98df5f9b8d186b9877f12b603aa06c7debf0f610d5a49f9ed7262b5e095b309af2b0eae1c554e03b6cc4a5a0df207b662b329623f27fdce8d088554d82b1e63bedeb3fe9bd7754c7deccdfe277bcbfad4bbaff6302d3488bd2a8565f4f6e753fc7942fa29051e258da2e06d13b352220b9eadb31d8ead7f88b244f13c0835db4a3909cee6106b276684aba0f8d8b1b0ba02dff4d659b081adfeab6f3a26d7fd65eff7c72a539dbeee68a9497476b69082958eae7d6a7f0f1d5a1b99a0a349691e80429667831f9b818431514bb2763e26e94a65428d22f3827d491c474c7a1885fe1d2d557e27bbcd81bffa9f3a507649e623b47681d6c9893301d8f635ec49e983cc537c4b81399bb24027ac4be709ce1a4eeb448e98a9aecfe249696419a67cb9e0f29d0297d840048bddf6612a383f37d7b96348a1bc5f1f9ac6eed6eb911dc43e120c8480e0258a6b33e0b91734cc64f144827053b17ae91c62e6866d8b68c1b0e53df0d0f0f4f187278db30c7b95d2741f4d0c8c59507984482b48d356ce8e299268b100c61a9ba5f96a757cf98150683a3e8aa85484a4590b293b6ec62c77f022542a73651a42b50f05a8d10bbb546746ca82221ca3b18105a05e4a7ea9c9d5096a37c8b3ce1a9c62ebd7badd7ee6f1c6e5961a08d066d5e025e08e3ec72531c476098287b13295fa606fab8275418e0c4c54f236c9e73fbfdaa00a5205310cb0d1bd54175647482fae300cc66b36e7846e82288e9f0290d9479d0c1998373900dfb72900d1c9f55c018dd7eeed4ce0e988bb3da03a22910ddec7c51b2eab4d96831a8b9e84a42cebdadae62bdea26ca7b0c640e8a21f86c72277ed20efe15bab1abcf34656e7d2336e42133fa99331e874b5458b28fabe6cb62c4606ee7046d07bc9e5eec2246068396590b59194c10bbe82f7c8b5ddea0d85a4cf74a91c85d7f90873bfbdc40c8c939377bec9a26d66b895a1bbeaa94028d6eafa1c0d6218077d174cc59cea6f2ea17ef1c002160e549f43b03112b0a978fd659c69448273e35554e21bac35458fe2b199f8b8fb81a6488ee99c734e2eefb4dd06c686ca29cdb2173a53ec8322a6cb9128e3b7cdf4bf5a5c2e8906b840bd86fa97ef694a34fd47740c2d44ff7378d773ee090903796a719697e67d8df4bc26d8aeb83ed380c04fe8aa4f23678989ebffd29c647eb96d4999b4a6736dd66c7a479fe0352fda60876f173519b4e567f0a0f0798d25e198603c1c5569b95fefa2edb64720ba97bd4d5f82614236b3a1f5deb344df02d095fccfe1db9b000f38ebe212f804ea0fbbeb645b8375e21d27f5381de0e0c0156f2fa3a0a0a055b8afe90b542f6e0fffb744f1dba74e34bb4d3ea6c84e49796f5e549781a2f5c2dc01d7b8e814661b5e2d2a51a258b2f7032a83082e6e36a5e51ef9af960b0587e1190f4beb3657bcf5c044a5df2a86daf2116518f65c36992ede73dd6777c826e59b42880a9992cd2b1b08b7da26430a1857d8d0888e323ec63339dbbaccf47964906e479f2172f4eaea2446e44964680213193ac981c3ca77015df8b02f4c8772d6e13a1519a43afcc9e79b4556c972e90b380f289d993506253c9de1edd1c88ae88472391e18ec1b416f69ccf13399c4a0f6cc6abe662a88e033fb3f745957584413f927bee5a607d36d9eedc451ad5dab66af39d53c548649260e06760142f4515da05794e97d8a4b15679913ca844fa4e25cbe936aa92afce02ceac80718c9b6893834f9af8cf8410ed26f6e9c8029891b9b08e36d2706226d42058042369cfb82e998e36468e79b3eaeec213d9ec9d76e25edc1b65d41c85af32cca3656482e09117de608a6dd0cad6583bc044200c79308448e32a2bc3a97fddb0bc4802f80695f0a4a2308246c88134b2de759e347d527189742dd42e98724bd5a9bce6a3b907163bcd96cbd90fea75f15a20b03f7517c353e96ca4d7122a7063a6df53bdf319c8bf5992e7ad6215aa2dda2db44e6cc0caad127544a0f83362bca022d11692190eb3874068225007cb4253f23d535a0b0169b0b54014d083e9e39990597f675a8d47cb4165e9eede08ef9f7cb5906cbc9b44da080a21f9f9e905f730c39ad005c6ce056b0fa7baff65f4d77ed645aef3488080396c568f1c7989c60ac7733b942502973f2fa2cdff5d506e77224ff428b0f775fbf07942975b1788368207bf4bf7e95006f8da5463b44c77c8e334456e2c4c4d4cd5ffb74e8e45d07195a90ca29491de8982145611188f1a18bba22e29371d0ab494cd8bd0f02ab5b56db5e3b16a2cc4fc1de701c21a073981eb1a50caeb2e0294a21b786e92f084750bee9adb232d9c7248feefdfb3e3cc33364e3fc15a4a00e60e171cf930ef5ad0d9214fdd325d892b23d375f366478df7f2a426f9f29b66c2c9fa517954c2f2c34484e4346eb0b894f148234dd9ec736a2d37c428ae191b8359ed9b03b0f6017d978a06ff17956afd5cbb8a1e85267dccad9ea0aef5cb75f1c22af1baf2901358e4dc8f4c7d0d17009c8e2a4d462b9d6dec5abdcf9b9e74c5af88a518c6df8dce73071024c4dfbc81863e7201db88fc1a6fa6dcc4f6f2f0a8a33ef8ff3ef944733ed9fc282b5a8c2203d79501f4d1ba9b673196c6684b17bba0e3380637ddf578fc8f03e8f87dc94bcb10b18b8ad3d2394533c41b51c0fc6f0c89d7498bc07cf2ad19893bd78c611087f21c0f33ca1edbb053aa5fdb3aa00ff6a4ba7d6ba286b8064a31abd0d2431f3109a44c8b00e724f56731a99852408c83985157fa6276dafa74875360c9fab71916824e96bccf6417c4ea23f01c298f31bc8775689dd93b496f13c8d934c49cf357ff744fe0adb9ba8bb4c0a6697194ab6c807be92f590ac4a1b230780455f5f18ec4b84386cde822e62ceefa8d32fbca0b9f398e434c86e7a3dcc0cca510bddefbbe11a17745e12fedf8fb577bb574805d175e51b059edc3322124f3e4161dd1b57a7bb412c9352084215b1ceb15550850b3452a32d46793d6ef85a74086d6863220f27e625f3cd2a864a52808580d1058d2ac8133a4b71b2c1b3dec0bd38311fc2f97d2d7292eb4fe35161acc671164cd1363de38fda514574d49c457d877e91db73a93ac8ca5fc595ca25c25f156398f32322dd71f5904d3b737505bfbf1eee1375118f8d584302f2da7eefaa3c5d7b095f3cb4859384d30a8acb88d2bc2b1ae46a5e760ce4233c0bb9c03a57422009d6c2cd0b3367a5da3581ce065c8037217ecf718fc9b97e40bd40f36038f1136135c9ee5a9d5f2a9e1f3861a27d63510f7ff632506216abd09c1f9c0233c558bda127ef07f018e1e05e4b40cf57ae8965cac3ea994a135f020b6a4e02ac478d3025dfe2f33d12c61771bd363e7deb0e9f260e9a7e0e36e646cdb30ae5e8b7ed55ce45411a4ac4c").unwrap();
    //     run_test::<LONG_MESSAGE_LENGTH, 32>(
    //         msg.as_slice(),
    //         "e786491aaeec8777b3f5c3fe5d1ca1292fb38cc7d4250b19763fd931a9757b12",
    //     )
    //     .expect("Failed test #6");
    // }
}
