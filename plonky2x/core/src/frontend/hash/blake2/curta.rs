use core::marker::PhantomData;

use ::curta::machine::hash::blake::blake2b::BLAKE2B;
use curta::chip::register::array::ArrayRegister;
use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::uint::operations::instruction::UintInstruction;
use curta::chip::AirParameters;
use curta::machine::bytes::builder::BytesBuilder;
use curta::machine::hash::blake::blake2b;
use curta::machine::hash::blake::blake2b::builder::BlakeBuilder;
use curta::machine::hash::blake::blake2b::pure::BLAKE2BPure;
use curta::machine::hash::blake::blake2b::utils::BLAKE2BUtil;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hash::curta::accelerator::HashAccelerator;
use crate::frontend::hash::curta::request::HashRequest;
use crate::frontend::hash::curta::Hash;
use crate::frontend::vars::{Bytes32Variable, EvmVariable};
use crate::prelude::*;

pub type BLAKE2BAccelerator = HashAccelerator<U64Variable, 4>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BLAKE2BAirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for BLAKE2BAirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 1527;
    const EXTENDED_COLUMNS: usize = 708;
}

impl<L: PlonkParameters<D>, const D: usize> Hash<L, D, 96, true, 4> for BLAKE2B {
    type IntVariable = U64Variable;
    type DigestVariable = Bytes32Variable;

    type AirParameters = BLAKE2BAirParameters<L, D>;
    type AirInstruction = UintInstruction;

    fn pad_circuit(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
    ) -> Vec<Self::IntVariable> {
        let num_pad_bytes = 128 - (input.len() % 128);

        let mut padded_message = Vec::new();
        padded_message.extend_from_slice(input);

        for _ in 0..num_pad_bytes {
            padded_message.push(builder.zero());
        }

        padded_message
            .chunks_exact(8)
            .map(|bytes| {
                let mut bytes_copy = Vec::new();
                bytes_copy.extend_from_slice(bytes);
                bytes_copy.reverse();
                U64Variable::decode(builder, &bytes_copy)
            })
            .collect()
    }

    fn pad_circuit_variable_length(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
        _: U32Variable,
    ) -> Vec<Self::IntVariable> {
        Self::pad_circuit(builder, input)
    }

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <Self::IntRegister as curta::chip::register::Register>::Value<Variable>,
    ) -> Self::IntVariable {
        let low_limbs = &value[0..4];
        let high_limbs = &value[4..8];
        let mut acc_low = builder.zero::<Variable>();
        let mut acc_high = builder.zero::<Variable>();
        for (i, (low_byte, high_byte)) in low_limbs.iter().zip(high_limbs).enumerate() {
            let two_i = builder.constant::<Variable>(L::Field::from_canonical_u32(1 << (8 * i)));
            let two_i_low_byte = builder.mul(two_i, *low_byte);
            let two_i_high_byte = builder.mul(two_i, *high_byte);
            acc_low = builder.add(acc_low, two_i_low_byte);
            acc_high = builder.add(acc_high, two_i_high_byte);
        }
        let low_limb = U32Variable::from_variables_unsafe(&[acc_low]);
        let high_limb = U32Variable::from_variables_unsafe(&[acc_high]);
        U64Variable {
            limbs: [low_limb, high_limb],
        }
    }

    fn digest_to_array(
        builder: &mut CircuitBuilder<L, D>,
        digest: Self::DigestVariable,
    ) -> [Self::IntVariable; 4] {
        digest
            .as_bytes()
            .chunks_exact(8)
            .map(|x| {
                let mut x_copy = Vec::new();
                x_copy.extend_from_slice(x);
                x_copy.reverse();
                U64Variable::decode(builder, &x_copy)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn hash(message: Vec<u8>) -> [Self::Integer; 4] {
        let mut num_message_chunks = (message.len() as u64 / 128) + 1;
        if num_message_chunks % 128 == 0 {
            num_message_chunks -= 1;
        }

        let padded_chunks = BLAKE2BUtil::pad(&message, num_message_chunks);

        // Initialize the hash state.
        let mut state = blake2b::IV;
        let mut t = 0;
        for (i, chunk) in padded_chunks.chunks_exact(128).enumerate() {
            let at_last_chunk = i as u64 == num_message_chunks - 1;
            if at_last_chunk {
                t = message.len() as u64;
            } else {
                t += 128;
            }
            <curta::machine::hash::blake::blake2b::BLAKE2B as BLAKE2BPure>::compress(
                chunk,
                &mut state,
                t,
                at_last_chunk,
            );
        }

        // Write the digest to the output stream.
        let mut digest: [u64; 4] = Default::default();
        digest.copy_from_slice(&state[0..4]);
        digest
    }

    fn hash_circuit(
        builder: &mut BytesBuilder<Self::AirParameters>,
        padded_chunks: &[ArrayRegister<Self::IntRegister>],
        t_values: &Option<ArrayRegister<Self::IntRegister>>,
        end_bits: &ArrayRegister<BitRegister>,
        digest_bits: &ArrayRegister<BitRegister>,
        digest_indices: &ArrayRegister<ElementRegister>,
        num_messages: &ElementRegister,
    ) -> Vec<Self::DigestRegister> {
        builder.blake2b::<BLAKE2B>(
            padded_chunks,
            &t_values.unwrap(),
            end_bits,
            digest_bits,
            digest_indices,
            num_messages,
        )
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn curta_blake2b(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        if self.blake2b_accelerator.is_none() {
            self.blake2b_accelerator = Some(BLAKE2BAccelerator {
                hash_requests: Vec::new(),
                hash_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_array = BLAKE2B::digest_to_array(self, digest);
        let accelerator = self
            .blake2b_accelerator
            .as_mut()
            .expect("blake2b accelerator should exist");
        accelerator
            .hash_requests
            .push(HashRequest::Fixed(input.to_vec()));
        accelerator.hash_responses.push(digest_array);

        digest
    }

    pub fn curta_blake2b_variable(
        &mut self,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> Bytes32Variable {
        let last_chunk = self.compute_blake2b_last_chunk_index(length);
        if self.blake2b_accelerator.is_none() {
            self.blake2b_accelerator = Some(BLAKE2BAccelerator {
                hash_requests: Vec::new(),
                hash_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_array = BLAKE2B::digest_to_array(self, digest);

        let accelerator = self
            .blake2b_accelerator
            .as_mut()
            .expect("blake2b accelerator should exist");
        accelerator
            .hash_requests
            .push(HashRequest::Variable(input.to_vec(), length, last_chunk));
        accelerator.hash_responses.push(digest_array);

        digest
    }

    pub fn compute_blake2b_last_chunk_index(
        &mut self,
        input_byte_length: U32Variable,
    ) -> U32Variable {
        let chunk_size = self.constant::<U32Variable>(128);
        self.div(input_byte_length, chunk_size)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use itertools::Itertools;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{BytesVariable, CircuitBuilder, U32Variable};
    use crate::utils::bytes32;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_blake2b_curta_empty_string() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.zero::<U32Variable>();
        let result = builder.curta_blake2b_variable(&[], zero);

        let expected_digest =
            bytes32!("0x0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        builder.assert_is_equal(result, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_blake2b_curta_long_string() {
        let _ = env_logger::builder().is_test(true).try_init();

        let msg_hex = "00f43f3ef4c05d1aca645d7b2b59af99d65661810b8a724818052db75e04afb60ea210002f9cac87493604cb5fff6644ea17c3b1817d243bc5a0aa6f0d11ab3df46f37b9adbf1ff3a446807e7a9ebc77647776b8bbda37dcf2f4f34ca7ba7bf4c7babfbe080642414245b501032c000000b7870a0500000000360b79058f3b331fbbb10d38a2e309517e24cc12094d0a5a7c9faa592884e9621aecff0224bc1a857a0bacadf4455e2c5b39684d2d5879b108c98315f6a14504348846c6deed3addcba24fc3af531d59f31c87bc454bf6f1d73eadaf2d22d60c05424142450101eead41c1266af7bc7becf961dcb93f3691642c9b6d50aeb65b92528b99c675608f2095a296ed52aa433c1bfed56e8546dae03b61cb59643a9cb39f82618f958b00041000000000000000000000000000000000000000000000000000000000000000008101a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e07918a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e079180400";
        let msg_bytes = hex::decode(msg_hex).unwrap();
        const MSG_LEN: usize = 423;
        assert!(msg_bytes.len() == MSG_LEN);

        let mut builder = CircuitBuilder::<L, D>::new();

        let msg = builder.constant::<BytesVariable<MSG_LEN>>(msg_bytes.clone().try_into().unwrap());
        let bytes_length = builder.constant::<U32Variable>(msg_bytes.len() as u32);
        let result = builder.curta_blake2b_variable(&msg.0, bytes_length);

        let expected_digest =
            bytes32!("7c38fc8356aa20394c7f538e3cee3f924e6d9252494c8138d1a6aabfc253118f");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        builder.assert_is_equal(result, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_blake2b_curta_multiple_hashes_variable() {
        let _ = env_logger::builder().is_test(true).try_init();
        const MAX_MSG_SIZE: usize = 960;

        let mut builder = CircuitBuilder::<L, D>::new();

        let msgs = [
            "00f43f3ef4c05d1aca645d7b2b59af99d65661810b8a724818052db75e04afb60ea210002f9cac87493604cb5fff6644ea17c3b1817d243bc5a0aa6f0d11ab3df46f37b9adbf1ff3a446807e7a9ebc77647776b8bbda37dcf2f4f34ca7ba7bf4c7babfbe080642414245b501032c000000b7870a0500000000360b79058f3b331fbbb10d38a2e309517e24cc12094d0a5a7c9faa592884e9621aecff0224bc1a857a0bacadf4455e2c5b39684d2d5879b108c98315f6a14504348846c6deed3addcba24fc3af531d59f31c87bc454bf6f1d73eadaf2d22d60c05424142450101eead41c1266af7bc7becf961dcb93f3691642c9b6d50aeb65b92528b99c675608f2095a296ed52aa433c1bfed56e8546dae03b61cb59643a9cb39f82618f958b00041000000000000000000000000000000000000000000000000000000000000000008101a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e07918a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e079180400",
            "39285734897537894674835698460198237adce984eda487459893754091",
        ];
        let mut msg_bytes = msgs.map(|x| hex::decode(x).unwrap());

        let mut results = Vec::new();
        for msg in msg_bytes.iter_mut() {
            let msg_len = builder.constant::<U32Variable>(msg.len().try_into().unwrap());
            msg.resize(MAX_MSG_SIZE, 0);
            let msg_var =
                builder.constant::<BytesVariable<MAX_MSG_SIZE>>(msg.clone().try_into().unwrap());
            results.push(builder.curta_blake2b_variable(&msg_var.0, msg_len));
        }

        let expected_digests = [
            bytes32!("7c38fc8356aa20394c7f538e3cee3f924e6d9252494c8138d1a6aabfc253118f"),
            bytes32!("7cd6b73d53b4bd7fef48f0d45782caac149615387c13891b0f3665dcfa50a4c0"),
        ];

        for (expected_digest, result) in expected_digests.iter().zip_eq(results.iter()) {
            let expected_digest_var = builder.constant::<Bytes32Variable>(*expected_digest);
            builder.assert_is_equal(*result, expected_digest_var);
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_blake2b_curta_multiple_hashes_fixed() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut builder = CircuitBuilder::<L, D>::new();

        let msgs = [
            "00f43f3ef4c05d1aca645d7b2b59af99d65661810b8a724818052db75e04afb60ea210002f9cac87493604cb5fff6644ea17c3b1817d243bc5a0aa6f0d11ab3df46f37b9adbf1ff3a446807e7a9ebc77647776b8bbda37dcf2f4f34ca7ba7bf4c7babfbe080642414245b501032c000000b7870a0500000000360b79058f3b331fbbb10d38a2e309517e24cc12094d0a5a7c9faa592884e9621aecff0224bc1a857a0bacadf4455e2c5b39684d2d5879b108c98315f6a14504348846c6deed3addcba24fc3af531d59f31c87bc454bf6f1d73eadaf2d22d60c05424142450101eead41c1266af7bc7becf961dcb93f3691642c9b6d50aeb65b92528b99c675608f2095a296ed52aa433c1bfed56e8546dae03b61cb59643a9cb39f82618f958b00041000000000000000000000000000000000000000000000000000000000000000008101a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e07918a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e079180400",
            "39285734897537894674835698460198237adce984eda487459893754091",
        ];

        let msg_bytes = msgs.map(|x| hex::decode(x).unwrap());
        let mut results = Vec::new();

        const MSG_LEN_1: usize = 423;
        let msg_var =
            builder.constant::<BytesVariable<MSG_LEN_1>>(msg_bytes[0].clone().try_into().unwrap());
        results.push(builder.curta_blake2b(&msg_var.0));

        const MSG_LEN_2: usize = 30;
        let msg_var =
            builder.constant::<BytesVariable<MSG_LEN_2>>(msg_bytes[1].clone().try_into().unwrap());
        results.push(builder.curta_blake2b(&msg_var.0));

        let expected_digests = [
            bytes32!("7c38fc8356aa20394c7f538e3cee3f924e6d9252494c8138d1a6aabfc253118f"),
            bytes32!("7cd6b73d53b4bd7fef48f0d45782caac149615387c13891b0f3665dcfa50a4c0"),
        ];

        for (expected_digest, result) in expected_digests.iter().zip_eq(results.iter()) {
            let expected_digest_var = builder.constant::<Bytes32Variable>(*expected_digest);
            builder.assert_is_equal(*result, expected_digest_var);
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }
}
