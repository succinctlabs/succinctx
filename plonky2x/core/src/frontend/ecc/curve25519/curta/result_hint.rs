use curta::chip::ec::edwards::ed25519::decompress::decompress;
use curta::chip::ec::point::AffinePoint;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use super::request::EcOpRequestType;
use super::Curve;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::Uint;
use crate::prelude::{PlonkParameters, U256Variable, ValueStream};

/// Provides the result of a EC operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcOpResultHint {
    ec_op: EcOpRequestType,
}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for EcOpResultHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let mut result: Option<AffinePoint<Curve>> = None;
        match &self.ec_op {
            EcOpRequestType::Add => {
                let a = input_stream.read_value::<AffinePointVariable<Curve>>();
                let b = input_stream.read_value::<AffinePointVariable<Curve>>();
                result = Some(a + b);
            }
            EcOpRequestType::ScalarMul => {
                let scalar = BigUint::new(
                    input_stream
                        .read_value::<U256Variable>()
                        .to_u32_limbs()
                        .to_vec(),
                );
                let point = input_stream.read_value::<AffinePointVariable<Curve>>();
                result = Some(point * scalar);
            }
            EcOpRequestType::Decompress => {
                let compressed_point = input_stream.read_value::<CompressedEdwardsYVariable>();
                result = Some(decompress(&compressed_point));
            }
            EcOpRequestType::IsValid => {}
        }

        if let Some(return_val) = result {
            output_stream.write_value::<AffinePointVariable<Curve>>(return_val);
        }
    }
}

impl EcOpResultHint {
    pub fn new(ec_op: EcOpRequestType) -> Self {
        Self { ec_op }
    }
}
