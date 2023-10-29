use core::marker::PhantomData;

use curta::chip::uint::operations::instruction::UintInstruction;
use curta::chip::uint::register::U64Register;
use curta::chip::AirParameters;
use curta::machine::hash::sha::sha512::SHA512;
use serde::{Deserialize, Serialize};

use crate::frontend::hash::sha::curta::accelerator::SHAAccelerator;
use crate::frontend::hash::sha::curta::request::SHARequest;
use crate::frontend::hash::sha::curta::SHA;
use crate::frontend::vars::EvmVariable;
use crate::prelude::*;

pub type SHA512Accelerator = SHAAccelerator<U64Variable>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SHA512AirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for SHA512AirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 1191;
    const EXTENDED_COLUMNS: usize = 654;
}

impl<L: PlonkParameters<D>, const D: usize> SHA<L, D, 80> for SHA512 {
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
        last_chunk: U32Variable,
    ) -> (Vec<Self::IntVariable>, Variable) {
        let (padded_bytes, last_chunk) =
            builder.pad_sha512_variable_length(input, length, last_chunk);

        (
            padded_bytes
                .chunks_exact(8)
                .map(|bytes| U64Variable::decode(builder, bytes))
                .collect(),
            last_chunk,
        )
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
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Executes a SHA512 hash on the given input of fixed size.
    pub fn curta_sha512(&mut self, input: &[ByteVariable]) -> BytesVariable<64> {
        if self.sha512_accelerator.is_none() {
            self.sha512_accelerator = Some(SHA512Accelerator {
                sha_requests: Vec::new(),
                sha_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<BytesVariable<64>>();
        let digest_array = SHA512::digest_to_array(self, digest);
        let accelerator = self
            .sha512_accelerator
            .as_mut()
            .expect("sha512 accelerator should exist");
        accelerator
            .sha_requests
            .push(SHARequest::Fixed(input.to_vec()));
        accelerator.sha_responses.push(digest_array);

        digest
    }

    pub fn curta_sha512_variable(
        &mut self,
        input: &[ByteVariable],
        length: U32Variable,
        last_chunk: U32Variable,
    ) -> BytesVariable<64> {
        if self.sha512_accelerator.is_none() {
            self.sha512_accelerator = Some(SHA512Accelerator {
                sha_requests: Vec::new(),
                sha_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<BytesVariable<64>>();
        let digest_array = SHA512::digest_to_array(self, digest);
        let accelerator = self
            .sha512_accelerator
            .as_mut()
            .expect("sha512 accelerator should exist");
        accelerator
            .sha_requests
            .push(SHARequest::Variable(input.to_vec(), length, last_chunk));
        accelerator.sha_responses.push(digest_array);

        digest
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
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

    fn test_sha512_variable_length(msg: &[u8], expected_digest: [u8; 64]) {
        todo!();
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
}
