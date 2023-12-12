use core::marker::PhantomData;

use curta::chip::uint::operations::instruction::UintInstruction;
use curta::chip::AirParameters;
use serde::{Deserialize, Serialize};

use super::accelerator::BLAKE2BAccelerator;
use super::request::BLAKE2BRequest;
use super::stark::{compute_blake2b_last_chunk_index, digest_to_array};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BLAKE2BAirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for BLAKE2BAirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = UintInstruction;

    const NUM_FREE_COLUMNS: usize = 1527;
    const EXTENDED_COLUMNS: usize = 708;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn curta_blake2b(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        if self.blake2b_accelerator.is_none() {
            self.blake2b_accelerator = Some(BLAKE2BAccelerator {
                blake2b_requests: Vec::new(),
                blake2b_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_array = digest_to_array(self, digest);
        let accelerator = self
            .blake2b_accelerator
            .as_mut()
            .expect("blake2b accelerator should exist");
        accelerator
            .blake2b_requests
            .push(BLAKE2BRequest::Fixed(input.to_vec()));
        accelerator.blake2b_responses.push(digest_array);

        digest
    }

    pub fn curta_blake2b_variable(
        &mut self,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> Bytes32Variable {
        let last_chunk = compute_blake2b_last_chunk_index(self, length);
        if self.blake2b_accelerator.is_none() {
            self.blake2b_accelerator = Some(BLAKE2BAccelerator {
                blake2b_requests: Vec::new(),
                blake2b_responses: Vec::new(),
            });
        }

        let digest = self.init_unsafe::<Bytes32Variable>();
        let digest_array = digest_to_array(self, digest);

        let accelerator = self
            .blake2b_accelerator
            .as_mut()
            .expect("blake2b accelerator should exist");
        accelerator.blake2b_requests.push(BLAKE2BRequest::Variable(
            input.to_vec(),
            length,
            last_chunk,
        ));
        accelerator.blake2b_responses.push(digest_array);

        digest
    }
}

#[cfg(test)]
mod tests {
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
        let _ = env_logger::builder().is_test(true).try_init();

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
