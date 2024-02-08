use curta::chip::ec::edwards::ed25519::params::Ed25519;
use curta::chip::ec::EllipticCurveParameters;

use super::curta::accelerator::EcOpAccelerator;
use super::curta::request::{EcOpRequest, EcOpRequestType, EcOpResponse};
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::curta::field::variable::FieldVariable;
use crate::prelude::{CircuitBuilder, PlonkParameters, U256Variable};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Add two points on the curve.
    pub fn curta_25519_add(
        &mut self,
        a: AffinePointVariable<Ed25519>,
        b: AffinePointVariable<Ed25519>,
    ) -> AffinePointVariable<Ed25519> {
        let request = EcOpRequest::Add(Box::new(a), Box::new(b));
        match self.add_ec_25519_ops_request(request) {
            EcOpResponse::Add(result) => result,
            _ => unreachable!("response should be of type Add"),
        }
    }

    /// Multiply a point on the curve by a scalar.
    pub fn curta_25519_scalar_mul(
        &mut self,
        scalar: U256Variable,
        point: AffinePointVariable<Ed25519>,
    ) -> AffinePointVariable<Ed25519> {
        let request = EcOpRequest::ScalarMul(Box::new(scalar), Box::new(point));
        match self.add_ec_25519_ops_request(request) {
            EcOpResponse::ScalarMul(result) => result,
            _ => unreachable!("response should be of type ScalarMul"),
        }
    }

    /// Decompress a compressed point.
    pub fn curta_25519_decompress(
        &mut self,
        compressed_point: CompressedEdwardsYVariable,
    ) -> AffinePointVariable<Ed25519> {
        let request = EcOpRequest::Decompress(Box::new(compressed_point));
        match self.add_ec_25519_ops_request(request) {
            EcOpResponse::Decompress(point, _) => point,
            _ => unreachable!("response should be of type Decompress"),
        }
    }

    /// Check if a point is valid.
    pub fn curta_25519_is_valid(&mut self, point: AffinePointVariable<Ed25519>) {
        let request = EcOpRequest::IsValid(Box::new(point));
        match self.add_ec_25519_ops_request(request) {
            EcOpResponse::IsValid => {}
            _ => unreachable!("response should be of type IsValid"),
        }
    }

    /// Add an EC operation request to the accelerator.
    fn add_ec_25519_ops_request(&mut self, request: EcOpRequest<Ed25519>) -> EcOpResponse<Ed25519> {
        if self.ec_25519_ops_accelerator.is_none() {
            self.ec_25519_ops_accelerator = Some(EcOpAccelerator {
                ec_op_requests: Vec::new(),
                ec_op_responses: Vec::new(),
            });
        }

        let response = match request.req_type() {
            EcOpRequestType::Add => EcOpResponse::Add(self.init::<AffinePointVariable<Ed25519>>()),
            EcOpRequestType::ScalarMul => {
                EcOpResponse::ScalarMul(self.init::<AffinePointVariable<Ed25519>>())
            }
            EcOpRequestType::Decompress => EcOpResponse::Decompress(
                self.init::<AffinePointVariable<Ed25519>>(),
                self.init::<FieldVariable<<Ed25519 as EllipticCurveParameters>::BaseField>>(),
            ),
            EcOpRequestType::IsValid => EcOpResponse::IsValid,
        };

        let accelerator = self
            .ec_25519_ops_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");

        accelerator.ec_op_requests.push(request);
        accelerator.ec_op_responses.push(response.clone());

        response
    }
}
