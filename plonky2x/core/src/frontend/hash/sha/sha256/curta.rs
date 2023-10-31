use core::marker::PhantomData;

use curta::chip::uint::operations::instruction::UintInstruction;
use curta::chip::AirParameters;
use curta::machine::hash::sha::sha256::SHA256;
use serde::{Deserialize, Serialize};

use crate::frontend::hash::sha::curta::accelerator::SHAAccelerator;
use crate::frontend::hash::sha::curta::request::SHARequest;
use crate::frontend::hash::sha::curta::SHA;
use crate::frontend::vars::EvmVariable;
use crate::prelude::*;

pub type SHA256Accelerator = SHAAccelerator<U32Variable>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SHA256AirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for SHA256AirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 605;
    const EXTENDED_COLUMNS: usize = 351;
}

impl<L: PlonkParameters<D>, const D: usize> SHA<L, D, 64> for SHA256 {
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
        last_chunk: U32Variable,
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_message_sha256_variable(input, length, last_chunk);

        padded_bytes
            .chunks_exact(4)
            .map(|bytes| U32Variable::decode(builder, bytes))
            .collect()
    }

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <Self::IntRegister as curta::chip::register::Register>::Value<Variable>,
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
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Executes a SHA256 hash on the given input of fixed size.
    pub fn curta_sha256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(SHA256Accelerator {
                sha_requests: Vec::new(),
                sha_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_array = SHA256::digest_to_array(self, digest);
        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator
            .sha_requests
            .push(SHARequest::Fixed(input.to_vec()));
        accelerator.sha_responses.push(digest_array);

        digest
    }

    pub fn curta_sha256_variable(
        &mut self,
        input: &[ByteVariable],
        length: U32Variable,
        last_chunk: U32Variable,
    ) -> Bytes32Variable {
        assert_eq!(
            input.len() % 64,
            0,
            "input length should be a multiple of 64"
        );
        if self.sha256_accelerator.is_none() {
            self.sha256_accelerator = Some(SHA256Accelerator {
                sha_requests: Vec::new(),
                sha_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_array = SHA256::digest_to_array(self, digest);
        let accelerator = self
            .sha256_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");
        accelerator
            .sha_requests
            .push(SHARequest::Variable(input.to_vec(), length, last_chunk));
        accelerator.sha_responses.push(digest_array);

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
        let last_chunk = builder.constant::<U32Variable>(0);

        let expected_digest =
            bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        let msg_hash = builder.curta_sha256_variable(&msg, bytes_length, last_chunk);
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
        let last_chunk = builder.constant::<U32Variable>(0);

        let expected_digest =
            bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
        let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

        let msg_hash = builder.curta_sha256_variable(&msg, bytes_length, last_chunk);
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

        let max_number_of_chunks = 2;
        let total_message_length = 64 * max_number_of_chunks;
        let max_len = 56;

        let mut rng = thread_rng();
        let total_message = (0..total_message_length)
            .map(|_| rng.gen::<u8>())
            .collect::<Vec<_>>();
        for i in 0..max_len {
            let message = &total_message[..i];
            let expected_digest = H256::from(sha256(message));

            let length = builder.constant::<U32Variable>(i as u32);
            let last_chunk = builder.constant::<U32Variable>((i as u32 + 8) / 64);

            let total_message = total_message
                .iter()
                .map(|b| builder.constant::<ByteVariable>(*b))
                .collect::<Vec<_>>();
            let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

            let digest = builder.curta_sha256_variable(&total_message, length, last_chunk);
            builder.watch(&digest, &format!("digest of message {}", i));
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
}
