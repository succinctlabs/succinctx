use core::marker::PhantomData;

use curta::chip::register::array::ArrayRegister;
use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::uint::operations::instruction::UintInstruction;
use curta::chip::uint::register::U64Register;
use curta::chip::AirParameters;
use curta::machine::bytes::builder::BytesBuilder;
use curta::machine::hash::sha::algorithm::SHAPure;
use curta::machine::hash::sha::builder::SHABuilder;
use curta::machine::hash::sha::sha512::SHA512;
use serde::{Deserialize, Serialize};

use crate::frontend::hash::curta::accelerator::HashAccelerator;
use crate::frontend::hash::curta::request::HashRequest;
use crate::frontend::hash::curta::Hash;
use crate::frontend::vars::EvmVariable;
use crate::prelude::*;

pub type SHA512Accelerator = HashAccelerator<U64Variable, 8>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SHA512AirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for SHA512AirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 1191;
    const EXTENDED_COLUMNS: usize = 654;
}

impl<L: PlonkParameters<D>, const D: usize> Hash<L, D, 80, false, 8> for SHA512 {
    type IntVariable = U64Variable;
    type DigestVariable = BytesVariable<64>;

    type AirParameters = SHA512AirParameters<L, D>;
    type AirInstruction = UintInstruction;

    fn pad_circuit(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_message_sha512(input);

        padded_bytes
            .chunks_exact(8)
            .map(|bytes| U64Variable::decode(builder, bytes))
            .collect()
    }

    fn pad_circuit_variable_length(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_sha512_variable_length(input, length);

        padded_bytes
            .chunks_exact(8)
            .map(|bytes| U64Variable::decode(builder, bytes))
            .collect()
    }

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <U64Register as curta::chip::register::Register>::Value<Variable>,
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
    ) -> [Self::IntVariable; 8] {
        digest
            .0
            .chunks_exact(8)
            .map(|x| U64Variable::decode(builder, x))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn hash(message: Vec<u8>) -> [Self::Integer; 8] {
        let mut current_state = SHA512::INITIAL_HASH;
        let padded_chunks = SHA512::pad(&message);
        for chunk in padded_chunks.chunks_exact(16) {
            let pre_processed = SHA512::pre_process(chunk);
            current_state = SHA512::process(current_state, &pre_processed);
        }
        current_state
    }

    fn hash_circuit(
        builder: &mut BytesBuilder<Self::AirParameters>,
        padded_chunks: &[ArrayRegister<Self::IntRegister>],
        _: &Option<ArrayRegister<Self::IntRegister>>,
        end_bits: &ArrayRegister<BitRegister>,
        digest_bits: &ArrayRegister<BitRegister>,
        digest_indices: &ArrayRegister<ElementRegister>,
        _: &ElementRegister,
    ) -> Vec<Self::DigestRegister> {
        builder.sha::<SHA512, 80>(padded_chunks, end_bits, digest_bits, *digest_indices)
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Executes a SHA512 hash on the given input of fixed size.
    pub fn curta_sha512(&mut self, input: &[ByteVariable]) -> BytesVariable<64> {
        if self.sha512_accelerator.is_none() {
            self.sha512_accelerator = Some(SHA512Accelerator {
                hash_requests: Vec::new(),
                hash_responses: Vec::new(),
            });
        }

        let digest = self.init::<BytesVariable<64>>();
        let digest_array = SHA512::digest_to_array(self, digest);
        let accelerator = self
            .sha512_accelerator
            .as_mut()
            .expect("sha512 accelerator should exist");
        accelerator
            .hash_requests
            .push(HashRequest::Fixed(input.to_vec()));
        accelerator.hash_responses.push(digest_array);

        digest
    }

    pub fn curta_sha512_variable(
        &mut self,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> BytesVariable<64> {
        let last_chunk = self.compute_sha512_last_chunk(length);

        if self.sha512_accelerator.is_none() {
            self.sha512_accelerator = Some(SHA512Accelerator {
                hash_requests: Vec::new(),
                hash_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<BytesVariable<64>>();
        let digest_array = SHA512::digest_to_array(self, digest);
        let accelerator = self
            .sha512_accelerator
            .as_mut()
            .expect("sha512 accelerator should exist");
        accelerator
            .hash_requests
            .push(HashRequest::Variable(input.to_vec(), length, last_chunk));
        accelerator.hash_responses.push(digest_array);

        digest
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::prelude::*;
    use crate::utils::hash::sha512;
    use crate::utils::setup_logger;

    fn test_sha512_fixed(msg: &[u8], expected_digest: [u8; 64]) {
        setup_logger();
        let mut builder = DefaultBuilder::new();
        let message = msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();
        let digest = builder.curta_sha512(&message);

        let expected_digest = builder.constant::<BytesVariable<64>>(expected_digest);
        builder.assert_is_equal(digest, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    fn test_sha512_variable_length(message: &[u8], input_length: u32, expected_digest: [u8; 64]) {
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let input_length = builder.constant::<U32Variable>(input_length);

        let message = message
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();
        let digest = builder.curta_sha512_variable(&message, input_length);

        let expected_digest = builder.constant::<BytesVariable<64>>(expected_digest);
        builder.assert_is_equal(digest, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_empty() {
        let msg = b"";
        let expected_digest = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";

        test_sha512_fixed(msg, bytes!(expected_digest));
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_small_msg() {
        let msg = b"plonky2";
        let expected_digest = "7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87";

        test_sha512_fixed(msg, bytes!(expected_digest));
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_large_msg() {
        let msg : Vec<u8> = bytes!("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273");
        let expected_digest = bytes!("4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39");

        test_sha512_fixed(&msg, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_empty_short() {
        let msg: Vec<u8> = vec![1; 128];
        let expected_digest = bytes!("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        test_sha512_variable_length(&msg, 0, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_empty_long() {
        let msg: Vec<u8> = vec![1; 256];
        let expected_digest = bytes!("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        test_sha512_variable_length(&msg, 0, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_large_message() {
        let mut msg : Vec<u8> = bytes!("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273");
        let len = msg.len() as u32;
        msg.resize(256, 1);
        let expected_digest = bytes!("4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39");

        test_sha512_variable_length(&msg, len, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_short_message_same_slice() {
        let mut msg: Vec<u8> = b"plonky2".to_vec();
        let len = msg.len() as u32;
        msg.resize(128, 1);
        let expected_digest = bytes!("7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87");

        test_sha512_variable_length(&msg, len, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_short_message_different_slice() {
        let mut msg: Vec<u8> = b"plonky2".to_vec();
        let len = msg.len() as u32;
        msg.resize(512, 1);
        let expected_digest = bytes!("7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87");

        test_sha512_variable_length(&msg, len, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_fixed_length() {
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let max_len = 300;

        let mut rng = rand::thread_rng();
        for i in 0..max_len {
            let message = (0..i).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();
            let expected_digest = sha512(&message);
            let message = message
                .iter()
                .map(|b| builder.constant::<ByteVariable>(*b))
                .collect::<Vec<_>>();
            let digest = builder.curta_sha512(&message);
            let expected_digest = builder.constant::<BytesVariable<64>>(expected_digest);
            builder.assert_is_equal(digest, expected_digest);
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_variable_length_random() {
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let max_number_of_chunks = 2;
        let total_message_length = 128 * max_number_of_chunks;
        let max_len = total_message_length - 8;

        let mut rng = thread_rng();
        let total_message = (0..total_message_length)
            .map(|_| rng.gen::<u8>())
            .collect::<Vec<_>>();
        let message = total_message
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();
        for i in 0..max_len {
            let expected_digest = sha512(&total_message[..i]);

            let length = builder.constant::<U32Variable>(i as u32);

            let digest = builder.curta_sha512_variable(&message, length);
            let expected_digest = builder.constant::<BytesVariable<64>>(expected_digest);
            builder.assert_is_equal(digest, expected_digest);
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_variable_length_max_size() {
        // This test checks that sha512_variable_pad works as intended, especially when the max
        // input length is (length % 128 > 128 - 17).
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let max_number_of_chunks = 1;
        let total_message_length = 128 * max_number_of_chunks;

        for i in 127 - 20..total_message_length + 1 {
            let mut rng = thread_rng();
            let total_message = (0..i).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();
            let message = total_message
                .iter()
                .map(|b| builder.constant::<ByteVariable>(*b))
                .collect::<Vec<_>>();
            let expected_digest = sha512(&total_message);

            let length = builder.constant::<U32Variable>(i as u32);

            let digest = builder.curta_sha512_variable(&message, length);
            let expected_digest = builder.constant::<BytesVariable<64>>(expected_digest);
            builder.assert_is_equal(digest, expected_digest);
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
