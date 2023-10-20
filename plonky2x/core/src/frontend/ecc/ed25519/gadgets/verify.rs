use num::BigUint;
use plonky2::field::types::PrimeField;

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve};
use crate::frontend::ecc::ed25519::curve::ed25519::Ed25519;
use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
use crate::frontend::ecc::ed25519::gadgets::curve::{AffinePointTarget, CircuitBuilderCurve};
use crate::frontend::ecc::ed25519::gadgets::eddsa::{
    curta_batch_eddsa_verify_variable, EDDSASignatureTarget,
};
use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::frontend::vars::U32Variable;
use crate::prelude::{
    ArrayVariable, BoolVariable, BytesVariable, CircuitBuilder, Field, PlonkParameters,
};

pub struct DummySignatureTarget<C: Curve, const MAX_MESSAGE_LENGTH: usize> {
    pub pubkey: AffinePointTarget<C>,
    pub signature: EDDSASignatureTarget<C>,
    pub message: BytesVariable<MAX_MESSAGE_LENGTH>,
    pub message_byte_length: U32Variable,
}

// DUMMY_PRIVATE_KEY is [1u8; 32].
pub const DUMMY_PUBLIC_KEY: [u8; 32] = [
    138, 136, 227, 221, 116, 9, 241, 149, 253, 82, 219, 45, 60, 186, 93, 114, 202, 103, 9, 191, 29,
    148, 18, 27, 243, 116, 136, 1, 180, 15, 111, 92,
];
pub const DUMMY_MSG: [u8; 32] = [0u8; 32];
pub const DUMMY_MSG_LENGTH_BYTES: u32 = 32;
pub const DUMMY_MSG_LENGTH_BITS: u32 = 256;

// Produced by signing DUMMY_MSG with DUMMY_PRIVATE_KEY.
pub const DUMMY_SIGNATURE: [u8; 64] = [
    55, 20, 104, 158, 84, 120, 194, 17, 6, 237, 157, 164, 85, 88, 158, 137, 187, 119, 187, 240,
    159, 73, 80, 63, 133, 162, 74, 91, 48, 53, 6, 138, 1, 41, 22, 121, 249, 46, 198, 145, 155, 102,
    3, 210, 168, 135, 173, 55, 252, 72, 45, 126, 169, 178, 191, 7, 153, 67, 112, 90, 150, 33, 140,
    7,
];

pub trait EDDSABatchVerify<L: PlonkParameters<D>, const D: usize> {
    type Curve: Curve;
    type ScalarField: PrimeField;

    /// Dummy targets are used for inactive signatures and will always verify.
    /// Invokers of batch_eddsa_verify must ensure the valid signatures are constrained correctly.
    fn get_dummy_targets<const MAX_MESSAGE_BYTE_LENGTH: usize>(
        &mut self,
    ) -> DummySignatureTarget<Self::Curve, MAX_MESSAGE_BYTE_LENGTH>;

    /// Verifies NUM_SIGS EdDSA signatures. Assumes all messages are the same length.
    fn batch_eddsa_verify<const NUM_SIGS: usize, const MESSAGE_BYTE_LENGTH: usize>(
        &mut self,
        messages: ArrayVariable<BytesVariable<MESSAGE_BYTE_LENGTH>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureTarget<Self::Curve>, NUM_SIGS>,
        pubkeys: ArrayVariable<AffinePointTarget<Self::Curve>, NUM_SIGS>,
    );

    /// Verifies NUM_SIGS EdDSA signatures. is_active is a bit vector of length NUM_SIGS, where each bit indicates whether to verify the corresponding signature.
    /// message_byte_lengths is a vector of length NUM_SIGS, where each element is the (variable) byte length of the corresponding message.
    fn conditional_batch_eddsa_verify<const NUM_SIGS: usize, const MAX_MESSAGE_BYTE_LENGTH: usize>(
        &mut self,
        is_active: ArrayVariable<BoolVariable, NUM_SIGS>,
        message_byte_lengths: ArrayVariable<U32Variable, NUM_SIGS>,
        messages: ArrayVariable<BytesVariable<MAX_MESSAGE_BYTE_LENGTH>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureTarget<Self::Curve>, NUM_SIGS>,
        pubkeys: ArrayVariable<AffinePointTarget<Self::Curve>, NUM_SIGS>,
    );
}

impl<L: PlonkParameters<D>, const D: usize> EDDSABatchVerify<L, D> for CircuitBuilder<L, D> {
    type Curve = Ed25519;
    type ScalarField = Ed25519Scalar;

    fn get_dummy_targets<const MAX_MESSAGE_BYTE_LENGTH: usize>(
        &mut self,
    ) -> DummySignatureTarget<Self::Curve, MAX_MESSAGE_BYTE_LENGTH> {
        let pub_key_uncompressed: AffinePoint<Self::Curve> =
            AffinePoint::new_from_compressed_point(&DUMMY_PUBLIC_KEY);
        let pubkey = self.constant::<AffinePointTarget<Self::Curve>>(pub_key_uncompressed);

        let sig_r: AffinePoint<Self::Curve> =
            AffinePoint::new_from_compressed_point(&DUMMY_SIGNATURE[0..32]);
        assert!(sig_r.is_valid());
        let sig_s_biguint = BigUint::from_bytes_le(&DUMMY_SIGNATURE[32..64]);
        let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
        let signature = EDDSASignatureTarget {
            r: self.constant::<AffinePointTarget<Self::Curve>>(sig_r),
            s: self.constant::<NonNativeTarget<Self::ScalarField>>(sig_s),
        };

        let message = self.zero::<BytesVariable<MAX_MESSAGE_BYTE_LENGTH>>();

        let dummy_msg_byte_length = self.constant::<U32Variable>(DUMMY_MSG_LENGTH_BYTES);

        DummySignatureTarget {
            pubkey,
            signature,
            message,
            message_byte_length: dummy_msg_byte_length,
        }
    }

    fn batch_eddsa_verify<const NUM_SIGS: usize, const MESSAGE_BYTE_LENGTH: usize>(
        &mut self,
        messages: ArrayVariable<BytesVariable<MESSAGE_BYTE_LENGTH>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureTarget<Self::Curve>, NUM_SIGS>,
        pubkeys: ArrayVariable<AffinePointTarget<Self::Curve>, NUM_SIGS>,
    ) {
        let eddsa_target = curta_batch_eddsa_verify_variable::<
            L::Field,
            Self::Curve,
            L::CubicParams,
            L::CurtaConfig,
            D,
            MESSAGE_BYTE_LENGTH,
        >(&mut self.api, NUM_SIGS);

        for i in 0..NUM_SIGS {
            let byte_length = self.constant::<U32Variable>(MESSAGE_BYTE_LENGTH as u32);

            // TODO: Simplify these constraints after verify_variable_signatures_circuit uses CircuitVariable
            let msg_bool_targets = self.to_be_bits(messages[i]);
            for j in 0..MESSAGE_BYTE_LENGTH * 8 {
                self.api
                    .connect(eddsa_target.msgs[i][j].target, msg_bool_targets[j].0 .0);
            }

            // TODO: verify_variable_signatures_circuit expects bit length, will be removed in future
            let eight_u32 = self.constant::<U32Variable>(8);
            let bit_length = self.mul(byte_length, eight_u32);
            self.api
                .connect(eddsa_target.msgs_bit_lengths[i], bit_length.0 .0);

            self.api
                .connect_nonnative(&eddsa_target.sigs[i].s, &signatures[i].s);
            self.api
                .connect_affine_point(&signatures[i].r, &eddsa_target.sigs[i].r);

            self.api
                .connect_affine_point(&pubkeys[i], &eddsa_target.pub_keys[i].0);
        }
    }

    /// Verifies signatures marked with is_active.
    fn conditional_batch_eddsa_verify<
        const NUM_SIGS: usize,
        const MAX_MESSAGE_BYTE_LENGTH: usize,
    >(
        &mut self,
        is_active: ArrayVariable<BoolVariable, NUM_SIGS>,
        message_byte_lengths: ArrayVariable<U32Variable, NUM_SIGS>,
        messages: ArrayVariable<BytesVariable<MAX_MESSAGE_BYTE_LENGTH>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureTarget<Self::Curve>, NUM_SIGS>,
        pubkeys: ArrayVariable<AffinePointTarget<Self::Curve>, NUM_SIGS>,
    ) {
        let dummy_target = self.get_dummy_targets();

        let eddsa_target = curta_batch_eddsa_verify_variable::<
            L::Field,
            Self::Curve,
            L::CubicParams,
            L::CurtaConfig,
            D,
            MAX_MESSAGE_BYTE_LENGTH,
        >(&mut self.api, NUM_SIGS);

        // If the validator is active, use the corresponding signature and public key. Otherwise, use the dummy signature and public key.
        for i in 0..NUM_SIGS {
            let eddsa_pubkey = self.select(
                is_active[i],
                pubkeys[i].clone(),
                dummy_target.pubkey.clone(),
            );

            let eddsa_sig = self.select(
                is_active[i],
                signatures[i].clone(),
                dummy_target.signature.clone(),
            );

            let msg = self.select(is_active[i], messages[i], dummy_target.message);

            let byte_length = self.select(
                is_active[i],
                message_byte_lengths[i],
                dummy_target.message_byte_length,
            );

            // TODO: Simplify these constraints after verify_variable_signatures_circuit uses CircuitVariable
            let msg_bool_targets = self.to_be_bits(msg);
            for j in 0..MAX_MESSAGE_BYTE_LENGTH * 8 {
                self.api
                    .connect(eddsa_target.msgs[i][j].target, msg_bool_targets[j].0 .0);
            }

            // TODO: verify_variable_signatures_circuit expects bit length, will be removed in future
            let eight_u32 = self.constant::<U32Variable>(8);
            let bit_length = self.mul(byte_length, eight_u32);
            self.api
                .connect(eddsa_target.msgs_bit_lengths[i], bit_length.0 .0);

            self.api
                .connect_nonnative(&eddsa_target.sigs[i].s, &eddsa_sig.s);
            self.api
                .connect_affine_point(&eddsa_sig.r, &eddsa_target.sigs[i].r);

            self.api
                .connect_affine_point(&eddsa_pubkey, &eddsa_target.pub_keys[i].0);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use ed25519_dalek::{Signature, VerifyingKey};
    use num::BigUint;
    use plonky2::field::types::Field;

    use super::*;
    use crate::frontend::ecc::ed25519::curve::curve_types::AffinePoint;
    use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
    use crate::frontend::ecc::ed25519::gadgets::eddsa::EDDSASignatureTargetValue;
    use crate::prelude::{ArrayVariable, DefaultBuilder};

    #[test]
    fn test_generate_signature() {
        let priv_key_bytes = [0u8; 32];
        let signing_key = ed25519_consensus::SigningKey::try_from(&priv_key_bytes[..])
            .expect("failed to create key");

        let verification_key = signing_key.verification_key();

        println!("public key: {:?}", verification_key.clone().to_bytes());

        let signature = signing_key.sign(&[0u8; 32]);
        println!("signature: {:?}", hex::encode(signature.clone().to_bytes()));
        println!("signature: {:?}", signature.clone().to_bytes());

        verification_key
            .verify(&signature, &[0u8; 32])
            .expect("failed to verify signature");
    }

    fn verify_eddsa_signature<const MSG_BYTES_LENGTH: usize>(
        msg_bytes: Vec<u8>,
        pub_key_bytes: Vec<u8>,
        sig_bytes: Vec<u8>,
    ) {
        let pub_key = VerifyingKey::try_from(pub_key_bytes.as_slice()).unwrap();
        let sig = Signature::from_slice(&sig_bytes).unwrap();
        pub_key
            .verify_strict(&msg_bytes, &sig)
            .expect("signature verification failed");
        println!("verified signature");

        type Curve = Ed25519;

        let mut builder = DefaultBuilder::new();

        let msg = builder.read::<ArrayVariable<BytesVariable<MSG_BYTES_LENGTH>, 1>>();
        let signature = builder.read::<ArrayVariable<EDDSASignatureTarget<Curve>, 1>>();
        let pubkey = builder.read::<ArrayVariable<AffinePointTarget<Curve>, 1>>();

        builder.batch_eddsa_verify::<1, MSG_BYTES_LENGTH>(msg, signature, pubkey);

        let circuit = builder.build();

        let pub_key_uncompressed: AffinePoint<Curve> =
            AffinePoint::new_from_compressed_point(&pub_key_bytes);

        let sig_r = AffinePoint::new_from_compressed_point(&sig_bytes[0..32]);
        assert!(sig_r.is_valid());

        let sig_s_biguint = BigUint::from_bytes_le(&sig_bytes[32..64]);
        let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);

        let mut input = circuit.input();
        input.write::<ArrayVariable<BytesVariable<MSG_BYTES_LENGTH>, 1>>(vec![msg_bytes
            .try_into()
            .unwrap()]);
        input.write::<ArrayVariable<EDDSASignatureTarget<Curve>, 1>>(vec![
            EDDSASignatureTargetValue { r: sig_r, s: sig_s },
        ]);
        input.write::<ArrayVariable<AffinePointTarget<Curve>, 1>>(vec![pub_key_uncompressed]);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    fn verify_conditional_eddsa_signature(
        msg_bytes: Vec<u8>,
        pub_key_bytes: Vec<u8>,
        sig_bytes: Vec<u8>,
        active: bool,
    ) {
        let pub_key = VerifyingKey::try_from(pub_key_bytes.as_slice()).unwrap();
        let sig = Signature::from_slice(&sig_bytes).unwrap();
        pub_key
            .verify_strict(&msg_bytes, &sig)
            .expect("signature verification failed");
        println!("verified signature");

        type Curve = Ed25519;

        let mut builder = DefaultBuilder::new();

        const MESSAGE_BYTES_LENGTH_MAX: usize = 124;

        let is_active = builder.read::<ArrayVariable<BoolVariable, 1>>();
        let msg_bytes_variable =
            builder.read::<ArrayVariable<BytesVariable<MESSAGE_BYTES_LENGTH_MAX>, 1>>();
        let msg_byte_length = builder.read::<ArrayVariable<U32Variable, 1>>();
        let eddsa_sig_target = builder.read::<ArrayVariable<EDDSASignatureTarget<Curve>, 1>>();
        let eddsa_pub_key_target = builder.read::<ArrayVariable<AffinePointTarget<Curve>, 1>>();

        builder.conditional_batch_eddsa_verify::<1, MESSAGE_BYTES_LENGTH_MAX>(
            is_active,
            msg_byte_length,
            msg_bytes_variable,
            eddsa_sig_target,
            eddsa_pub_key_target,
        );

        let circuit = builder.build();

        let mut new_msg_bytes = msg_bytes.clone();

        new_msg_bytes.resize(MESSAGE_BYTES_LENGTH_MAX, 0u8);

        let pub_key_uncompressed: AffinePoint<Curve> =
            AffinePoint::new_from_compressed_point(&pub_key_bytes);

        let sig_r = AffinePoint::new_from_compressed_point(&sig_bytes[0..32]);
        assert!(sig_r.is_valid());

        let sig_s_biguint = BigUint::from_bytes_le(&sig_bytes[32..64]);
        let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);

        let mut input = circuit.input();
        input.write::<ArrayVariable<BoolVariable, 1>>(vec![active]);
        input.write::<ArrayVariable<BytesVariable<MESSAGE_BYTES_LENGTH_MAX>, 1>>(vec![
            new_msg_bytes.try_into().unwrap(),
        ]);
        input.write::<ArrayVariable<U32Variable, 1>>(vec![msg_bytes.len() as u32]);
        input.write::<ArrayVariable<EDDSASignatureTarget<Curve>, 1>>(vec![
            EDDSASignatureTargetValue { r: sig_r, s: sig_s },
        ]);
        input.write::<ArrayVariable<AffinePointTarget<Curve>, 1>>(vec![pub_key_uncompressed]);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_verify_eddsa_signature_conditional_active() {
        let msg = "6c080211f82a00000000000022480a2036f2d954fe1ba37c5036cb3c6b366d0daf68fccbaa370d9490361c51a0a38b61122408011220cddf370e891591c9d912af175c966cd8dfa44b2c517e965416b769eb4b9d5d8d2a0c08f6b097a50610dffbcba90332076d6f6368612d33";
        let pubkey = "de25aec935b10f657b43fa97e5a8d4e523bdb0f9972605f0b064eff7b17048ba";
        let sig = "091576e9e3ad0e5ba661f7398e1adb3976ba647b579b8e4a224d1d02b591ade6aedb94d3bf55d258f089d6413155a57adfd4932418a798c2d68b29850f6fb50b";
        let msg_bytes = hex::decode(msg).unwrap();
        let pub_key_bytes = hex::decode(pubkey).unwrap();
        let sig_bytes = hex::decode(sig).unwrap();
        verify_conditional_eddsa_signature(msg_bytes, pub_key_bytes, sig_bytes, true)
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_verify_eddsa_avail() {
        let msg_bytes: [u8; 53] = [
            1, 164, 81, 146, 119, 87, 120, 84, 45, 84, 206, 199, 171, 245, 50, 223, 18, 145, 16,
            20, 30, 74, 39, 118, 236, 132, 187, 1, 187, 203, 3, 182, 59, 16, 197, 8, 0, 235, 7, 0,
            0, 0, 0, 0, 0, 25, 2, 0, 0, 0, 0, 0, 0,
        ];
        let pub_key_bytes: [u8; 32] = [
            43, 167, 192, 11, 252, 193, 43, 86, 163, 6, 196, 30, 196, 76, 65, 16, 66, 208, 184, 55,
            164, 13, 128, 252, 101, 47, 165, 140, 207, 183, 134, 0,
        ];
        let sig_bytes: [u8; 64] = [
            181, 147, 15, 125, 55, 28, 34, 104, 182, 165, 82, 204, 204, 73, 16, 207, 185, 157, 77,
            145, 128, 9, 51, 132, 54, 115, 29, 172, 162, 95, 181, 176, 47, 25, 165, 27, 174, 193,
            83, 51, 85, 17, 162, 57, 133, 169, 77, 68, 160, 216, 58, 230, 14, 128, 149, 202, 53, 8,
            232, 253, 28, 251, 207, 6,
        ];
        verify_conditional_eddsa_signature(
            msg_bytes.to_vec(),
            pub_key_bytes.to_vec(),
            sig_bytes.to_vec(),
            true,
        )
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_verify_eddsa_signature_conditional_dummy() {
        verify_conditional_eddsa_signature(
            DUMMY_MSG.to_vec(),
            DUMMY_PUBLIC_KEY.to_vec(),
            DUMMY_SIGNATURE.to_vec(),
            false,
        )
    }
    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_verify_eddsa_signature_fixed() {
        const MSG_BYTES_LENGTH: usize = 117;
        let msg = "74080211612b00000000000019010000000000000022480a205047a5a855854ca8bc610fb47ee849084c04fe25a2f037a07de6ae343c55216b122408011220cb05d8adc7c24d55f06d3bd0aea50620d3f0d73a9656a9073cc47a959a0961672a0b08acbd97a50610b1a5f31132076d6f6368612d33";
        let pubkey = "de25aec935b10f657b43fa97e5a8d4e523bdb0f9972605f0b064eff7b17048ba";
        let sig = "b4ea1e808fa88073ae8fe9d9d33d99ae7990cb148c81f2158e56c90aa45d9c3457aaffb875853956b0093ab1b3606b4eb450f5b476e54c508375a25c78376e0d";
        let msg_bytes = hex::decode(msg).unwrap();
        let pub_key_bytes = hex::decode(pubkey).unwrap();
        let sig_bytes = hex::decode(sig).unwrap();
        verify_eddsa_signature::<MSG_BYTES_LENGTH>(msg_bytes, pub_key_bytes, sig_bytes)
    }

    #[test]
    fn generate_eddsa_public_key() {
        let priv_key_bytes = [1u8; 32];
        let signing_key = ed25519_consensus::SigningKey::try_from(&priv_key_bytes[..])
            .expect("failed to create key");

        let verification_key = signing_key.verification_key();

        println!("public key: {:?}", verification_key.clone().to_bytes());

        let signature = signing_key.sign(&[0u8; 32]);

        println!("signature: {:?}", signature.clone().to_bytes());

        verification_key
            .verify(&signature, &[0u8; 32])
            .expect("failed to verify signature");
    }
}
