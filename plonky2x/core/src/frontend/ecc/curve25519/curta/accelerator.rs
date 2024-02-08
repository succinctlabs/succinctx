use super::request::{EcOpRequest, EcOpResponse};
use super::Curve;

#[derive(Debug, Clone)]
pub struct EcOpAccelerator {
    pub ec_op_requests: Vec<EcOpRequest<Curve>>,
    pub ec_op_responses: Vec<EcOpResponse<Curve>>,
}
