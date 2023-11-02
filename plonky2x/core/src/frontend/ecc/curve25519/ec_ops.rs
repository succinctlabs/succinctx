use curta::chip::ec::edwards::ed25519::params::{Ed25519, Ed25519ScalarField};

use super::curta::accelerator::EcOpAccelerator;
use super::curta::request::{EcOpRequest, EcOpRequestType};
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::num::nonnative::nonnative::NonNativeTarget;
use crate::prelude::{CircuitBuilder, PlonkParameters};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn curta_add(
        &mut self,
        a: AffinePointVariable<Ed25519>,
        b: AffinePointVariable<Ed25519>,
    ) -> AffinePointVariable<Ed25519> {
        let request = EcOpRequest::Add(Box::new(a), Box::new(b));
        self.add_ec_ops_request(request).unwrap()
    }

    pub fn curta_scalar_mul(
        &mut self,
        scalar: NonNativeTarget<Ed25519ScalarField>,
        point: AffinePointVariable<Ed25519>,
    ) -> AffinePointVariable<Ed25519> {
        let request = EcOpRequest::ScalarMul(Box::new(scalar), Box::new(point));
        self.add_ec_ops_request(request).unwrap()
    }

    pub fn curta_decompress(
        &mut self,
        compressed_point: CompressedEdwardsYVariable,
    ) -> AffinePointVariable<Ed25519> {
        let request = EcOpRequest::Decompress(Box::new(compressed_point));
        self.add_ec_ops_request(request).unwrap()
    }

    pub fn curta_is_valid(&mut self, point: AffinePointVariable<Ed25519>) {
        let request = EcOpRequest::IsValid(Box::new(point));
        self.add_ec_ops_request(request);
    }

    fn add_ec_ops_request(
        &mut self,
        request: EcOpRequest<Ed25519, Ed25519ScalarField>,
    ) -> Option<AffinePointVariable<Ed25519>> {
        if self.ec_ops_accelerator.is_none() {
            self.ec_ops_accelerator = Some(EcOpAccelerator::<Ed25519ScalarField> {
                ec_op_requests: Vec::new(),
                ec_op_responses: Vec::new(),
            });
        }

        let mut result: Option<AffinePointVariable<Ed25519>> = None;
        match &request.req_type() {
            EcOpRequestType::Add | EcOpRequestType::ScalarMul | EcOpRequestType::Decompress => {
                result = Some(self.init_unsafe::<AffinePointVariable<Ed25519>>());
            }
            EcOpRequestType::IsValid => {}
        }

        let accelerator = self
            .ec_ops_accelerator
            .as_mut()
            .expect("sha256 accelerator should exist");

        accelerator.ec_op_requests.push(request);
        accelerator.ec_op_responses.push(result.clone());

        result
    }
}
