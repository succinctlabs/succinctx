use num::BigUint;
use serde::{Deserialize, Serialize};

use super::request::EcOpRequestType;
use super::stark::{Ed25519CurtaOpValue, Ed25519Stark};
use super::Curve;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::Uint;
use crate::prelude::{PlonkParameters, U256Variable, ValueStream};

/// Provides a STARK proof for a set of EC operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcOpProofHint {
    requests: Vec<EcOpRequestType>,
}

impl EcOpProofHint {
    /// Creates a new proof hint for a set of EC operations.
    pub fn new(requests: &[EcOpRequestType]) -> Self {
        Self {
            requests: requests.to_vec(),
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for EcOpProofHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        // Read inputs.
        let input = self
            .requests
            .iter()
            .map(|req| match req {
                EcOpRequestType::Add => {
                    let a = input_stream.read_value::<AffinePointVariable<Curve>>();
                    let b = input_stream.read_value::<AffinePointVariable<Curve>>();
                    let result = input_stream.read_value::<AffinePointVariable<Curve>>();
                    Ed25519CurtaOpValue::Add(a, b, result)
                }
                EcOpRequestType::ScalarMul => {
                    let scalar = BigUint::new(
                        input_stream
                            .read_value::<U256Variable>()
                            .to_u32_limbs()
                            .to_vec(),
                    );
                    let point = input_stream.read_value::<AffinePointVariable<Curve>>();
                    let result = input_stream.read_value::<AffinePointVariable<Curve>>();
                    Ed25519CurtaOpValue::ScalarMul(scalar, point, result)
                }
                EcOpRequestType::Decompress => {
                    let compressed = input_stream.read_value::<CompressedEdwardsYVariable>();
                    let result = input_stream.read_value::<AffinePointVariable<Curve>>();
                    Ed25519CurtaOpValue::Decompress(compressed, result)
                }
                EcOpRequestType::IsValid => {
                    let point = input_stream.read_value::<AffinePointVariable<Curve>>();
                    Ed25519CurtaOpValue::IsValid(point)
                }
            })
            .collect::<Vec<_>>();

        // Initialize the STARK.
        let stark = Ed25519Stark::<L, D>::new(&self.requests);

        // Create the proof.
        let (proof, public_inputs) = stark.prove(&input);

        // Write outputs.
        output_stream.write_emulated_stark_proof(proof);
        output_stream.write_slice(&public_inputs)
    }
}
