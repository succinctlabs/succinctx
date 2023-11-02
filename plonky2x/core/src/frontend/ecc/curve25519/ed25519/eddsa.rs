use core::fmt::Debug;

use ::curta::chip::ec::edwards::ed25519::params::Ed25519ScalarField;
use ::curta::chip::field::parameters::FieldParameters;
use curta::chip::ec::edwards::ed25519::params::Ed25519Parameters;
use curta::chip::ec::edwards::EdwardsParameters;
use curta::chip::ec::point::AffinePoint;
use plonky2::hash::hash_types::RichField;

use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::num::biguint::biguint_from_bytes_variable;
use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::prelude::{
    ArrayVariable, BytesVariable, CircuitBuilder, CircuitVariable, PlonkParameters, U32Variable,
    Variable,
};

const MAX_NUM_SIGS: usize = 256;

#[derive(Clone, Debug, CircuitVariable)]
pub struct EDDSASignatureVariable<FF: FieldParameters> {
    pub r: CompressedEdwardsYVariable,
    pub s: NonNativeTarget<FF>,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn curta_eddsa_verify_sigs_constant_msg_len<
        const MSG_LENGTH: usize,
        const NUM_SIGS: usize,
    >(
        &mut self,
        messages: ArrayVariable<BytesVariable<MSG_LENGTH>, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureVariable<Ed25519ScalarField>, NUM_SIGS>,
        pubkeys: ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>,
    ) {
        self.curta_eddsa_verify_sigs(messages, None, signatures, pubkeys);
    }

    pub fn curta_eddsa_verify_sigs_variable_msg_len<
        // Maximum length of a signed message in bytes.
        const MAX_MSG_LENGTH_BYTES: usize,
        const NUM_SIGS: usize,
    >(
        &mut self,
        messages: ArrayVariable<BytesVariable<MAX_MSG_LENGTH_BYTES>, NUM_SIGS>,
        message_byte_lengths: ArrayVariable<U32Variable, NUM_SIGS>,
        signatures: ArrayVariable<EDDSASignatureVariable<Ed25519ScalarField>, NUM_SIGS>,
        pubkeys: ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>,
    ) {
        self.curta_eddsa_verify_sigs(messages, Some(message_byte_lengths), signatures, pubkeys);
    }

    fn curta_eddsa_verify_sigs<
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
    fn test_curta_eddsa_verify_variable_msg_len() {
        utils::setup_logger();

        const MAX_MSG_LEN_BYTES: usize = 192;
        const NUM_SIGS: usize = 10;
        type FF = Ed25519ScalarField;

        let mut builder = DefaultBuilder::new();

        let messages = builder.read::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>();
        let message_lens = builder.read::<ArrayVariable<U32Variable, NUM_SIGS>>();
        let pkeys = builder.read::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>();
        let signatures = builder.read::<ArrayVariable<EDDSASignatureVariable<FF>, NUM_SIGS>>();

        builder.curta_eddsa_verify_sigs_variable_msg_len(messages, message_lens, signatures, pkeys);

        let circuit = builder.build();

        // Generate random messages and private keys
        let mut test_messages: Vec<[u8; MAX_MSG_LEN_BYTES]> = Vec::new();
        let mut test_message_lens = Vec::new();
        let mut test_pub_keys = Vec::new();
        let mut test_signatures = Vec::new();

        let mut csprng = OsRng;
        for _i in 0..NUM_SIGS {
            // Generate random length
            let msg_len = rand::thread_rng().gen_range(1..MAX_MSG_LEN_BYTES);
            test_message_lens.push(msg_len as u32);
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
        input.write::<ArrayVariable<BytesVariable<MAX_MSG_LEN_BYTES>, NUM_SIGS>>(test_messages);
        input.write::<ArrayVariable<U32Variable, NUM_SIGS>>(test_message_lens);
        input.write::<ArrayVariable<CompressedEdwardsYVariable, NUM_SIGS>>(test_pub_keys);
        input.write::<ArrayVariable<EDDSASignatureVariable<FF>, NUM_SIGS>>(test_signatures);

        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
