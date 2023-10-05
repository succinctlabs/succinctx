use log::debug;
use num::traits::ToBytes;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use super::fetch::{verify_signature, RpcDataFetcher};
use super::vars::{
    Curve, EDDSAPublicKeyVariable, SignatureValueType, SimpleJustificationData,
    ENCODED_PRECOMMIT_LENGTH,
};
use crate::frontend::ecc::ed25519::curve::curve_types::AffinePoint;
use crate::frontend::ecc::ed25519::curve::ed25519::Ed25519;
use crate::frontend::ecc::ed25519::curve::eddsa::{verify_message, EDDSASignature};
use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
use crate::frontend::ecc::ed25519::gadgets::eddsa::EDDSASignatureTarget;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::ValueStream;
use crate::prelude::{
    ArrayVariable, BoolVariable, BytesVariable, Field, PlonkParameters, RichField,
};
use crate::utils::to_be_bits;

fn signature_to_value_type<F: RichField>(sig_bytes: &[u8]) -> SignatureValueType<F> {
    let sig_r = AffinePoint::new_from_compressed_point(&sig_bytes[0..32]);
    assert!(sig_r.is_valid());
    let sig_s_biguint = BigUint::from_bytes_le(&sig_bytes[32..64]);
    if sig_s_biguint.to_u32_digits().is_empty() {
        panic!("sig_s_biguint has 0 limbs which will cause problems down the line")
    }
    let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
    SignatureValueType::<F> { r: sig_r, s: sig_s }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintSimpleJustification<const NUM_AUTHORITIES: usize> {}

impl<const NUM_AUTHORITIES: usize, L: PlonkParameters<D>, const D: usize> Hint<L, D>
    for HintSimpleJustification<NUM_AUTHORITIES>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let block_number = input_stream.read_value::<U32Variable>();
        let authority_set_id = input_stream.read_value::<U64Variable>();

        debug!(
            "HintSimpleJustification: downloading justification for block_number {} and authority_set_id {}",
            block_number,
            authority_set_id
        );

        let rt = Runtime::new().expect("failed to create tokio runtime");
        let justification_data: SimpleJustificationData = rt.block_on(async {
            let data_fetcher = RpcDataFetcher::new().await;
            data_fetcher
                .get_simple_justification::<NUM_AUTHORITIES>(block_number)
                .await
        });

        if justification_data.authority_set_id != authority_set_id {
            panic!("Authority set id does not match");
        }

        let encoded_precommit = justification_data.signed_message;
        if encoded_precommit.len() != ENCODED_PRECOMMIT_LENGTH {
            panic!("Encoded precommit is not the correct length");
        }

        verify_signature(
            &justification_data.pubkeys[0].compress_point().to_le_bytes(),
            &encoded_precommit,
            &justification_data.signatures[0],
        );

        let value_type = signature_to_value_type::<L::Field>(&justification_data.signatures[0]);

        verify_message(
            &to_be_bits(&encoded_precommit),
            &EDDSASignature::<Ed25519> {
                r: value_type.r,
                s: value_type.s,
            },
            &crate::frontend::ecc::ed25519::curve::eddsa::EDDSAPublicKey(
                justification_data.pubkeys[0],
            ),
        );

        output_stream.write_value::<BytesVariable<ENCODED_PRECOMMIT_LENGTH>>(
            encoded_precommit.try_into().unwrap(),
        );
        output_stream.write_value::<ArrayVariable<BoolVariable, NUM_AUTHORITIES>>(
            justification_data.validator_signed,
        );
        output_stream.write_value::<ArrayVariable<EDDSASignatureTarget<Curve>, NUM_AUTHORITIES>>(
            justification_data
                .signatures
                .iter()
                .map(|x| signature_to_value_type::<L::Field>(x))
                .collect(),
        );
        output_stream.write_value::<ArrayVariable<EDDSAPublicKeyVariable, NUM_AUTHORITIES>>(
            justification_data.pubkeys,
        );
    }
}
