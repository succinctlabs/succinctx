use curta::chip::hash::blake::blake2b::builder_gadget::{BLAKE2BBuilder, BLAKE2BBuilderGadget};
use curta::chip::hash::blake::blake2b::generator::BLAKE2BAirParameters;
use plonky2::iop::target::Target;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CurtaRequest;
use crate::frontend::hash::bit_operations::{
    convert_byte_target_to_byte_var, convert_byte_var_to_target,
};
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ByteVariable, CircuitBuilder, CircuitVariable, Div};

pub struct CurtaBlake2BRequest {
    message: Vec<Target>,
    message_len: Target,
    digest: [Target; 32],
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn curta_blake2b_pad<'a, const MAX_NUM_CHUNKS: usize>(
        &mut self,
        message: &[ByteVariable],
    ) -> Vec<ByteVariable> {
        // TODO: Currently, Curta does not support no-ops over BLAKE2B chunks. Until Curta BLAKE2B supports no-ops, last_chunk should always be equal to MAX_NUM_CHUNKS - 1.
        if (message.len() % 128 == 0) && (!message.len() == 0) {
            message.to_vec()
        } else {
            let padlen = 128 - (message.len() % 128);

            let mut padded_message = Vec::new();
            padded_message.extend(message);

            for _i in 0..padlen {
                padded_message.push(self.constant::<ByteVariable>(0u8));
            }

            padded_message
        }
    }

    /// Executes a BLAKE2B hash on the given message.
    pub fn curta_blake2b_variable<const MAX_NUM_CHUNKS: usize>(
        &mut self,
        message: &[ByteVariable],
        message_len: U64Variable,
    ) -> Bytes32Variable {
        // TODO: Currently, Curta does not support no-ops over BLAKE2B chunks. Until Curta BLAKE2B supports no-ops, last_chunk should always be equal to MAX_NUM_CHUNKS - 1.
        let expected_last_chunk_num = self.constant::<U64Variable>((MAX_NUM_CHUNKS - 1).into());
        let last_chunk_num = message_len.div(self.constant::<U64Variable>(128.into()), self);
        self.assert_is_equal(expected_last_chunk_num, last_chunk_num);

        let padded_message = self.curta_blake2b_pad::<MAX_NUM_CHUNKS>(message);

        let message_target_bytes = padded_message
            .iter()
            .map(|x| convert_byte_var_to_target(*x, &mut self.api))
            .collect::<Vec<_>>();
        let message_len_target = message_len.targets()[0];
        let digest = self.api.add_virtual_target_arr::<32>();

        let curta_blake2b_request = CurtaBlake2BRequest {
            message: message_target_bytes,
            message_len: message_len_target,
            digest,
        };
        self.curta_requests
            .push(CurtaRequest::Blake2b(curta_blake2b_request));
        self.register_curta_contraint_fn(CircuitBuilder::curta_constrain_blake2b);

        let bytes: [ByteVariable; 32] =
            digest.map(|x| convert_byte_target_to_byte_var(x, &mut self.api));

        bytes.into()
    }

    pub fn curta_constrain_blake2b(&mut self) {
        let mut padded_messages = Vec::new();
        let mut msg_lengths = Vec::new();
        let mut digests = Vec::new();

        for curta_req in self.curta_requests.iter() {
            match curta_req {
                CurtaRequest::Blake2b(curta_blake2b_req) => {
                    padded_messages.extend(curta_blake2b_req.message.clone());
                    msg_lengths.push(curta_blake2b_req.message_len);
                    digests.extend(curta_blake2b_req.digest);
                }
                _ => {}
            }
        }
        let mut blake2b_builder_gadget: BLAKE2BBuilderGadget<
            BLAKE2BAirParameters<L::Field, L::CubicParams>,
        > = self.api.init_blake2b();
        blake2b_builder_gadget
            .padded_messages
            .extend(padded_messages.clone());
        blake2b_builder_gadget.msg_lengths.extend(msg_lengths);
        blake2b_builder_gadget.digests.extend(digests);

        // For now, only allow 1 blake2b curta proof per circuit
        let max_num_chunks = blake2b_builder_gadget.max_num_chunks();
        assert!(
            padded_messages.len() <= max_num_chunks * 128,
            "Too many chunks for Curta BLAKE2B"
        );

        self.api
            .constrain_blake2b_gadget::<L::CubicParams, L::CurtaConfig>(blake2b_builder_gadget);
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::CircuitBuilder;
    use crate::utils::bytes32;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_blake2b_curta() {
        env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let zero = builder.zero();
        let result = builder.curta_blake2b_variable::<1>(&[], zero);
        builder.watch(&result, "result");

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
}
