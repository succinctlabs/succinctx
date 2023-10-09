use core::marker::PhantomData;

use curta::chip::hash::blake::blake2b::builder_gadget::{BLAKE2BBuilder, BLAKE2BBuilderGadget};
use curta::chip::hash::blake::blake2b::generator::BLAKE2BAirParameters;
use plonky2::iop::target::Target;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ByteVariable, CircuitBuilder, CircuitVariable, Variable};

pub const MAX_NUM_CURTA_CHUNKS: usize = 1600;

#[derive(Debug, Clone)]
pub struct CurtaBlake2BRequest {
    message: Vec<Target>,
    message_len: Target,
    digest: [Target; 32],
    chunk_size: usize,
}

#[derive(Debug, Clone)]
pub struct Blake2bAccelerator<L: PlonkParameters<D>, const D: usize> {
    pub requests: Vec<CurtaBlake2BRequest>,
    _marker: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> Blake2bAccelerator<L, D> {
    pub fn build(&self, builder: &mut CircuitBuilder<L, D>) {
        builder.curta_constrain_blake2b(self);
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Pads a BLAKE2B input
    pub fn curta_blake2b_pad<const MAX_NUM_CHUNKS: usize>(
        &mut self,
        message: &[ByteVariable],
    ) -> Vec<ByteVariable> {
        assert!(message.len() <= MAX_NUM_CHUNKS * 128, "message too long");

        let padlen = MAX_NUM_CHUNKS * 128 - message.len();
        if padlen > 0 {
            let mut padded_message = Vec::new();
            padded_message.extend(message);

            for _i in 0..padlen {
                padded_message.push(self.constant::<ByteVariable>(0u8));
            }

            padded_message
        } else {
            message.to_vec()
        }
    }

    /// Executes a BLAKE2B hash on the given message.
    pub fn curta_blake2b_variable<const MAX_NUM_CHUNKS: usize>(
        &mut self,
        message: &[ByteVariable],
        message_len: Variable,
    ) -> Bytes32Variable {
        let padded_message = self.curta_blake2b_pad::<MAX_NUM_CHUNKS>(message);

        let message_target_bytes = padded_message
            .iter()
            .map(|x| x.to_variable(self).0)
            .collect::<Vec<_>>();
        let message_len_target = message_len.targets()[0];
        let digest = self.api.add_virtual_target_arr::<32>();

        if self.blake2b_accelerator.is_none() {
            self.blake2b_accelerator = Some(Blake2bAccelerator::<L, D> {
                requests: Vec::new(),
                _marker: PhantomData,
            });
        }

        let accelerator = self
            .blake2b_accelerator
            .as_mut()
            .expect("blake2b accelerator should exist");

        let curta_blake2b_request = CurtaBlake2BRequest {
            message: message_target_bytes,
            message_len: message_len_target,
            digest,
            chunk_size: MAX_NUM_CHUNKS,
        };

        accelerator.requests.push(curta_blake2b_request);

        let bytes: [ByteVariable; 32] = digest.map(|x| ByteVariable::from_target(self, x));

        bytes.into()
    }

    /// Verifies a blake2b curta instance
    pub fn curta_constrain_blake2b(&mut self, accelerator: &Blake2bAccelerator<L, D>) {
        let mut padded_messages = Vec::new();
        let mut msg_lengths = Vec::new();
        let mut digests = Vec::new();
        let mut chunk_sizes = Vec::new();

        for curta_req in accelerator.requests.iter() {
            padded_messages.extend(curta_req.message.clone());
            msg_lengths.push(curta_req.message_len);
            digests.extend(curta_req.digest);
            chunk_sizes.push(curta_req.chunk_size as u64);
        }

        let mut blake2b_builder_gadget: BLAKE2BBuilderGadget<
            BLAKE2BAirParameters<L::Field, L::CubicParams>,
            MAX_NUM_CURTA_CHUNKS,
        > = self.api.init_blake2b();
        blake2b_builder_gadget
            .padded_messages
            .extend(padded_messages.clone());
        blake2b_builder_gadget.msg_lengths.extend(msg_lengths);
        blake2b_builder_gadget.digests.extend(digests);
        blake2b_builder_gadget.chunk_sizes.extend(chunk_sizes);

        // For now, only allow 1 blake2b curta proof per circuit
        assert!(
            padded_messages.len() <= MAX_NUM_CURTA_CHUNKS * 128,
            "Too many chunks for Curta BLAKE2B"
        );

        self.api
            .constrain_blake2b_gadget::<L::CubicParams, L::CurtaConfig>(blake2b_builder_gadget);
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::{BytesVariable, CircuitBuilder, Variable};
    use crate::utils::bytes32;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_blake2b_curta_empty_string() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        const MAX_NUM_CHUNKS: usize = 4;

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.zero();
        let result = builder.curta_blake2b_variable::<MAX_NUM_CHUNKS>(&[], zero);

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
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        type F = GoldilocksField;

        let msg_hex = "00f43f3ef4c05d1aca645d7b2b59af99d65661810b8a724818052db75e04afb60ea210002f9cac87493604cb5fff6644ea17c3b1817d243bc5a0aa6f0d11ab3df46f37b9adbf1ff3a446807e7a9ebc77647776b8bbda37dcf2f4f34ca7ba7bf4c7babfbe080642414245b501032c000000b7870a0500000000360b79058f3b331fbbb10d38a2e309517e24cc12094d0a5a7c9faa592884e9621aecff0224bc1a857a0bacadf4455e2c5b39684d2d5879b108c98315f6a14504348846c6deed3addcba24fc3af531d59f31c87bc454bf6f1d73eadaf2d22d60c05424142450101eead41c1266af7bc7becf961dcb93f3691642c9b6d50aeb65b92528b99c675608f2095a296ed52aa433c1bfed56e8546dae03b61cb59643a9cb39f82618f958b00041000000000000000000000000000000000000000000000000000000000000000008101a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e07918a26cc6796f1025d51bd927351af541d3ab01d7a1b978a65e19c16ae2799b3286ca2401211009421c4e6bd80ef9e079180400";
        let msg_bytes = hex::decode(msg_hex).unwrap();
        const MSG_LEN: usize = 423;
        assert!(msg_bytes.len() == MSG_LEN);

        const MAX_NUM_CHUNKS: usize = 5;

        let mut builder = CircuitBuilder::<L, D>::new();

        let msg = builder.constant::<BytesVariable<MSG_LEN>>(msg_bytes.clone().try_into().unwrap());
        let bytes_length = builder.constant::<Variable>(F::from_canonical_usize(msg_bytes.len()));
        let result = builder.curta_blake2b_variable::<MAX_NUM_CHUNKS>(&msg.0, bytes_length);

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
}
