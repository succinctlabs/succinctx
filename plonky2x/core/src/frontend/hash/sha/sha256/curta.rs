use core::marker::PhantomData;

use serde::{Deserialize, Serialize};
use starkyx::chip::register::array::ArrayRegister;
use starkyx::chip::register::bit::BitRegister;
use starkyx::chip::register::element::ElementRegister;
use starkyx::chip::uint::operations::instruction::UintInstruction;
use starkyx::chip::uint::register::U64Register;
use starkyx::chip::AirParameters;
use starkyx::machine::bytes::builder::BytesBuilder;
use starkyx::machine::hash::sha::algorithm::SHAPure;
use starkyx::machine::hash::sha::builder::SHABuilder;
use starkyx::machine::hash::sha::sha256::SHA256;

use crate::frontend::hash::curta::accelerator::HashAccelerator;
use crate::frontend::hash::curta::request::HashRequest;
use crate::frontend::hash::curta::Hash;
use crate::frontend::vars::EvmVariable;
use crate::prelude::*;

pub type SHA256Accelerator = HashAccelerator<U32Variable, 8>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SHA256AirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for SHA256AirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 418;
    const EXTENDED_COLUMNS: usize = 912;
}

impl<L: PlonkParameters<D>, const D: usize> Hash<L, D, 64, false, 8> for SHA256 {
    type IntVariable = U32Variable;
    type DigestVariable = Bytes32Variable;

    type AirParameters = SHA256AirParameters<L, D>;
    type AirInstruction = UintInstruction;

    fn pad_circuit(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_message_sha256(input);

        padded_bytes
            .chunks_exact(4)
            .map(|bytes| U32Variable::decode(builder, bytes))
            .collect()
    }

    fn pad_circuit_variable_length(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_sha256_variable_length(input, length);

        padded_bytes
            .chunks_exact(4)
            .map(|bytes| U32Variable::decode(builder, bytes))
            .collect()
    }

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <Self::IntRegister as starkyx::chip::register::Register>::Value<Variable>,
    ) -> Self::IntVariable {
        let mut acc = builder.zero::<Variable>();
        for (i, byte) in value.into_iter().enumerate() {
            let two_i = builder.constant::<Variable>(L::Field::from_canonical_u32(1 << (8 * i)));
            let two_i_byte = builder.mul(two_i, byte);
            acc = builder.add(acc, two_i_byte);
        }
        U32Variable::from_variables_unsafe(&[acc])
    }

    fn digest_to_array(
        builder: &mut CircuitBuilder<L, D>,
        digest: Self::DigestVariable,
    ) -> [Self::IntVariable; 8] {
        digest
            .as_bytes()
            .chunks_exact(4)
            .map(|x| U32Variable::decode(builder, x))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn hash(message: Vec<u8>) -> [Self::Integer; 8] {
        let mut current_state = SHA256::INITIAL_HASH;
        let padded_chunks = SHA256::pad(&message);
        for chunk in padded_chunks.chunks_exact(16) {
            let pre_processed = SHA256::pre_process(chunk);
            current_state = SHA256::process(current_state, &pre_processed);
        }
        current_state
    }

    fn hash_circuit(
        builder: &mut BytesBuilder<Self::AirParameters>,
        padded_chunks: &[ArrayRegister<Self::IntRegister>],
        _: &Option<ArrayRegister<U64Register>>,
        end_bits: &ArrayRegister<BitRegister>,
        digest_bits: &ArrayRegister<BitRegister>,
        digest_indices: &ArrayRegister<ElementRegister>,
        _: &ElementRegister,
    ) -> Vec<Self::DigestRegister> {
        builder.sha::<SHA256, 64>(padded_chunks, end_bits, digest_bits, *digest_indices)
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Executes a SHA256 hash on the given input of fixed size.
    pub fn curta_sha256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(SHA256Accelerator {
                hash_requests: Vec::new(),
                hash_responses: Vec::new(),
            });
        }

        let digest = self.init::<Bytes32Variable>();
        let digest_array = SHA256::digest_to_array(self, digest);
        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator
            .hash_requests
            .push(HashRequest::Fixed(input.to_vec()));
        accelerator.hash_responses.push(digest_array);

        digest
    }

    pub fn curta_sha256_variable(
        &mut self,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> Bytes32Variable {
        let true_v = self._true();
        // Check that length <= input.len(). This is needed to ensure that users cannot prove the
        // hash of a longer message than they supplied.
        let supplied_input_length = self.constant::<U32Variable>(input.len() as u32);
        let is_length_valid = self.lte(length, supplied_input_length);
        self.assert_is_equal(is_length_valid, true_v);

        // Extend input's length to the nearest multiple of 64 (if it is not already).
        let mut input = input.to_vec();
        if (input.len() % 64) != 0 {
            input.resize(
                input.len() + 64 - (input.len() % 64),
                self.constant::<ByteVariable>(0),
            );
        }

        let last_chunk = self.compute_sha256_last_chunk(length);
        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(SHA256Accelerator {
                hash_requests: Vec::new(),
                hash_responses: Vec::new(),
            });
        }

        let digest = self.init::<Bytes32Variable>();
        let digest_array = SHA256::digest_to_array(self, digest);
        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator
            .hash_requests
            .push(HashRequest::Variable(input.to_vec(), length, last_chunk));
        accelerator.hash_responses.push(digest_array);

        digest
    }

    pub fn curta_sha256_pair(
        &mut self,
        left: Bytes32Variable,
        right: Bytes32Variable,
    ) -> Bytes32Variable {
        let mut input = Vec::new();
        input.extend(&left.as_bytes());
        input.extend(&right.as_bytes());
        self.curta_sha256(&input)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use ethers::types::H256;
    use rand::{thread_rng, Rng};

    use crate::backend::circuit::{CircuitBuild, DefaultParameters};
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::*;
    use crate::utils::hash::sha256;
    use crate::utils::{bytes, bytes32};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_fixed_short_single() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.constant::<ByteVariable>(0u8);
        let result = builder.curta_sha256(&[zero; 1]);
        builder.watch(&result, "result");

        let expected_digest =
            bytes32!("0x6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        builder.assert_is_equal(result, expected_digest);

        let circuit = builder.build();
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_fixed_long_single() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let byte_msg : Vec<u8> = bytes!("243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917");
        let msg = byte_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();
        let result = builder.curta_sha256(&msg);
        builder.watch(&result, "result");

        let expected_digest = H256::from(sha256(&byte_msg));
        // bytes32!("aca16131a2e4c4c49e656d35aac1f0e689b3151bb108fa6cf5bcc3ac08a09bf9");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        builder.assert_is_equal(result, expected_digest);

        let circuit = builder.build();
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_different_lengths() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let max_len = 513;

        let mut rng = thread_rng();
        for i in 0..max_len {
            let message = (0..i).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();
            let expected_digest = H256::from(sha256(&message));

            let message = message
                .iter()
                .map(|b| builder.constant::<ByteVariable>(*b))
                .collect::<Vec<_>>();
            let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

            let digest = builder.curta_sha256(&message);
            builder.assert_is_equal(digest, expected_digest);
        }

        let circuit = builder.build();
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_variable_single() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let byte_msg : Vec<u8> = bytes!("00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d00000000000000000000000000000000000000000000000000");
        let msg = byte_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();

        let bytes_length = builder.constant::<U32Variable>(39);

        let expected_digest =
            bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        let msg_hash = builder.curta_sha256_variable(&msg, bytes_length);
        builder.watch(&msg_hash, "msg_hash");
        builder.assert_is_equal(msg_hash, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_variable_different_chunk() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let mut byte_msg : Vec<u8> = bytes!("00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d00000000000000000000000000000000000000000000000000");
        byte_msg.resize(192, 0);
        let msg = byte_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();

        let bytes_length = builder.constant::<U32Variable>(39);

        let expected_digest =
            bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        let msg_hash = builder.curta_sha256_variable(&msg, bytes_length);
        builder.watch(&msg_hash, "msg_hash");
        builder.assert_is_equal(msg_hash, expected_digest);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_variable_different_lengths() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let max_number_of_chunks = 20;
        let total_message_length = 64 * max_number_of_chunks;

        let max_len = (total_message_length - 9) / 64;

        let mut rng = thread_rng();
        let total_message = (0..total_message_length)
            .map(|_| rng.gen::<u8>())
            .collect::<Vec<_>>();
        for i in 0..max_len {
            let message = &total_message[..i];
            let expected_digest = H256::from(sha256(message));

            let length = builder.constant::<U32Variable>(i as u32);

            let total_message = total_message
                .iter()
                .map(|b| builder.constant::<ByteVariable>(*b))
                .collect::<Vec<_>>();
            let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

            let digest = builder.curta_sha256_variable(&total_message, length);
            builder.assert_is_equal(digest, expected_digest);
        }

        let circuit = builder.build();
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_diff_sizes() {
        // Confirm that Curta SHA256 works for all sizes from 0 to 256 bytes.
        let _ = env_logger::builder().is_test(true).try_init();

        let mut builder = CircuitBuilder::<L, D>::new();

        // Generate random Vec of <u8> of size 256 bytes.
        let mut rng = rand::thread_rng();
        let msg_bytes: Vec<u8> = (0..256).map(|_| rng.gen()).collect();

        let msg_var = builder.constant::<BytesVariable<256>>(msg_bytes.clone().try_into().unwrap());

        for i in 1..256 {
            let msg = &msg_bytes.clone()[0..i];
            let msg_len = builder.constant::<U32Variable>(msg.len() as u32);

            let variable_result = builder.curta_sha256_variable(&msg_var.0, msg_len);
            let fixed_result = builder.curta_sha256(&msg_var[0..i]);

            let expected_digest = builder.constant::<Bytes32Variable>(H256::from(sha256(msg)));

            builder.assert_is_equal(variable_result, expected_digest);
            builder.assert_is_equal(fixed_result, expected_digest);
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
