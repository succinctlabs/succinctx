use core::fmt::Debug;

use curta::chip::ec::edwards::ed25519::Ed25519 as CurtaEd25519;
use curta::chip::ec::edwards::scalar_mul::generator::ScalarMulEd25519Gadget;
use curta::chip::ec::EllipticCurve;
use curta::math::extension::cubic::parameters::CubicParameters;
use curta::plonky2::stark::config::CurtaConfig;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder as BaseCircuitBuilder;

use crate::frontend::ecc::ed25519::curve::curve_types::Curve;
use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
use crate::frontend::ecc::ed25519::gadgets::curve::{AffinePointTarget, CircuitBuilderCurve};
use crate::frontend::hash::deprecated::sha512::{
    sha512, sha512_variable, CHUNK_BITS_1024, LENGTH_BITS_128,
};
use crate::frontend::num::biguint::BigUintTarget;
use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::prelude::{CircuitBuilder, CircuitVariable, PlonkParameters, Variable};

const MAX_NUM_SIGS: usize = 256;
const COMPRESSED_SIG_AND_PK_LEN_BITS: usize = 512;

#[derive(Clone, Debug)]
pub struct EDDSAPublicKeyTarget<C: Curve>(pub AffinePointTarget<C>);

#[derive(Clone, Debug, CircuitVariable)]
pub struct EDDSASignatureTarget<C: Curve> {
    pub r: AffinePointTarget<C>,
    pub s: NonNativeTarget<C::ScalarField>,
}

#[derive(Clone, Debug)]
pub struct EDDSATargets<C: Curve> {
    pub msgs: Vec<Vec<BoolTarget>>,
    pub sigs: Vec<EDDSASignatureTarget<C>>,
    pub pub_keys: Vec<EDDSAPublicKeyTarget<C>>,
}

// Variable length message EDDSA
#[derive(Clone, Debug)]
pub struct EDDSAVariableTargets<C: Curve> {
    pub msgs: Vec<Vec<BoolTarget>>,
    pub msgs_bit_lengths: Vec<Target>,
    pub sigs: Vec<EDDSASignatureTarget<C>>,
    pub pub_keys: Vec<EDDSAPublicKeyTarget<C>>,
}

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
    builder: &mut BaseCircuitBuilder<F, D>,
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
        u32_targets.push(U32Target(builder.le_sum(u32_chunk.iter().rev())));
    }

    BigUintTarget { limbs: u32_targets }
}

pub const fn calculate_eddsa_num_chunks(msg_len_bits: usize) -> usize {
    ((msg_len_bits + COMPRESSED_SIG_AND_PK_LEN_BITS + LENGTH_BITS_128 + 1) / CHUNK_BITS_1024) + 1
}

// Note: This function should not be used outside of succinctx.
// TODO: Migrate to CircuitVariable
pub fn curta_batch_eddsa_verify_variable<
    F: RichField + Extendable<D>,
    C: Curve,
    E: CubicParameters<F>,
    Config: CurtaConfig<D, F = F, FE = F::Extension>,
    const D: usize,
    // Maximum length of a signed message in bytes.
    const MAX_MSG_LENGTH_BYTES: usize,
>(
    builder: &mut BaseCircuitBuilder<F, D>,
    num_sigs: usize,
) -> EDDSAVariableTargets<C> {
    assert!(num_sigs > 0 && num_sigs <= MAX_NUM_SIGS);

    let max_msg_len_bits = MAX_MSG_LENGTH_BYTES * 8;

    // Note: This will calculate number of chunks in the message, including the compressed sig and pk bits (512 bits).
    let max_num_chunks: usize = calculate_eddsa_num_chunks(max_msg_len_bits);

    // Create the eddsa circuit's virtual targets.
    let mut msgs = Vec::new();
    let mut msgs_bit_lengths = Vec::new();
    let mut sigs = Vec::new();
    let mut pub_keys = Vec::new();
    let mut curta_pub_keys = Vec::new();
    let mut h_scalars_limbs = Vec::new();
    let mut generators = Vec::new();
    let mut sigs_s_limbs = Vec::new();

    for _i in 0..num_sigs {
        let mut msg = Vec::new();
        for _ in 0..max_msg_len_bits {
            // Note that add_virtual_bool_target_safe will do a range check to verify each element is 0 or 1.
            msg.push(builder.add_virtual_bool_target_safe());
        }

        // Targets for the message length and number of chunks
        // TODO: Should we range check that msg_length is less than MAX_MSG_LEN * 8?
        let msg_bit_length = builder.add_virtual_target();
        msgs_bit_lengths.push(msg_bit_length);

        // Note: Add 512 bits for the sig.r and pk_compressed
        let compressed_sig_and_pk_bit_length =
            builder.constant(F::from_canonical_usize(COMPRESSED_SIG_AND_PK_LEN_BITS));
        let hash_msg_bit_length = builder.add(msg_bit_length, compressed_sig_and_pk_bit_length);

        // There is already a calculation for the number of limbs needed for the underlying biguint targets.
        let sig = EDDSASignatureTarget {
            r: builder.add_virtual_affine_point_target(),
            s: builder.add_virtual_nonnative_target(),
        };
        let pub_key = EDDSAPublicKeyTarget(builder.add_virtual_affine_point_target());
        builder.curve_assert_valid(&pub_key.0);
        builder.curve_assert_valid(&sig.r);

        // Convert into format for the curta scalar mul
        curta_pub_keys.push(builder.convert_to_curta_affine_point_target(&pub_key.0));

        // Calculate h = hash(sig.r + pk + msg) mod q
        let mut hash_msg = Vec::new();
        let a = builder.compress_point(&sig.r);
        let r_compressed = reverse_byte_ordering(a.bit_targets.to_vec());
        let b = builder.compress_point(&pub_key.0);
        let pk_compressed = reverse_byte_ordering(b.bit_targets.to_vec());

        for i in 0..256 {
            hash_msg.push(r_compressed[i]);
        }

        for i in 0..256 {
            hash_msg.push(pk_compressed[i]);
        }

        for i in 0..max_msg_len_bits {
            hash_msg.push(msg[i]);
        }

        for _ in
            (max_msg_len_bits + COMPRESSED_SIG_AND_PK_LEN_BITS)..(max_num_chunks * CHUNK_BITS_1024)
        {
            hash_msg.push(builder._false());
        }

        msgs.push(msg);

        let sha512_targets = sha512_variable::<F, D>(builder, max_num_chunks);
        builder.connect(sha512_targets.hash_msg_length_bits, hash_msg_bit_length);

        for i in 0..max_num_chunks * CHUNK_BITS_1024 {
            builder.connect(sha512_targets.message[i].target, hash_msg[i].target);
        }

        let digest = biguint_from_le_bytes(builder, sha512_targets.digest);
        let h_scalar = builder.reduce::<Ed25519Scalar>(&digest);

        let h_scalar_limbs = h_scalar.value.limbs.iter().map(|x| x.0).collect::<Vec<_>>();
        h_scalars_limbs.push(h_scalar_limbs);

        let sig_s_limbs = sig.s.value.limbs.iter().map(|x| x.0).collect::<Vec<_>>();
        sigs_s_limbs.push(sig_s_limbs);

        let generator =
            ScalarMulEd25519Gadget::constant_affine_point(builder, CurtaEd25519::ec_generator());

        pub_keys.push(pub_key);
        sigs.push(sig);
        generators.push(generator);
    }

    // "Pad" the rest of the scalar mul inputs with dummy operands
    for _i in num_sigs..MAX_NUM_SIGS {
        curta_pub_keys.push(ScalarMulEd25519Gadget::constant_affine_point(
            builder,
            CurtaEd25519::ec_generator(),
        ));
        h_scalars_limbs.push([builder.zero(); 8].to_vec());

        generators.push(ScalarMulEd25519Gadget::constant_affine_point(
            builder,
            CurtaEd25519::ec_generator(),
        ));
        sigs_s_limbs.push([builder.zero(); 8].to_vec());
    }

    // Now do the batch scalar mul verification
    let pk_times_h_witnesses = builder.ed_scalar_mul_batch_hint(&curta_pub_keys, &h_scalars_limbs);
    let pk_times_h_results =
        builder.ed_scalar_mul_batch::<E, Config>(&curta_pub_keys, &h_scalars_limbs);

    let s_times_g_witnesses = builder.ed_scalar_mul_batch_hint(&generators, &sigs_s_limbs);
    let s_times_g_results = builder.ed_scalar_mul_batch::<E, Config>(&generators, &sigs_s_limbs);

    for i in 0..num_sigs {
        // Verify the scalar muls
        ScalarMulEd25519Gadget::connect_affine_point(
            builder,
            &pk_times_h_witnesses[i],
            &pk_times_h_results[i],
        );
        ScalarMulEd25519Gadget::connect_affine_point(
            builder,
            &s_times_g_witnesses[i],
            &s_times_g_results[i],
        );

        // Complete the signature verification
        let pk_times_h = builder.convert_from_curta_affine_point_target(&pk_times_h_results[i]);
        let rhs = builder.curve_add(&sigs[i].r, &pk_times_h);
        let s_times_g = builder.convert_from_curta_affine_point_target(&s_times_g_results[i]);
        CircuitBuilderCurve::connect_affine_point(builder, &s_times_g, &rhs);
    }

    EDDSAVariableTargets {
        msgs,
        msgs_bit_lengths,
        pub_keys,
        sigs,
    }
}

// Note: This function should not be used outside of succinctx.
// TODO: Migrate to CircuitVariable
// TODO: If there is one shared message length for all signed messages, then we can optimize this function with sha512, instead of variable_sha512.
// TODO: If there is one shared message for all signed messages, then we can optimize this function by computing the sha512 once.
pub fn curta_batch_eddsa_verify<
    F: RichField + Extendable<D>,
    C: Curve,
    E: CubicParameters<F>,
    Config: CurtaConfig<D, F = F, FE = F::Extension>,
    const D: usize,
>(
    builder: &mut BaseCircuitBuilder<F, D>,
    num_sigs: usize,
    msg_len: u128, // message length in bytes
) -> EDDSATargets<C> {
    assert!(num_sigs > 0 && num_sigs <= MAX_NUM_SIGS);

    // Create the eddsa circuit's virtual targets.
    let mut msgs = Vec::new();
    let mut sigs = Vec::new();
    let mut pub_keys = Vec::new();
    let mut curta_pub_keys = Vec::new();
    let mut h_scalars_limbs = Vec::new();
    let mut generators = Vec::new();
    let mut sigs_s_limbs = Vec::new();

    for _i in 0..num_sigs {
        let mut msg = Vec::new();
        for _ in 0..msg_len * 8 {
            // Note that add_virtual_bool_target_safe will do a range check to verify each element is 0 or 1.
            msg.push(builder.add_virtual_bool_target_safe());
        }

        // There is already a calculation for the number of limbs needed for the underlying biguint targets.
        let sig = EDDSASignatureTarget {
            r: builder.add_virtual_affine_point_target(),
            s: builder.add_virtual_nonnative_target(),
        };
        let pub_key = EDDSAPublicKeyTarget(builder.add_virtual_affine_point_target());
        builder.curve_assert_valid(&pub_key.0);
        builder.curve_assert_valid(&sig.r);

        // Convert into format for the curta scalar mul
        curta_pub_keys.push(builder.convert_to_curta_affine_point_target(&pub_key.0));

        // Calculate h = hash(sig.r + pk + msg) mod q
        let mut hash_msg = Vec::new();
        let a = builder.compress_point(&sig.r);
        let r_compressed = reverse_byte_ordering(a.bit_targets.to_vec());
        let b = builder.compress_point(&pub_key.0);
        let pk_compressed = reverse_byte_ordering(b.bit_targets.to_vec());

        for i in 0..r_compressed.len() {
            hash_msg.push(r_compressed[i]);
        }

        for i in 0..pk_compressed.len() {
            hash_msg.push(pk_compressed[i]);
        }

        for i in 0..msg.len() {
            hash_msg.push(msg[i]);
        }
        msgs.push(msg);

        let digest_bits_target = sha512(builder, &hash_msg);
        let digest = biguint_from_le_bytes(builder, digest_bits_target);
        let h_scalar = builder.reduce::<Ed25519Scalar>(&digest);

        let h_scalar_limbs = h_scalar.value.limbs.iter().map(|x| x.0).collect::<Vec<_>>();
        h_scalars_limbs.push(h_scalar_limbs);

        let sig_s_limbs = sig.s.value.limbs.iter().map(|x| x.0).collect::<Vec<_>>();
        sigs_s_limbs.push(sig_s_limbs);

        let generator =
            ScalarMulEd25519Gadget::constant_affine_point(builder, CurtaEd25519::ec_generator());

        pub_keys.push(pub_key);
        sigs.push(sig);
        generators.push(generator);
    }

    // "Pad" the rest of the scalar mul inputs with dummy operands
    for _i in num_sigs..MAX_NUM_SIGS {
        curta_pub_keys.push(ScalarMulEd25519Gadget::constant_affine_point(
            builder,
            CurtaEd25519::ec_generator(),
        ));
        h_scalars_limbs.push([builder.zero(); 8].to_vec());

        generators.push(ScalarMulEd25519Gadget::constant_affine_point(
            builder,
            CurtaEd25519::ec_generator(),
        ));
        sigs_s_limbs.push([builder.zero(); 8].to_vec());
    }

    // Now do the batch scalar mul verification
    let pk_times_h_witnesses = builder.ed_scalar_mul_batch_hint(&curta_pub_keys, &h_scalars_limbs);
    let pk_times_h_results =
        builder.ed_scalar_mul_batch::<E, Config>(&curta_pub_keys, &h_scalars_limbs);

    let s_times_g_witnesses = builder.ed_scalar_mul_batch_hint(&generators, &sigs_s_limbs);
    let s_times_g_results = builder.ed_scalar_mul_batch::<E, Config>(&generators, &sigs_s_limbs);

    for i in 0..num_sigs {
        // Verify the scalar muls
        ScalarMulEd25519Gadget::connect_affine_point(
            builder,
            &pk_times_h_witnesses[i],
            &pk_times_h_results[i],
        );
        ScalarMulEd25519Gadget::connect_affine_point(
            builder,
            &s_times_g_witnesses[i],
            &s_times_g_results[i],
        );

        // Complete the signature verification
        let pk_times_h = builder.convert_from_curta_affine_point_target(&pk_times_h_results[i]);
        let rhs = builder.curve_add(&sigs[i].r, &pk_times_h);
        let s_times_g = builder.convert_from_curta_affine_point_target(&s_times_g_results[i]);
        CircuitBuilderCurve::connect_affine_point(builder, &s_times_g, &rhs);
    }

    EDDSATargets {
        msgs,
        pub_keys,
        sigs,
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use curta::plonky2::stark::config::CurtaPoseidonGoldilocksConfig;
    use log::debug;
    use num::BigUint;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::{Field, PrimeField};
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder as BaseCircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve, CurveScalar};
    use crate::frontend::ecc::ed25519::curve::ed25519::Ed25519;
    use crate::frontend::ecc::ed25519::curve::eddsa::{
        verify_message, EDDSAPublicKey, EDDSASignature,
    };
    use crate::frontend::ecc::ed25519::field::ed25519_base::Ed25519Base;
    use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
    use crate::frontend::ecc::ed25519::gadgets::eddsa::{
        curta_batch_eddsa_verify, curta_batch_eddsa_verify_variable,
    };
    use crate::frontend::num::biguint::WitnessBigUint;
    use crate::utils;

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

    fn test_eddsa_circuit_with_config(config: CircuitConfig) {
        utils::setup_logger();
        type F = GoldilocksField;
        type E = GoldilocksCubicParameters;
        type SC = CurtaPoseidonGoldilocksConfig;
        type C = PoseidonGoldilocksConfig;
        type Curve = Ed25519;
        const D: usize = 2;

        let mut pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(config);

        let msg = b"plonky2";
        let msg_bits = to_bits(msg.to_vec());

        let priv_key_big_uint = BigUint::parse_bytes(
            b"37459004492869955828084841511595085533954592893308545697612417456352227510728",
            10,
        )
        .unwrap()
            % Ed25519Scalar::order();
        let priv_key = Ed25519Scalar::from_noncanonical_biguint(priv_key_big_uint);
        let pub_key = (CurveScalar(priv_key) * Curve::GENERATOR_PROJECTIVE).to_affine();
        assert!(pub_key.is_valid());

        let sig_r_x_biguint = BigUint::parse_bytes(
            b"34429777554096177233623231228348362084988839912431844356123812156003444176586",
            10,
        )
        .unwrap()
            % Ed25519Base::order();
        let sig_r_x = Ed25519Base::from_noncanonical_biguint(sig_r_x_biguint);
        let sig_r_y_biguint = BigUint::parse_bytes(
            b"22119998304038584770835958502800813263484475345060077692632207186036243708011",
            10,
        )
        .unwrap()
            % Ed25519Base::order();
        let sig_r_y = Ed25519Base::from_noncanonical_biguint(sig_r_y_biguint);
        let sig_r = AffinePoint {
            x: sig_r_x,
            y: sig_r_y,
            zero: false,
        };
        assert!(sig_r.is_valid());

        let sig_s_biguint = BigUint::parse_bytes(
            b"357993861880021552933445860478192139704202248883620882393404009688746217794",
            10,
        )
        .unwrap()
            % Ed25519Scalar::order();
        let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
        let sig = EDDSASignature { r: sig_r, s: sig_s };

        assert!(verify_message(&msg_bits, &sig, &EDDSAPublicKey(pub_key)));

        let eddsa_target = curta_batch_eddsa_verify::<F, Curve, E, SC, D>(
            &mut builder,
            1,
            msg.len().try_into().unwrap(),
        );
        for i in 0..msg_bits.len() {
            pw.set_bool_target(eddsa_target.msgs[0][i], msg_bits[i]);
        }

        pw.set_biguint_target(
            &eddsa_target.pub_keys[0].0.x.value,
            &pub_key.x.to_canonical_biguint(),
        );
        pw.set_biguint_target(
            &eddsa_target.pub_keys[0].0.y.value,
            &pub_key.y.to_canonical_biguint(),
        );

        pw.set_biguint_target(
            &eddsa_target.sigs[0].r.x.value,
            &sig_r.x.to_canonical_biguint(),
        );
        pw.set_biguint_target(
            &eddsa_target.sigs[0].r.y.value,
            &sig_r.y.to_canonical_biguint(),
        );

        pw.set_biguint_target(&eddsa_target.sigs[0].s.value, &sig_s.to_canonical_biguint());

        dbg!(builder.num_gates());

        let circuit_builder_start_time = SystemTime::now();
        let data = builder.build::<C>();
        let circuit_builder_time = circuit_builder_start_time.elapsed().unwrap();

        let proof_start_time = SystemTime::now();
        let proof = data.prove(pw).unwrap();
        let proof_time = proof_start_time.elapsed().unwrap();

        let verify_start_time = SystemTime::now();
        data.verify(proof).unwrap();
        let verify_time = verify_start_time.elapsed().unwrap();

        debug!(
            "circuit_builder_time: {}\nproof_time: {}\nverify_time: {}",
            circuit_builder_time.as_secs(),
            proof_time.as_secs(),
            verify_time.as_secs()
        );
    }

    fn test_eddsa_circuit_with_test_case(
        msgs: Vec<Vec<u8>>,
        pub_keys: Vec<Vec<u8>>,
        sigs: Vec<Vec<u8>>,
    ) {
        utils::setup_logger();

        assert!(msgs.len() == pub_keys.len());
        assert!(pub_keys.len() == sigs.len());

        // TODO: Need to update to handle variable message len
        let msg_len = msgs[0].len();

        type F = GoldilocksField;
        type E = GoldilocksCubicParameters;
        type SC = CurtaPoseidonGoldilocksConfig;
        type C = PoseidonGoldilocksConfig;
        type Curve = Ed25519;
        const D: usize = 2;

        let mut pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());

        let eddsa_target = curta_batch_eddsa_verify::<F, Curve, E, SC, D>(
            &mut builder,
            msgs.len(),
            msg_len.try_into().unwrap(),
        );

        for i in 0..msgs.len() {
            let msg_bits = to_bits(msgs[i].to_vec());

            let pub_key = AffinePoint::new_from_compressed_point(&pub_keys[i]);
            assert!(pub_key.is_valid());

            let sig_r = AffinePoint::new_from_compressed_point(&sigs[i][0..32]);
            assert!(sig_r.is_valid());

            let sig_s_biguint = BigUint::from_bytes_le(&sigs[i][32..64]);
            let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
            let sig = EDDSASignature { r: sig_r, s: sig_s };

            assert!(verify_message(&msg_bits, &sig, &EDDSAPublicKey(pub_key)));

            for j in 0..msg_bits.len() {
                pw.set_bool_target(eddsa_target.msgs[i][j], msg_bits[j]);
            }

            pw.set_biguint_target(
                &eddsa_target.pub_keys[i].0.x.value,
                &pub_key.x.to_canonical_biguint(),
            );
            pw.set_biguint_target(
                &eddsa_target.pub_keys[i].0.y.value,
                &pub_key.y.to_canonical_biguint(),
            );

            pw.set_biguint_target(
                &eddsa_target.sigs[i].r.x.value,
                &sig_r.x.to_canonical_biguint(),
            );
            pw.set_biguint_target(
                &eddsa_target.sigs[i].r.y.value,
                &sig_r.y.to_canonical_biguint(),
            );

            pw.set_biguint_target(&eddsa_target.sigs[i].s.value, &sig_s.to_canonical_biguint());
        }

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();
        inner_data.verify(inner_proof.clone()).unwrap();

        let mut outer_builder =
            BaseCircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data =
            outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(
            &inner_proof_target,
            &inner_verifier_data,
            &inner_data.common,
        );

        let outer_data = outer_builder.build::<C>();
        for gate in outer_data.common.gates.iter() {
            debug!("ecddsa verify recursive gate: {:?}", gate);
        }

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();

        outer_data.verify(outer_proof).unwrap();
    }

    fn test_variable_eddsa_circuit_with_test_case(
        msgs: Vec<Vec<u8>>,
        pub_keys: Vec<Vec<u8>>,
        sigs: Vec<Vec<u8>>,
    ) {
        utils::setup_logger();

        assert!(msgs.len() == pub_keys.len());
        assert!(pub_keys.len() == sigs.len());

        type F = GoldilocksField;
        type E = GoldilocksCubicParameters;
        type SC = CurtaPoseidonGoldilocksConfig;
        type C = PoseidonGoldilocksConfig;
        type Curve = Ed25519;
        const D: usize = 2;

        let mut pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        const MAX_MSG_LEN_BYTES: usize = 128;
        const MAX_MSG_LEN_BITS: usize = MAX_MSG_LEN_BYTES * 8;
        // Length of sig.r and pk_compressed in hash_msg
        let eddsa_target = curta_batch_eddsa_verify_variable::<F, Curve, E, SC, D, MAX_MSG_LEN_BYTES>(
            &mut builder,
            msgs.len(),
        );

        for i in 0..msgs.len() {
            let msg_bits = to_bits(msgs[i].to_vec());

            let pub_key = AffinePoint::new_from_compressed_point(&pub_keys[i]);
            assert!(pub_key.is_valid());

            let sig_r = AffinePoint::new_from_compressed_point(&sigs[i][0..32]);
            assert!(sig_r.is_valid());

            let sig_s_biguint = BigUint::from_bytes_le(&sigs[i][32..64]);
            let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
            let sig = EDDSASignature { r: sig_r, s: sig_s };

            assert!(verify_message(&msg_bits, &sig, &EDDSAPublicKey(pub_key)));

            for j in 0..msg_bits.len() {
                pw.set_bool_target(eddsa_target.msgs[i][j], msg_bits[j]);
            }

            for j in msg_bits.len()..(MAX_MSG_LEN_BITS) {
                pw.set_bool_target(eddsa_target.msgs[i][j], false);
            }

            let msg_len = msg_bits.len();

            pw.set_target(
                eddsa_target.msgs_bit_lengths[i],
                F::from_canonical_usize(msg_len),
            );

            pw.set_biguint_target(
                &eddsa_target.pub_keys[i].0.x.value,
                &pub_key.x.to_canonical_biguint(),
            );
            pw.set_biguint_target(
                &eddsa_target.pub_keys[i].0.y.value,
                &pub_key.y.to_canonical_biguint(),
            );

            pw.set_biguint_target(
                &eddsa_target.sigs[i].r.x.value,
                &sig_r.x.to_canonical_biguint(),
            );
            pw.set_biguint_target(
                &eddsa_target.sigs[i].r.y.value,
                &sig_r.y.to_canonical_biguint(),
            );

            pw.set_biguint_target(&eddsa_target.sigs[i].s.value, &sig_s.to_canonical_biguint());
        }

        let inner_data = builder.build::<C>();
        let circuit_digest = inner_data.verifier_only.circuit_digest;
        debug!("circuit_digest: {:?}", circuit_digest);

        let inner_proof = inner_data.prove(pw).unwrap();
        inner_data.verify(inner_proof.clone()).unwrap();

        let mut outer_builder =
            BaseCircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data =
            outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(
            &inner_proof_target,
            &inner_verifier_data,
            &inner_data.common,
        );

        let outer_data = outer_builder.build::<C>();
        for gate in outer_data.common.gates.iter() {
            debug!("ecddsa verify recursive gate: {:?}", gate);
        }

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();

        outer_data.verify(outer_proof).unwrap();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eddsa_circuit_narrow() {
        test_eddsa_circuit_with_config(CircuitConfig::standard_ecc_config());
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eddsa_circuit_wide() {
        test_eddsa_circuit_with_config(CircuitConfig::wide_ecc_config());
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eddsa_circuit_with_avail_test_case() {
        let msg_bytes = [
            1, 164, 81, 146, 119, 87, 120, 84, 45, 84, 206, 199, 171, 245, 50, 223, 18, 145, 16,
            20, 30, 74, 39, 118, 236, 132, 187, 1, 187, 203, 3, 182, 59, 16, 197, 8, 0, 235, 7, 0,
            0, 0, 0, 0, 0, 25, 2, 0, 0, 0, 0, 0, 0,
        ];
        let pub_key_bytes = [
            43, 167, 192, 11, 252, 193, 43, 86, 163, 6, 196, 30, 196, 76, 65, 16, 66, 208, 184, 55,
            164, 13, 128, 252, 101, 47, 165, 140, 207, 183, 134, 0,
        ];
        let sig_bytes = [
            181, 147, 15, 125, 55, 28, 34, 104, 182, 165, 82, 204, 204, 73, 16, 207, 185, 157, 77,
            145, 128, 9, 51, 132, 54, 115, 29, 172, 162, 95, 181, 176, 47, 25, 165, 27, 174, 193,
            83, 51, 85, 17, 162, 57, 133, 169, 77, 68, 160, 216, 58, 230, 14, 128, 149, 202, 53, 8,
            232, 253, 28, 251, 207, 6,
        ];

        test_eddsa_circuit_with_test_case(
            vec![msg_bytes.to_vec()],
            vec![pub_key_bytes.to_vec()],
            vec![sig_bytes.to_vec()],
        );
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eddsa_circuit_with_celestia_test_case() {
        let msg = "6b080211de3202000000000022480a208909e1b73b7d987e95a7541d96ed484c17a4b0411e98ee4b7c890ad21302ff8c12240801122061263df4855e55fcab7aab0a53ee32cf4f29a1101b56de4a9d249d44e4cf96282a0b089dce84a60610ebb7a81932076d6f6368612d33";
        let pubkey = "77d8fe19357540c479649c7943639b72973093f4c74391dc7a2291d112b9bd64";
        let sig = "9dbab016b0d985150842b9d22220601829efbcb3ee3e43b74e8707dec4fd26d43f1173c00e8c7aef1d7b0a49c2fb9d1a3ddeb798feb74a8abf4c51e90beffe04";

        let msg_bytes = hex::decode(msg).unwrap();
        let pub_key_bytes = hex::decode(pubkey).unwrap();
        let sig_bytes = hex::decode(sig).unwrap();

        test_eddsa_circuit_with_test_case(
            vec![msg_bytes.to_vec()],
            vec![pub_key_bytes.to_vec()],
            vec![sig_bytes.to_vec()],
        );
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_variable_eddsa_circuit_with_celestia_test_case() {
        let msg = "6c080211f82a00000000000022480a2036f2d954fe1ba37c5036cb3c6b366d0daf68fccbaa370d9490361c51a0a38b61122408011220cddf370e891591c9d912af175c966cd8dfa44b2c517e965416b769eb4b9d5d8d2a0c08f6b097a50610dffbcba90332076d6f6368612d33";
        let pubkey = "de25aec935b10f657b43fa97e5a8d4e523bdb0f9972605f0b064eff7b17048ba";
        let sig = "091576e9e3ad0e5ba661f7398e1adb3976ba647b579b8e4a224d1d02b591ade6aedb94d3bf55d258f089d6413155a57adfd4932418a798c2d68b29850f6fb50b";

        let msg_bytes = hex::decode(msg).unwrap();
        let pub_key_bytes = hex::decode(pubkey).unwrap();
        let sig_bytes = hex::decode(sig).unwrap();

        test_variable_eddsa_circuit_with_test_case(
            vec![msg_bytes.to_vec()],
            vec![pub_key_bytes.to_vec()],
            vec![sig_bytes.to_vec()],
        );
    }
}
