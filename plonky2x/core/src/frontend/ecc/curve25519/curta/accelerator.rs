use curta::chip::field::parameters::FieldParameters;

use super::request::EcOpRequest;
use super::Curve;
use crate::frontend::curta::ec::point::AffinePointVariable;

#[derive(Debug, Clone)]
pub struct EcOpAccelerator<FF: FieldParameters> {
    pub ec_op_requests: Vec<EcOpRequest<Curve, FF>>,
    pub ec_op_responses: Vec<Option<AffinePointVariable<Curve>>>,
}
