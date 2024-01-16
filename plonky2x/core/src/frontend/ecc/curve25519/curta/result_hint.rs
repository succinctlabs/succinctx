use curta::chip::ec::edwards::ed25519::decompress::decompress;
use curta::chip::ec::EllipticCurveParameters;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use super::request::EcOpRequestType;
use super::Curve;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::curta::field::variable::FieldVariable;
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
        match &self.ec_op {
            EcOpRequestType::Add => {
                let a = input_stream.read_value::<AffinePointVariable<Curve>>();
                let b = input_stream.read_value::<AffinePointVariable<Curve>>();
                output_stream.write_value::<AffinePointVariable<Curve>>(a + b);
            }
            EcOpRequestType::ScalarMul => {
                let scalar = BigUint::new(
                    input_stream
                        .read_value::<U256Variable>()
                        .to_u32_limbs()
                        .to_vec(),
                );
                let point = input_stream.read_value::<AffinePointVariable<Curve>>();
                output_stream.write_value::<AffinePointVariable<Curve>>(point * scalar);
            }
            EcOpRequestType::Decompress => {
                let compressed_point = input_stream.read_value::<CompressedEdwardsYVariable>();
                let (point, root) = decompress(&compressed_point);
                output_stream.write_value::<AffinePointVariable<Curve>>(point);
                output_stream
                    .write_value::<FieldVariable<<Curve as EllipticCurveParameters>::BaseField>>(
                        root,
                    );
            }
            EcOpRequestType::IsValid => {}
        }
    }
}

impl EcOpResultHint {
    pub fn new(ec_op: EcOpRequestType) -> Self {
        Self { ec_op }
    }
}
