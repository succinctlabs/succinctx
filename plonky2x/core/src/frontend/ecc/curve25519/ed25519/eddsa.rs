use core::fmt::Debug;

use ::curta::chip::ec::edwards::ed25519::params::Ed25519ScalarField;
use ::curta::chip::field::parameters::FieldParameters;
use array_macro::array;
use curta::chip::ec::edwards::ed25519::params::Ed25519Parameters;
use curta::chip::ec::edwards::EdwardsParameters;
use curta::chip::ec::point::AffinePoint;
use curve25519_dalek::edwards::CompressedEdwardsY;
use ethers::types::{U256, U512};
use plonky2::hash::hash_types::RichField;

use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::uint::num::biguint::biguint_from_bytes_variable;
use crate::frontend::uint::uint512::U512Variable;
use crate::prelude::{
    ArrayVariable, BoolVariable, BytesVariable, CircuitBuilder, CircuitVariable, PlonkParameters,
    U256Variable, U32Variable, Variable,
};

#[derive(Clone, Debug, CircuitVariable)]
pub struct EDDSASignatureVariable {
    pub r: CompressedEdwardsYVariable,
    pub s: U256Variable,
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
        EDDSASignatureVariable,
        BytesVariable<MAX_MSG_BYTE_LENGTH>,
        U32Variable,
    ) {
        let pub_key: CompressedEdwardsYVariable =
            CompressedEdwardsYVariable::constant(self, CompressedEdwardsY(DUMMY_PUBLIC_KEY));
        let signature = EDDSASignatureVariable::constant(
            self,
            EDDSASignatureVariableValue {
                r: CompressedEdwardsY(DUMMY_SIGNATURE[0..32].try_into().unwrap()),
                s: U256::from_little_endian(&DUMMY_SIGNATURE[32..64]),
            },
        );
        let message = self.zero::<BytesVariable<MAX_MSG_BYTE_LENGTH>>();
        let dummy_msg_byte_length = self.constant::<U32Variable>(DUMMY_MSG_LENGTH_BYTES);

        (pub_key, signature, message, dummy_msg_byte_length)
    }

    /// This function will verify a set of eddsa signatures.  It also contains a BoolVariable array
    /// bitmask ("is_active") that will specify which signatures should be verified.  If
    /// message_byte_lengths is None, then all the messages should have the length of
    /// MAX_MSG_LENGTH_BYTES.
    pub fn curta_eddsa_verify_sigs_conditional<
        const MAX_MSG_LENGTH_BYTES: usize,
        const NUM_SIGS: usize,
    >(
        &mut self,
        is_active: ArrayVariable<BoolVariable, NUM_SIGS>,
        message_byte_lengths: Option<ArrayVariable<U32Variable, NUM_SIGS>>,
        messages: ArrayVariable<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureVariable, NUM_SIGS>,
        pubkeys: ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>,
    ) {
        assert!(NUM_SIGS > 0);
        assert!(is_active.len() == NUM_SIGS);
        assert!(messages.len() == NUM_SIGS);
        if let Some(ref msg_lens) = message_byte_lengths {
            assert!(msg_lens.len() == NUM_SIGS);
        }
        assert!(signatures.len() == NUM_SIGS);
        assert!(pubkeys.len() == NUM_SIGS);

        let max_msg_byte_length = self.constant::<U32Variable>(MAX_MSG_LENGTH_BYTES as u32);

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
            } else {
                msg_len_vec.push(self.select(
                    is_active[i],
                    max_msg_byte_length,
                    dummy_msg_byte_length,
                ))
            }
            sig_vec.push(self.select(is_active[i], signatures[i].clone(), dummy_sig.clone()));
            pub_key_vec.push(self.select(is_active[i], pubkeys[i].clone(), dummy_pub_key.clone()));
        }

        let msg_len_vec = ArrayVariable::<U32Variable, NUM_SIGS>::from(
            msg_len_vec.into_iter().collect::<Vec<_>>(),
        );
        let msg_array =
            ArrayVariable::<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>::from(msg_vec);
        let sig_array = ArrayVariable::<EDDSASignatureVariable, NUM_SIGS>::from(sig_vec);
        let pub_key_array =
            ArrayVariable::<CompressedEdwardsYVariable, NUM_SIGS>::from(pub_key_vec);

        self.curta_eddsa_verify_sigs(msg_array, Some(msg_len_vec), sig_array, pub_key_array);
    }

    /// This function will verify a set of eddsa signatures. If message_byte_lengths is None, then
    /// all the messages should have the length of MAX_MSG_LENGTH_BYTES.
    pub fn curta_eddsa_verify_sigs<
        // Maximum length of a signed message in bytes.
        const MAX_MSG_LENGTH_BYTES: usize,
        const NUM_SIGS: usize,
    >(
        &mut self,
        messages: ArrayVariable<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>,
        message_byte_lengths: Option<ArrayVariable<U32Variable, NUM_SIGS>>,
        signatures: ArrayVariable<EDDSASignatureVariable, NUM_SIGS>,
        pubkeys: ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>,
    ) {
        assert!(NUM_SIGS > 0);
        assert!(messages.len() == NUM_SIGS);
        if let Some(ref msg_lens) = message_byte_lengths {
            assert!(msg_lens.len() == NUM_SIGS);
        }
        assert!(signatures.len() == NUM_SIGS);
        assert!(pubkeys.len() == NUM_SIGS);

        let (generator_x, generator_y) = Ed25519Parameters::generator();
        let generator_affine = AffinePoint::new(generator_x, generator_y);
        let generator_var = AffinePointVariable::constant(self, generator_affine);

        let scalar_modulus_value =
            U512::from_little_endian(&Ed25519ScalarField::modulus().to_bytes_le());
        let scalar_modulus = self.constant::<U512Variable>(scalar_modulus_value);
        let scalar_mod_256_value =
            U256::from_little_endian(&Ed25519ScalarField::modulus().to_bytes_le());
        let scalar_mod_256 = self.constant::<U256Variable>(scalar_mod_256_value);

        for i in 0..NUM_SIGS {
            // Create a new BytesVariable that will contain the message to be hashed.
            // The hashed message is a concatenation of sigR, pk, and msg.
            let mut message_bytes = Vec::new();
            message_bytes.extend(signatures[i].r.0.as_bytes());
            message_bytes.extend(pubkeys[i].0.as_bytes());
            message_bytes.extend(messages[i].0);

            let digest: BytesVariable<64>;
            if let Some(ref msg_lens) = message_byte_lengths {
                let const_64 = U32Variable::constant(self, 64);
                let message_to_hash_len = self.add(msg_lens[i], const_64);
                digest = self.curta_sha512_variable(&message_bytes, message_to_hash_len);
            } else {
                digest = self.curta_sha512(&message_bytes);
            }

            let h_limbs = biguint_from_bytes_variable(self, digest)
                .limbs
                .into_iter()
                .map(|x| x.target)
                .collect::<Vec<_>>();
            let h_int = U512Variable::from_targets(&h_limbs);
            let h_scalar_512_limbs = self.rem(h_int, scalar_modulus).limbs;
            let h_scalar = U256Variable {
                limbs: array![i => h_scalar_512_limbs[i]; 8],
            };

            let s = signatures[i].s;
            // Assert that s is less than the scalar modulus.
            let s_lt_scalar_mod = self.lt(s, scalar_mod_256);
            let true_val = self.constant::<BoolVariable>(true);
            self.assert_is_equal(s_lt_scalar_mod, true_val);
            let p1 = self.curta_25519_scalar_mul(s, generator_var.clone());
            let pubkey_affine = self.curta_25519_decompress(pubkeys[i].clone());
            self.curta_25519_is_valid(pubkey_affine.clone());
            let mut p2 = self.curta_25519_scalar_mul(h_scalar, pubkey_affine);
            let sigr_affine = self.curta_25519_decompress(signatures[i].r.clone());
            // Assert that `r` is canonical, namely, r.y is a valid field element.
            self.curta_25519_is_valid(sigr_affine.clone());
            p2 = self.curta_25519_add(sigr_affine, p2);

            self.assert_is_equal(p1, p2);
        }
    }
}

#[cfg(test)]
mod tests {
    use curve25519_dalek::edwards::CompressedEdwardsY;
    use ed25519_dalek::{Signer, SigningKey};
    use ethers::types::U256;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use rand::rngs::OsRng;
    use rand::Rng;

    use crate::frontend::curta::ec::point::CompressedEdwardsYVariable;
    use crate::frontend::ecc::curve25519::ed25519::eddsa::{
        EDDSASignatureVariable, EDDSASignatureVariableValue,
    };
    use crate::prelude::{ArrayVariable, BoolVariable, BytesVariable, DefaultBuilder, U32Variable};
    use crate::utils;

    const MAX_MSG_LEN_BYTES: usize = 174;
    const NUM_SIGS: usize = 3;

    fn test_curta_eddsa_verify_sigs(
        test_pub_keys: Vec<CompressedEdwardsY>,
        test_signatures: Vec<EDDSASignatureVariableValue<GoldilocksField>>,
        test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]>,
        test_message_lens: Vec<u32>,
        variable_msg_len: bool,
    ) {
        utils::setup_logger();

        let mut builder = DefaultBuilder::new();

        let pkeys = builder.read::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>();
        let signatures = builder.read::<ArrayVariable<EDDSASignatureVariable, NUM_SIGS>>();
        let messages = builder.read::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>();
        if variable_msg_len {
            let message_lens = builder.read::<ArrayVariable<U32Variable, NUM_SIGS>>();
            builder.curta_eddsa_verify_sigs(messages, Some(message_lens), signatures, pkeys);
        } else {
            builder.curta_eddsa_verify_sigs(messages, None, signatures, pkeys);
        }

        let circuit = builder.build();

        let mut input = circuit.input();
        input.write::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>(test_pub_keys);
        input.write::<ArrayVariable<EDDSASignatureVariable, NUM_SIGS>>(test_signatures);
        input.write::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>(
            test_messages.to_vec(),
        );
        if variable_msg_len {
            input.write::<ArrayVariable<U32Variable, NUM_SIGS>>(test_message_lens);
        }

        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_curta_eddsa_verify_sigs_constant_msg_len() {
        // Generate random messages and private keys
        let mut test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]> = Vec::new();
        let test_message_lens = Vec::new();
        let mut test_pub_keys = Vec::new();
        let mut test_signatures = Vec::new();

        let mut csprng = OsRng;
        for _i in 0..NUM_SIGS {
            // Generate random length
            let msg_len = MAX_MSG_LEN_BYTES as u32;
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
                s: U256::from_little_endian(test_signature.s_bytes()),
            });
        }

        test_curta_eddsa_verify_sigs(
            test_pub_keys,
            test_signatures,
            test_messages,
            test_message_lens,
            false,
        );
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_curta_eddsa_verify_sigs_variable_msg_len() {
        // Generate random messages and private keys
        let mut test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]> = Vec::new();
        let mut test_message_lens = Vec::new();
        let mut test_pub_keys = Vec::new();
        let mut test_signatures = Vec::new();

        let mut csprng = OsRng;
        for _i in 0..NUM_SIGS {
            // Generate random length
            let msg_len = rand::thread_rng().gen_range(1..MAX_MSG_LEN_BYTES) as u32;
            test_message_lens.push(msg_len);
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
                s: U256::from_little_endian(test_signature.s_bytes()),
            });
        }

        test_curta_eddsa_verify_sigs(
            test_pub_keys,
            test_signatures,
            test_messages,
            test_message_lens,
            true,
        );
    }

    #[test]
    #[should_panic]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_curta_eddsa_verify_sigs_failure() {
        // Generate random messages and private keys
        let mut test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]> = Vec::new();
        let test_message_lens = Vec::new();
        let mut test_pub_keys = Vec::new();
        let mut test_signatures = Vec::new();

        let mut csprng = OsRng;
        for i in 0..NUM_SIGS {
            // Generate random length
            let msg_len = MAX_MSG_LEN_BYTES as u32;
            let mut test_message = Vec::new();
            for _ in 0..msg_len {
                test_message.push(rand::thread_rng().gen_range(0..255));
            }

            let test_signing_key = SigningKey::generate(&mut csprng);
            let test_pub_key = test_signing_key.verifying_key();
            let test_signature = test_signing_key.sign(&test_message);

            test_message.resize(MAX_MSG_LEN_BYTES, 0);
            if i == 0 {
                test_message[0] = test_message[0].wrapping_add(1);
            }
            test_messages.push(test_message.try_into().unwrap());
            test_pub_keys.push(CompressedEdwardsY(test_pub_key.to_bytes()));
            test_signatures.push(EDDSASignatureVariableValue {
                r: CompressedEdwardsY(*test_signature.r_bytes()),
                s: U256::from_little_endian(test_signature.s_bytes()),
            });
        }

        test_curta_eddsa_verify_sigs(
            test_pub_keys,
            test_signatures,
            test_messages,
            test_message_lens,
            false,
        );
    }

    fn test_curta_eddsa_verify_sigs_conditional(
        test_is_active: Vec<bool>,
        test_pub_keys: Vec<CompressedEdwardsY>,
        test_signatures: Vec<EDDSASignatureVariableValue<GoldilocksField>>,
        test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]>,
        test_message_lens: Vec<u32>,
        variable_msg_len: bool,
    ) {
        utils::setup_logger();

        let mut builder = DefaultBuilder::new();

        let is_active = builder.read::<ArrayVariable<BoolVariable, NUM_SIGS>>();
        let pkeys = builder.read::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>();
        let signatures = builder.read::<ArrayVariable<EDDSASignatureVariable, NUM_SIGS>>();
        let messages = builder.read::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>();
        if variable_msg_len {
            let message_lens = builder.read::<ArrayVariable<U32Variable, NUM_SIGS>>();
            builder.curta_eddsa_verify_sigs_conditional(
                is_active,
                Some(message_lens),
                messages,
                signatures,
                pkeys,
            );
        } else {
            builder
                .curta_eddsa_verify_sigs_conditional(is_active, None, messages, signatures, pkeys);
        }

        let circuit = builder.build();

        let mut input = circuit.input();
        input.write::<ArrayVariable<BoolVariable, NUM_SIGS>>(test_is_active);
        input.write::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>(test_pub_keys);
        input.write::<ArrayVariable<EDDSASignatureVariable, NUM_SIGS>>(test_signatures);
        input.write::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>(
            test_messages.to_vec(),
        );
        if variable_msg_len {
            input.write::<ArrayVariable<U32Variable, NUM_SIGS>>(test_message_lens);
        }

        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_curta_eddsa_verify_sigs_constant_msg_len_conditional() {
        // Generate random messages and private keys
        let mut test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]> = Vec::new();
        let test_message_lens = Vec::new();
        let mut test_is_active = Vec::new();
        let mut test_pub_keys = Vec::new();
        let mut test_signatures = Vec::new();

        let mut csprng = OsRng;
        for _i in 0..NUM_SIGS {
            // Generate random length
            let msg_len = MAX_MSG_LEN_BYTES as u32;
            let mut test_message = Vec::new();
            for _ in 0..msg_len {
                test_message.push(rand::thread_rng().gen_range(0..255));
            }

            let test_signing_key = SigningKey::generate(&mut csprng);
            let test_pub_key = test_signing_key.verifying_key();
            let test_signature = test_signing_key.sign(&test_message);

            test_message.resize(MAX_MSG_LEN_BYTES, 0);
            test_messages.push(test_message.try_into().unwrap());
            test_is_active.push(true);
            test_pub_keys.push(CompressedEdwardsY(test_pub_key.to_bytes()));
            test_signatures.push(EDDSASignatureVariableValue {
                r: CompressedEdwardsY(*test_signature.r_bytes()),
                s: U256::from_little_endian(test_signature.s_bytes()),
            });
        }

        test_curta_eddsa_verify_sigs_conditional(
            test_is_active,
            test_pub_keys,
            test_signatures,
            test_messages,
            test_message_lens,
            false,
        );
    }
}
