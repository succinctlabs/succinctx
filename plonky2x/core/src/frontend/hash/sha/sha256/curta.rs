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
        length: Variable,
    ) -> Vec<Self::IntVariable> {
        todo!()
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

    use crate::backend::circuit::{CircuitBuild, DefaultParameters};
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{
        ByteVariable, CircuitBuilder, DefaultBuilder, GateRegistry, HintRegistry,
    };
    use crate::utils::{bytes, bytes32};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha256_curta_fixed_single() {
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
    fn test_curta_allocation() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = DefaultBuilder::new();

        // Requires 2 chunks each.
        let short_msg = [1u8; 56];

        let short_msg_bytes = short_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();

        let mut msgs = (0..1024)
            .map(|_| short_msg_bytes.clone())
            .collect::<Vec<_>>();

        // Requires 3 chunks each.
        let long_msg = [1u8; 128];
        let long_msg_bytes = long_msg
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();

        msgs.extend(
            (0..2048)
                .map(|_| long_msg_bytes.clone())
                .collect::<Vec<_>>(),
        );

        let mut builder = CircuitBuilder::<L, D>::new();
        let _ = msgs
            .iter()
            .map(|msg| builder.curta_sha256(msg))
            .collect::<Vec<_>>();

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    // #[test]
    // #[cfg_attr(feature = "ci", ignore)]
    // fn test_sha256_curta_variable_single() {
    //     env::set_var("RUST_LOG", "debug");
    //     env_logger::try_init().unwrap_or_default();
    //     dotenv::dotenv().ok();

    //     let mut builder = CircuitBuilder::<L, D>::new();

    //     let msg = builder.constant::<BytesVariable<64>>(bytes!(
    //         "00de6ad0941095ada2a7996e6a888581928203b8b69e07ee254d289f5b9c9caea193c2ab01902d00000000000000000000000000000000000000000000000000"
    //     ));

    //     let bytes_length = builder.constant::<U32Variable>(39);

    //     let expected_digest =
    //         bytes32!("84f633a570a987326947aafd434ae37f151e98d5e6d429137a4cc378d4a7988e");
    //     let expected_digest = builder.constant::<Bytes32Variable>(expected_digest);

    //     let last_chunk = builder.constant::<U32Variable>(0);

    //     let msg_hash = builder.curta_sha256_variable::<1>(&msg.0, last_chunk, bytes_length);
    //     builder.watch(&msg_hash, "msg_hash");
    //     builder.assert_is_equal(msg_hash, expected_digest);

    //     let circuit = builder.build();
    //     let input = circuit.input();
    //     let (proof, output) = circuit.prove(&input);
    //     circuit.verify(&proof, &input, &output);

    //     circuit.test_default_serializers();
    // }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_serialized_sha256_curta() {
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
        let hint_serializer = HintRegistry::<L, D>::new();
        let bytes = circuit
            .serialize(&gate_serializer, &hint_serializer)
            .unwrap();
        let circuit =
            CircuitBuild::<L, D>::deserialize(&bytes, &gate_serializer, &hint_serializer).unwrap();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }
}
