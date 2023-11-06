use curta::chip::field::parameters::FieldParameters;

use super::accelerator::EcOpAccelerator;
use super::proof_hint::EcOpProofHint;
use super::request::EcOpRequest;
use super::result_hint::EcOpResultHint;
use super::stark::Ed25519Stark;
use super::Curve;
use crate::frontend::curta::ec::point::AffinePointVariable;
use crate::frontend::hint::synchronous::Async;
use crate::prelude::{CircuitBuilder, PlonkParameters, VariableStream};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// The constraints for an accelerated EC Ops computation using Curta.
    pub(crate) fn curta_constrain_ec_op<FF: FieldParameters>(
        &mut self,
        accelerator: EcOpAccelerator<FF>,
    ) {
        // Get all the responses using the request hint.
        for (request, response) in accelerator
            .ec_op_requests
            .iter()
            .zip(accelerator.ec_op_responses.iter())
        {
            let result_hint = EcOpResultHint::<FF>::new(request.req_type());
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

            match &request {
                EcOpRequest::Add(_, _)
                | EcOpRequest::ScalarMul(_, _)
                | EcOpRequest::Decompress(_) => {
                    let result = output_stream.read::<AffinePointVariable<Curve>>(self);
                    self.assert_is_equal(
                        result,
                        response.clone().expect("response should not be None"),
                    );
                }
                EcOpRequest::IsValid(_) => {}
            }
        }

        let mut input_stream = VariableStream::new();

        let mut requests = Vec::new();
        for (request, response) in accelerator
            .ec_op_requests
            .iter()
            .zip(accelerator.ec_op_responses.iter())
        {
            requests.push(request.req_type());
            match &request {
                EcOpRequest::Add(a, b) => {
                    let response = response.as_ref().unwrap();
                    input_stream.write(&**a);
                    input_stream.write(&**b);
                    input_stream.write(response);
                }
                EcOpRequest::ScalarMul(scalar, point) => {
                    let response = response.as_ref().unwrap();
                    input_stream.write(&**scalar);
                    input_stream.write(&**point);
                    input_stream.write(response);
                }
                EcOpRequest::Decompress(compressed_point) => {
                    let point = response.as_ref().unwrap();
                    input_stream.write(&**compressed_point);
                    input_stream.write(point);
                }
                EcOpRequest::IsValid(point) => {
                    input_stream.write(&**point);
                }
            }
        }

        let proof_hint = EcOpProofHint::new(&requests);
        let output_stream = self.async_hint(input_stream, Async(proof_hint));

        let stark = Ed25519Stark::<L, D>::new(&requests);
        let (proof, public_inputs) = stark.read_proof_with_public_input(self, &output_stream);
        stark.verify_proof(self, proof, &public_inputs, &[])
    }
}
