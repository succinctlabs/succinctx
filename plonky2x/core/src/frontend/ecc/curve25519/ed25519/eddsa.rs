use core::fmt::Debug;

use ::curta::chip::ec::edwards::ed25519::params::Ed25519ScalarField;
use ::curta::chip::field::parameters::FieldParameters;
use curta::chip::ec::edwards::ed25519::params::Ed25519Parameters;
use curta::chip::ec::edwards::EdwardsParameters;
use curta::chip::ec::point::AffinePoint;
use curve25519_dalek::edwards::CompressedEdwardsY;
use num_bigint::BigUint;
use plonky2::hash::hash_types::RichField;

use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::num::biguint::biguint_from_bytes_variable;
use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeVariable};
use crate::prelude::{
    ArrayVariable, BoolVariable, BytesVariable, CircuitBuilder, CircuitVariable, PlonkParameters,
    U32Variable, Variable,
};

const MAX_NUM_SIGS: usize = 256;

#[derive(Clone, Debug, CircuitVariable)]
pub struct EDDSASignatureVariable<FF: FieldParameters> {
    pub r: CompressedEdwardsYVariable,
    pub s: NonNativeVariable<FF>,
}

// DUMMY_PRIVATE_KEY is [1u8; 32].
pub const DUMMY_PUBLIC_KEY: [u8; 32] = [
    138, 136, 227, 221, 116, 9, 241, 149, 253, 82, 219, 45, 60, 186, 93, 114, 202, 103, 9, 191, 29,
    148, 18, 27, 243, 116, 136, 1, 180, 15, 111, 92,
];

pub const DUMMY_MSG_LENGTH_BYTES: u32 = 32;
pub const DUMMY_MSG: [u8; 32] = [0u8; DUMMY_MSG_LENGTH_BYTES as usize];

// Produced by signing DUMMY_MSG with DUMMY_PRIVATE_KEY.
pub const DUMMY_SIGNATURE: [u8; 64] = [
    55, 20, 104, 158, 84, 120, 194, 17, 6, 237, 157, 164, 85, 88, 158, 137, 187, 119, 187, 240,
    159, 73, 80, 63, 133, 162, 74, 91, 48, 53, 6, 138, 1, 41, 22, 121, 249, 46, 198, 145, 155, 102,
    3, 210, 168, 135, 173, 55, 252, 72, 45, 126, 169, 178, 191, 7, 153, 67, 112, 90, 150, 33, 140,
    7,
];

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn get_dummy_variables<const MAX_MSG_BYTE_LENGTH: usize>(
        &mut self,
    ) -> (
        CompressedEdwardsYVariable,
        EDDSASignatureVariable<Ed25519ScalarField>,
        BytesVariable<MAX_MSG_BYTE_LENGTH>,
        U32Variable,
    ) {
        let pub_key: CompressedEdwardsYVariable =
            CompressedEdwardsYVariable::constant(self, CompressedEdwardsY(DUMMY_PUBLIC_KEY));
        let signature = EDDSASignatureVariable::constant(
            self,
            EDDSASignatureVariableValue {
                r: CompressedEdwardsY(DUMMY_SIGNATURE[0..32].try_into().unwrap()),
                s: BigUint::from_bytes_le(&DUMMY_SIGNATURE[32..64]),
            },
        );
        let message = self.zero::<BytesVariable<MAX_MSG_BYTE_LENGTH>>();
        let dummy_msg_byte_length = self.constant::<U32Variable>(DUMMY_MSG_LENGTH_BYTES);

        (pub_key, signature, message, dummy_msg_byte_length)
    }

    pub fn curta_eddsa_verify_sigs_conditional<
        const MAX_MSG_LENGTH_BYTES: usize,
        const NUM_SIGS: usize,
    >(
        &mut self,
        is_active: ArrayVariable<BoolVariable, NUM_SIGS>,
        message_byte_lengths: Option<ArrayVariable<U32Variable, NUM_SIGS>>,
        messages: ArrayVariable<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureVariable<Ed25519ScalarField>, NUM_SIGS>,
        pubkeys: ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>,
    ) {
        let (dummy_pub_key, dummy_sig, dummy_msg, dummy_msg_byte_length) =
            self.get_dummy_variables::<MAX_MSG_LENGTH_BYTES>();

        let mut msg_vec = Vec::new();
        let mut msg_len_vec = Vec::new();
        let mut sig_vec = Vec::new();
        let mut pub_key_vec = Vec::new();

        for i in 0..NUM_SIGS {
            msg_vec.push(self.select(is_active[i], messages[i], dummy_msg));
            if let Some(ref msg_lens) = message_byte_lengths {
                msg_len_vec.push(self.select(is_active[i], msg_lens[i], dummy_msg_byte_length));
            }
            sig_vec.push(self.select(is_active[i], signatures[i].clone(), dummy_sig.clone()));
            pub_key_vec.push(self.select(is_active[i], pubkeys[i].clone(), dummy_pub_key.clone()));
        }

        let msg_array =
            ArrayVariable::<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>::from(msg_vec);
        let msg_len_array = ArrayVariable::<U32Variable, NUM_SIGS>::from(msg_len_vec);
        let sig_array =
            ArrayVariable::<EDDSASignatureVariable<Ed25519ScalarField>, NUM_SIGS>::from(sig_vec);
        let pub_key_array =
            ArrayVariable::<CompressedEdwardsYVariable, NUM_SIGS>::from(pub_key_vec);

        self.curta_eddsa_verify_sigs(
            msg_array,
            if message_byte_lengths.is_none() {
                None
            } else {
                Some(msg_len_array)
            },
            sig_array,
            pub_key_array,
        );
    }

    pub fn curta_eddsa_verify_sigs<
        // Maximum length of a signed message in bytes.
        const MAX_MSG_LENGTH_BYTES: usize,
        const NUM_SIGS: usize,
    >(
        &mut self,
        messages: ArrayVariable<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>,
        message_byte_lengths: Option<ArrayVariable<U32Variable, NUM_SIGS>>,
        signatures: ArrayVariable<EDDSASignatureVariable<Ed25519ScalarField>, NUM_SIGS>,
        pubkeys: ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>,
    ) {
        assert!(NUM_SIGS > 0 && NUM_SIGS <= MAX_NUM_SIGS);

        let (generator_x, generator_y) = Ed25519Parameters::generator();
        let generator_affine = AffinePoint::new(generator_x, generator_y);
        let generator_var = AffinePointVariable::constant(self, generator_affine);

        for i in 0..NUM_SIGS {
            // Create a new BytesVariable that will contain the message to be hashed.
            // The hashed message is a concatenation of sigR, pk, and msg.
            let mut message_bytes = Vec::new();
            message_bytes.extend(signatures[i].r.0.as_bytes());
            message_bytes.extend(pubkeys[i].0.as_bytes());
            message_bytes.extend(messages[i].0);

            // Can be unsafe since the message bytes elements were all retrieved from BytesVariables.
            let digest: BytesVariable<64>;
            if let Some(ref msg_lens) = message_byte_lengths {
                let const_64 = U32Variable::constant(self, 64);
                let message_to_hash_len = self.add(msg_lens[i], const_64);
                digest = self.curta_sha512_variable(&message_bytes, message_to_hash_len);
            } else {
                digest = self.curta_sha512(&message_bytes);
            }

            // digest is in big endian byte / big endian bit
            let h_biguint = biguint_from_bytes_variable(self, digest);
            let h_scalar = self.api.reduce::<Ed25519ScalarField>(&h_biguint);

            let p1 = self.curta_scalar_mul(signatures[i].s.clone(), generator_var.clone());
            let pubkey_affine = self.curta_decompress(pubkeys[i].clone());
            self.curta_is_valid(pubkey_affine.clone());
            let mut p2 = self.curta_scalar_mul(h_scalar, pubkey_affine);
            let sigr_affine = self.curta_decompress(signatures[i].r.clone());
            self.curta_is_valid(sigr_affine.clone());
            p2 = self.curta_add(sigr_affine, p2);

            self.assert_is_equal(p1, p2);
        }
    }
}

#[cfg(test)]
mod tests {
    use curta::chip::ec::edwards::ed25519::params::Ed25519ScalarField;
    use curve25519_dalek::edwards::CompressedEdwardsY;
    use ed25519_dalek::{Signer, SigningKey};
    use num::BigUint;
    use rand::rngs::OsRng;
    use rand::Rng;

    use crate::frontend::curta::ec::point::CompressedEdwardsYVariable;
    use crate::frontend::ecc::curve25519::ed25519::eddsa::{
        EDDSASignatureVariable, EDDSASignatureVariableValue,
    };
    use crate::prelude::{ArrayVariable, BytesVariable, DefaultBuilder, U32Variable};
    use crate::utils;

    #[test]
    fn test_curta_eddsa_verify_sigs_constant_msg_len() {
        test_curta_eddsa_verify_sigs(false);
    }

    #[test]
    fn test_curta_eddsa_verify_sigs_variable_msg_len() {
        test_curta_eddsa_verify_sigs(true);
    }

    fn test_curta_eddsa_verify_sigs(variable_msg_len: bool) {
        utils::setup_logger();

        const MAX_MSG_LEN_BYTES: usize = 192;
        const NUM_SIGS: usize = 1;
        type FF = Ed25519ScalarField;
        let mut builder = DefaultBuilder::new();

        let pkeys = builder.read::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>();
        let signatures = builder.read::<ArrayVariable<EDDSASignatureVariable<FF>, NUM_SIGS>>();
        let messages = builder.read::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>();
        if variable_msg_len {
            let message_lens = builder.read::<ArrayVariable<U32Variable, NUM_SIGS>>();
            builder.curta_eddsa_verify_sigs(messages, Some(message_lens), signatures, pkeys);
        } else {
            builder.curta_eddsa_verify_sigs(messages, None, signatures, pkeys);
        }

        let circuit = builder.build();

        // Generate random messages and private keys
        let mut test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]> = Vec::new();
        let mut test_message_lens = Vec::new();
        let mut test_pub_keys = Vec::new();
        let mut test_signatures = Vec::new();

        let mut csprng = OsRng;
        for _i in 0..NUM_SIGS {
            // Generate random length
            let msg_len: u32;
            if variable_msg_len {
                msg_len = rand::thread_rng().gen_range(1..MAX_MSG_LEN_BYTES) as u32;
                test_message_lens.push(msg_len);
            } else {
                msg_len = MAX_MSG_LEN_BYTES as u32;
            }
            let mut test_message = Vec::new();
            for _ in 0..msg_len {
                test_message.push(rand::thread_rng().gen_range(0..255));
            }

            let test_signing_key = SigningKey::generate(&mut csprng);
            let test_pub_key = test_signing_key.verifying_key();
            let test_signature = test_signing_key.sign(&test_message);

            test_message.resize(MAX_MSG_LEN_BYTES, 0);
            test_messages.push(test_message.try_into().unwrap());
            test_pub_keys.push(CompressedEdwardsY(test_pub_key.to_bytes()));
            test_signatures.push(EDDSASignatureVariableValue {
                r: CompressedEdwardsY(*test_signature.r_bytes()),
                s: BigUint::from_bytes_le(test_signature.s_bytes()),
            });
        }

        let mut input = circuit.input();
        input.write::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>(test_pub_keys.clone());
        input.write::<ArrayVariable<EDDSASignatureVariable<FF>, NUM_SIGS>>(test_signatures.clone());
        input.write::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>(
            test_messages.clone(),
        );
        if variable_msg_len {
            input.write::<ArrayVariable<U32Variable, NUM_SIGS>>(test_message_lens.clone());
        }

        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
