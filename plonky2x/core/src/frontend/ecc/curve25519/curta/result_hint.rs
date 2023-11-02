use core::marker::PhantomData;

use curta::chip::ec::edwards::ed25519::decompress::decompress;
use curta::chip::ec::point::AffinePoint;
use curta::chip::field::parameters::FieldParameters;
use serde::{Deserialize, Serialize};

use super::request::EcOpRequestType;
use super::Curve;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::num::nonnative::nonnative::NonNativeTarget;
use crate::prelude::{PlonkParameters, ValueStream};

/// Provides the result of a EC operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcOpResultHint<FF: FieldParameters> {
    ec_op: EcOpRequestType,
    _marker: PhantomData<FF>,
}

impl<L: PlonkParameters<D>, const D: usize, FF: FieldParameters> Hint<L, D> for EcOpResultHint<FF> {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let mut result: Option<AffinePoint<Curve>> = None;
        match &self.ec_op {
            EcOpRequestType::Add => {
                let a = input_stream.read_value::<AffinePointVariable<Curve>>();
                let b = input_stream.read_value::<AffinePointVariable<Curve>>();
                result = Some(a + b);
            }
            EcOpRequestType::ScalarMul => {
                let scalar = input_stream.read_value::<NonNativeTarget<FF>>();
                let point = input_stream.read_value::<AffinePointVariable<Curve>>();
                result = Some(point.scalar_mul(&scalar));
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

impl<FF: FieldParameters> EcOpResultHint<FF> {
    pub fn new(ec_op: EcOpRequestType) -> Self {
        Self {
            ec_op,
            _marker: PhantomData,
        }
    }
}
