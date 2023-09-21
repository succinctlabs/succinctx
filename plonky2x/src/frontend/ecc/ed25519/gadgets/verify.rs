use num::BigUint;
use plonky2::field::types::PrimeField;

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve};
use crate::frontend::ecc::ed25519::curve::ed25519::Ed25519;
use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
use crate::frontend::ecc::ed25519::gadgets::curve::{AffinePointTarget, CircuitBuilderCurve};
use crate::frontend::ecc::ed25519::gadgets::eddsa::{
    verify_variable_signatures_circuit, EDDSASignatureTarget,
};
use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::frontend::vars::U32Variable;
use crate::prelude::{BoolVariable, BytesVariable, CircuitBuilder, Field, PlonkParameters};

pub struct DummySignatureTarget<C: Curve, const MAX_MESSAGE_LENGTH: usize> {
    // TODO: Change back to EDDSAPublicKeyTarget after type alias on EDDSAPublicKeyTarget
    pub pubkey: AffinePointTarget<C>,
    pub signature: EDDSASignatureTarget<C>,
    pub message: BytesVariable<MAX_MESSAGE_LENGTH>,
    pub message_byte_length: U32Variable,
}

// Private key is [0u8; 32]
pub const DUMMY_PUBLIC_KEY: [u8; 32] = [
    59, 106, 39, 188, 206, 182, 164, 45, 98, 163, 168, 208, 42, 111, 13, 115, 101, 50, 21, 119, 29,
    226, 67, 166, 58, 192, 72, 161, 139, 89, 218, 41,
];
pub const DUMMY_MSG: [u8; 32] = [0u8; 32];
pub const DUMMY_MSG_LENGTH_BYTES: u32 = 32;
pub const DUMMY_MSG_LENGTH_BITS: u32 = 256;
// dummy_msg signed by the dummy private key
pub const DUMMY_SIGNATURE: [u8; 64] = [
    61, 161, 235, 223, 169, 110, 221, 24, 29, 190, 54, 89, 209, 192, 81, 196, 49, 240, 86, 165,
    173, 106, 151, 166, 13, 92, 202, 16, 70, 4, 56, 120, 53, 70, 70, 30, 49, 40, 95, 197, 159, 145,
    199, 7, 38, 66, 116, 80, 97, 226, 69, 29, 95, 243, 59, 204, 216, 195, 199, 77, 171, 202, 246,
    10,
];

pub trait EDDSABatchVerify<L: PlonkParameters<D>, const D: usize> {
    type Curve: Curve;
    type ScalarField: PrimeField;

    /// Dummy targets are used for inactive validators and will always verify.
    /// Invokers of verify_signatures must ensure the valid signatures are constrained correctly.
    fn get_dummy_targets<const MAX_MESSAGE_BYTE_LENGTH: usize>(
        &mut self,
    ) -> DummySignatureTarget<Self::Curve, MAX_MESSAGE_BYTE_LENGTH>;

    /// Verifies the signatures of the validators in the validator set.
    /// validator_active is a bit vector of length VALIDATOR_SET_SIZE_MAX, where each bit indicates whether the corresponding validator signed this round.
    /// message_byte_lengths is a vector of length VALIDATOR_SET_SIZE_MAX, where each element is the (variable) byte length of the corresponding message.
    fn verify_signatures<
        const VALIDATOR_SET_SIZE_MAX: usize,
        const MAX_MESSAGE_BYTE_LENGTH: usize,
    >(
        &mut self,
        validator_active: &[BoolVariable],
        messages: Vec<BytesVariable<MAX_MESSAGE_BYTE_LENGTH>>,
        message_byte_lengths: Vec<U32Variable>,
        eddsa_sig_targets: Vec<EDDSASignatureTarget<Self::Curve>>,
        eddsa_pubkey_targets: Vec<AffinePointTarget<Self::Curve>>,
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

    /// Verifies the signatures of the validators in the validator set.
    fn verify_signatures<
        const VALIDATOR_SET_SIZE_MAX: usize,
        const MAX_MESSAGE_BYTE_LENGTH: usize,
    >(
        &mut self,
        // This message should be range-checked before being passed in.
        validator_active: &[BoolVariable],
        messages: Vec<BytesVariable<MAX_MESSAGE_BYTE_LENGTH>>,
        message_byte_lengths: Vec<U32Variable>,
        eddsa_sig_targets: Vec<EDDSASignatureTarget<Self::Curve>>,
        eddsa_pubkey_targets: Vec<AffinePointTarget<Self::Curve>>,
    ) {
        let dummy_target = self.get_dummy_targets();

        let eddsa_target = verify_variable_signatures_circuit::<
            L::Field,
            Self::Curve,
            L::CubicParams,
            L::CurtaConfig,
            D,
            MAX_MESSAGE_BYTE_LENGTH,
        >(&mut self.api, messages.len());

        // If the validator is active, use the corresponding signature and public key. Otherwise, use the dummy signature and public key.
        for i in 0..VALIDATOR_SET_SIZE_MAX {
            let eddsa_pubkey = self.select(
                validator_active[i],
                eddsa_pubkey_targets[i].clone(),
                dummy_target.pubkey.clone(),
            );

            let eddsa_sig = self.select(
                validator_active[i],
                eddsa_sig_targets[i].clone(),
                dummy_target.signature.clone(),
            );

            let msg = self.select(validator_active[i], messages[i], dummy_target.message);

            let byte_length = self.select(
                validator_active[i],
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
    use num::BigUint;
    use plonky2::field::types::Field;

    use super::*;
    use crate::frontend::ecc::ed25519::curve::curve_types::AffinePoint;
    use crate::frontend::ecc::ed25519::curve::eddsa::{
        verify_message, EDDSAPublicKey, EDDSASignature,
    };
    use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
    use crate::frontend::ecc::ed25519::gadgets::eddsa::EDDSASignatureTargetValue;
    use crate::prelude::{ArrayVariable, DefaultBuilder};
    use crate::utils::to_be_bits;

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

    fn verify_eddsa_signature(msg_bytes: Vec<u8>, pub_key_bytes: Vec<u8>, sig_bytes: Vec<u8>) {
        type Curve = Ed25519;

        let mut builder = DefaultBuilder::new();

        const VALIDATOR_MESSAGE_BYTES_LENGTH_MAX: usize = 124;

        let validator_active = builder.read::<ArrayVariable<BoolVariable, 1>>();
        let msg_bytes_variable =
            builder.read::<ArrayVariable<BytesVariable<VALIDATOR_MESSAGE_BYTES_LENGTH_MAX>, 1>>();
        let msg_byte_length = builder.read::<ArrayVariable<U32Variable, 1>>();
        let eddsa_sig_target = builder.read::<ArrayVariable<EDDSASignatureTarget<Curve>, 1>>();
        let eddsa_pub_key_target = builder.read::<ArrayVariable<AffinePointTarget<Curve>, 1>>();

        builder.verify_signatures::<1, VALIDATOR_MESSAGE_BYTES_LENGTH_MAX>(
            &validator_active.as_vec(),
            msg_bytes_variable.as_vec(),
            msg_byte_length.as_vec(),
            eddsa_sig_target.as_vec(),
            eddsa_pub_key_target.as_vec(),
        );

        let circuit = builder.build();

        let mut new_msg_bytes = msg_bytes.clone();

        new_msg_bytes.resize(VALIDATOR_MESSAGE_BYTES_LENGTH_MAX, 0u8);

        let pub_key_uncompressed: AffinePoint<Curve> =
            AffinePoint::new_from_compressed_point(&pub_key_bytes);

        let sig_r = AffinePoint::new_from_compressed_point(&sig_bytes[0..32]);
        assert!(sig_r.is_valid());

        let sig_s_biguint = BigUint::from_bytes_le(&sig_bytes[32..64]);
        let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
        let sig = EDDSASignature { r: sig_r, s: sig_s };

        assert!(verify_message(
            &to_be_bits(&msg_bytes),
            &sig,
            &EDDSAPublicKey(pub_key_uncompressed)
        ));
        println!("verified signature");

        let mut input = circuit.input();
        input.write::<ArrayVariable<BoolVariable, 1>>(vec![true]);
        input
            .write::<ArrayVariable<BytesVariable<124>, 1>>(vec![new_msg_bytes.try_into().unwrap()]);
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
    fn test_verify_round_absent_eddsa_signature() {
        // First signature from block 11000
        let msg = "6c080211f82a00000000000022480a2036f2d954fe1ba37c5036cb3c6b366d0daf68fccbaa370d9490361c51a0a38b61122408011220cddf370e891591c9d912af175c966cd8dfa44b2c517e965416b769eb4b9d5d8d2a0c08f6b097a50610dffbcba90332076d6f6368612d33";
        let pubkey = "de25aec935b10f657b43fa97e5a8d4e523bdb0f9972605f0b064eff7b17048ba";
        let sig = "091576e9e3ad0e5ba661f7398e1adb3976ba647b579b8e4a224d1d02b591ade6aedb94d3bf55d258f089d6413155a57adfd4932418a798c2d68b29850f6fb50b";
        let msg_bytes = hex::decode(msg).unwrap();
        let pub_key_bytes = hex::decode(pubkey).unwrap();
        let sig_bytes = hex::decode(sig).unwrap();
        verify_eddsa_signature(msg_bytes, pub_key_bytes, sig_bytes)
    }
    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_verify_round_present_eddsa_signature() {
        // First signature from block 11105 (round present)
        let msg = "74080211612b00000000000019010000000000000022480a205047a5a855854ca8bc610fb47ee849084c04fe25a2f037a07de6ae343c55216b122408011220cb05d8adc7c24d55f06d3bd0aea50620d3f0d73a9656a9073cc47a959a0961672a0b08acbd97a50610b1a5f31132076d6f6368612d33";
        let pubkey = "de25aec935b10f657b43fa97e5a8d4e523bdb0f9972605f0b064eff7b17048ba";
        let sig = "b4ea1e808fa88073ae8fe9d9d33d99ae7990cb148c81f2158e56c90aa45d9c3457aaffb875853956b0093ab1b3606b4eb450f5b476e54c508375a25c78376e0d";
        let msg_bytes = hex::decode(msg).unwrap();
        let pub_key_bytes = hex::decode(pubkey).unwrap();
        let sig_bytes = hex::decode(sig).unwrap();
        verify_eddsa_signature(msg_bytes, pub_key_bytes, sig_bytes)
    }
    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_verify_dummy_signature() {
        verify_eddsa_signature(
            DUMMY_MSG.to_vec(),
            DUMMY_PUBLIC_KEY.to_vec(),
            DUMMY_SIGNATURE.to_vec(),
        )
    }
}
