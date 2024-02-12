use starkyx::chip::ec::EllipticCurveParameters;

use super::accelerator::EcOpAccelerator;
use super::proof_hint::EcOpProofHint;
use super::request::{EcOpRequest, EcOpResponse};
use super::result_hint::EcOpResultHint;
use super::stark::{Ed25519OpVariable, Ed25519Stark};
use super::Curve;
use crate::frontend::curta::ec::point::AffinePointVariable;
use crate::frontend::curta::field::variable::FieldVariable;
use crate::frontend::hint::synchronous::Async;
use crate::prelude::{CircuitBuilder, PlonkParameters, VariableStream};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// The constraints for an accelerated EC Ops computation using Curta.
    pub(crate) fn curta_constrain_ec_op(&mut self, accelerator: EcOpAccelerator) {
        // Get all the responses using the request hint.
        for (request, response) in accelerator
            .ec_op_requests
            .iter()
            .zip(accelerator.ec_op_responses.iter())
        {
            let result_hint = EcOpResultHint::new(request.req_type());
            let mut input_stream = VariableStream::new();

            match &request {
                EcOpRequest::Add(a, b) => {
                    input_stream.write(&**a);
                    input_stream.write(&**b);
                }
                EcOpRequest::ScalarMul(scalar, point) => {
                    input_stream.write(&**scalar);
                    input_stream.write(&**point);
                }
                EcOpRequest::Decompress(compressed_point) => {
                    input_stream.write(&**compressed_point);
                }
                EcOpRequest::IsValid(point) => {
                    input_stream.write(&**point);
                }
            }

            let output_stream = self.hint(input_stream, result_hint);

            match response {
                EcOpResponse::Add(c) => {
                    let c_hint = output_stream.read_unsafe::<AffinePointVariable<Curve>>(self);
                    self.assert_is_equal(c_hint, c.clone());
                }
                EcOpResponse::ScalarMul(c) => {
                    let c_hint = output_stream.read_unsafe::<AffinePointVariable<Curve>>(self);
                    self.assert_is_equal(c_hint, c.clone());
                }
                EcOpResponse::Decompress(point, root) => {
                    let point_hint = output_stream.read_unsafe::<AffinePointVariable<Curve>>(self);
                    let root_hint = output_stream
                        .read::<FieldVariable<<Curve as EllipticCurveParameters>::BaseField>>(self);
                    self.assert_is_equal(point_hint, point.clone());
                    self.assert_is_equal(root_hint, root.clone());
                }
                EcOpResponse::IsValid => {}
            }
        }

        let mut input_stream = VariableStream::new();

        let mut requests = Vec::new();
        let mut ec_ops = Vec::new();
        for (request, response) in accelerator
            .ec_op_requests
            .iter()
            .zip(accelerator.ec_op_responses.iter())
        {
            requests.push(request.req_type());
            match (request, response) {
                (EcOpRequest::Add(a, b), EcOpResponse::Add(c)) => {
                    input_stream.write(a.as_ref());
                    input_stream.write(b.as_ref());
                    input_stream.write(c);
                    ec_ops.push(Ed25519OpVariable::Add(*a.clone(), *b.clone(), c.clone()))
                }
                (EcOpRequest::ScalarMul(scalar, point), EcOpResponse::ScalarMul(response)) => {
                    input_stream.write(scalar.as_ref());
                    input_stream.write(point.as_ref());
                    input_stream.write(response);
                    ec_ops.push(Ed25519OpVariable::ScalarMul(
                        *scalar.clone(),
                        *point.clone(),
                        response.clone(),
                    ))
                }
                (
                    EcOpRequest::Decompress(compressed_point),
                    EcOpResponse::Decompress(point, root),
                ) => {
                    input_stream.write(compressed_point.as_ref());
                    input_stream.write(point);
                    ec_ops.push(Ed25519OpVariable::Decompress(
                        compressed_point.clone(),
                        point.clone(),
                        root.clone(),
                    ))
                }
                (EcOpRequest::IsValid(point), EcOpResponse::IsValid) => {
                    input_stream.write(point.as_ref());
                    ec_ops.push(Ed25519OpVariable::IsValid(*point.clone()))
                }
                _ => panic!("invalid request/response pair"),
            }
        }

        let proof_hint = EcOpProofHint::new(&requests);
        let output_stream = self.async_hint(input_stream, Async(proof_hint));

        let stark = Ed25519Stark::<L, D>::new(&requests);
        let (proof, public_inputs) = stark.read_proof_with_public_input(self, &output_stream);
        stark.verify_proof(self, proof, &public_inputs, &ec_ops)
    }
}
