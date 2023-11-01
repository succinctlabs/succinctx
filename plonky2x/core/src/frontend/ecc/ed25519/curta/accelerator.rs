use curta::chip::ec::EllipticCurve;
use curta::chip::field::parameters::FieldParameters;

use super::request::EcOpRequest;
use crate::frontend::curta::ec::point::AffinePointVariable;

#[derive(Debug, Clone)]
pub struct EcOpAccelerator<E: EllipticCurve, FF: FieldParameters> {
    pub ec_op_requests: Vec<EcOpRequest<E, FF>>,
    pub ec_op_responses: Vec<Option<AffinePointVariable<E>>>,
}
