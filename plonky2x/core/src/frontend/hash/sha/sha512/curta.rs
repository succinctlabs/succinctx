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
    ) -> Vec<Self::IntVariable> {
        let padded_bytes = builder.pad_sha512_variable_length(input, length, last_chunk);

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
    use rand::Rng;

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

    fn test_sha512_variable_length(
        message: &[u8],
        input_length: u32,
        last_chunk: u32,
        expected_digest: [u8; 64],
    ) {
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let input_length = builder.constant::<U32Variable>(input_length);
        let last_chunk = builder.constant::<U32Variable>(last_chunk);

        let message = message
            .iter()
            .map(|b| builder.constant::<ByteVariable>(*b))
            .collect::<Vec<_>>();
        let digest = builder.curta_sha512_variable(&message, input_length, last_chunk);

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

        test_sha512_variable_length(&msg, 0, 0, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_empty_long() {
        let msg: Vec<u8> = vec![1; 256];
        let expected_digest = bytes!("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

        test_sha512_variable_length(&msg, 0, 0, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_large_message() {
        let mut msg : Vec<u8> = bytes!("35c323757c20640a294345c89c0bfcebe3d554fdb0c7b7a0bdb72222c531b1ecf7ec1c43f4de9d49556de87b86b26a98942cb078486fdb44de38b80864c3973153756363696e6374204c616273");
        let len = msg.len() as u32;
        msg.resize(256, 1);
        let expected_digest = bytes!("4388243c4452274402673de881b2f942ff5730fd2c7d8ddb94c3e3d789fb3754380cba8faa40554d9506a0730a681e88ab348a04bc5c41d18926f140b59aed39");

        test_sha512_variable_length(&msg, len, 0, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_short_message_same_slice() {
        let mut msg: Vec<u8> = b"plonky2".to_vec();
        let len = msg.len() as u32;
        msg.resize(128, 1);
        let expected_digest = bytes!("7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87");

        test_sha512_variable_length(&msg, len, 0, expected_digest);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sha512_curta_variable_short_message_different_slice() {
        let mut msg: Vec<u8> = b"plonky2".to_vec();
        let len = msg.len() as u32;
        msg.resize(512, 1);
        let expected_digest = bytes!("7c6159dd615db8c15bc76e23d36106e77464759979a0fcd1366e531f552cfa0852dbf5c832f00bb279cbc945b44a132bff3ed0028259813b6a07b57326e88c87");

        test_sha512_variable_length(&msg, len, 0, expected_digest);
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
}
