use core::marker::PhantomData;

use curta::chip::ec::point::AffinePoint;
use curta::chip::ec::EllipticCurve;
use curta::chip::field::parameters::FieldParameters;
use serde::{Deserialize, Serialize};

use super::request::EcOpRequestType;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::num::nonnative::nonnative::NonNativeTarget;
use crate::prelude::{PlonkParameters, ValueStream};

/// Provides the result of a EC operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcOpResultHint<E: EllipticCurve, FF: FieldParameters> {
    ec_op: EcOpRequestType,
    _marker: PhantomData<(E, FF)>,
}

impl<L: PlonkParameters<D>, const D: usize, E: EllipticCurve, FF: FieldParameters> Hint<L, D>
    for EcOpResultHint<E, FF>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let mut result: Option<AffinePoint<E>> = None;
        match &self.ec_op {
            Add => {
                let a = input_stream.read_value::<AffinePointVariable<E>>();
                let b = input_stream.read_value::<AffinePointVariable<E>>();
                result = Some(a + b);
            }
            ScalarMul => {
                let scalar = input_stream.read_value::<NonNativeTarget<FF>>();
                let point = input_stream.read_value::<AffinePointVariable<E>>();
                result = Some(point.scalar_mul(&scalar));
            }
            Decompress => {
                let compressed_point = input_stream.read_value::<CompressedEdwardsYVariable>();
                result = Some(decompress_point(compressed_point));
            }
            IsValid => {}
        }

        if result.is_some() {
            output_stream.write_value::<AffinePointVariable<E>>(result.unwrap());
        }
    }
}

impl<E: EllipticCurve, FF: FieldParameters> EcOpResultHint<E, FF> {
    pub fn new(ec_op: EcOpRequestType) -> Self {
        Self {
            ec_op,
            _marker: PhantomData,
        }
    }
}
