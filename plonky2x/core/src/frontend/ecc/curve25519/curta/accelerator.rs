use super::request::EcOpRequest;
use super::Curve;
use crate::frontend::curta::ec::point::AffinePointVariable;

#[derive(Debug, Clone)]
pub struct EcOpAccelerator {
    pub ec_op_requests: Vec<EcOpRequest<Curve>>,
    pub ec_op_responses: Vec<Option<AffinePointVariable<Curve>>>,
}
