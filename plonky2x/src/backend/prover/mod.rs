use plonky2::plonk::proof::ProofWithPublicInputs;

use super::circuit::input::{CircuitInput, CircuitOutput};
use super::circuit::Circuit;
use super::config::PlonkParameters;

// pub mod enviroment;
pub mod local;
pub mod remote;
// pub mod service;

/// Basic methods for generating proofs from circuits.
pub trait Prover {
    /// Creates a new instance of the prover.
    fn new() -> Self;

    /// Generates a proof with the given input.
    async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &Circuit<L, D>,
        input: &CircuitInput<L, D>,
    ) -> (
        ProofWithPublicInputs<L::Field, L::Config, D>,
        CircuitOutput<L, D>,
    );

    /// Generates a batch of proofs with the given input.
    async fn prove_batch<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &Circuit<L, D>,
        inputs: &[CircuitInput<L, D>],
    ) -> (
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<CircuitOutput<L, D>>,
    ) {
        let mut proofs = Vec::new();
        let mut outputs = Vec::new();
        for input in inputs {
            let (proof, output) = self.prove(circuit, input).await;
            proofs.push(proof);
            outputs.push(output);
        }
        (proofs, outputs)
    }
}
