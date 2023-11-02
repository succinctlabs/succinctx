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
    ) -> BytesVariable<64> {
        let last_chunk = self.compute_sha512_last_chunk(length);

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

        let max_number_of_chunks = 20;
        let total_message_length = 128 * max_number_of_chunks;
        let max_len = (total_message_length - 18) / 128;

        let mut rng = thread_rng();
        let total_message = (0..total_message_length)
            .map(|_| rng.gen::<u8>())
            .collect::<Vec<_>>();
        for i in 0..max_len {
            let message = &total_message[..i];
            let expected_digest = sha512(message);
            let message = total_message
                .iter()
                .map(|b| builder.constant::<ByteVariable>(*b))
                .collect::<Vec<_>>();

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
    fn failing_sha512() {
        let mut test_message: Vec<u8> = vec![
            123, 91, 220, 35, 211, 213, 7, 45, 164, 151, 71, 169, 218, 173, 38, 104, 3, 16, 168,
            29, 247, 203, 160, 36, 33, 222, 19, 145, 18, 98, 135, 138, 217, 137, 40, 54, 42, 164,
            182, 68, 116, 5, 187, 83, 134, 107, 101, 27, 140, 142, 201, 195, 235, 207, 238, 29, 69,
            155, 125, 198, 29, 40, 193, 84, 52, 68, 94, 231, 122, 44, 188, 97, 64, 164, 221, 146,
            226, 151, 30, 140, 73, 172, 195, 53, 145, 239, 36, 2, 28, 2, 92, 208, 92, 83, 66, 22,
            251, 123, 154, 191, 60, 230, 209, 187, 102, 45, 113, 220, 174, 93, 94, 127, 106, 229,
            64, 64, 206, 216, 188, 137, 4, 223, 203, 36, 15, 33, 244, 21, 175, 120, 213, 8, 81,
            229, 222, 149, 66, 240, 83, 85, 177, 32, 116, 202, 142, 149, 249, 161, 205, 72, 9, 38,
            178, 124, 227, 133, 90, 235, 117, 192, 13, 252, 159, 135, 105, 38, 105, 214, 216, 39,
            201, 113, 153, 100, 105, 73, 191, 44, 41, 148, 160, 92, 73, 155, 139, 131, 101, 149,
            62, 58, 55, 108, 24, 73, 91, 58, 119, 29, 238, 226, 141, 59, 194, 80, 8, 231, 42, 177,
            217, 222, 144, 34, 212, 98, 37, 189, 49, 200, 4, 214, 190, 9, 132, 18, 57, 12, 161,
            157, 222, 198, 57, 140, 233, 231, 138, 80, 238, 217, 38, 239, 247, 120,
        ];

        let test_message_len = test_message.len();
        test_message.resize(256, 0);

        let mut builder = DefaultBuilder::new();
        let message = builder.read::<BytesVariable<256>>();
        let message_len = builder.read::<U32Variable>();
        builder.curta_sha512_variable(message.0.as_slice(), message_len);

        let circuit = builder.build();
        let mut input = circuit.input();
        input.write::<BytesVariable<256>>(test_message.try_into().unwrap());
        input.write::<U32Variable>(test_message_len as u32);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
