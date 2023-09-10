use plonky2::plonk::proof::ProofWithPublicInputs;

use super::Prover;
use crate::backend::circuit::io::{CircuitInput, CircuitOutput};
use crate::backend::circuit::Circuit;
use crate::backend::config::PlonkParameters;

/// A prover that generates proofs locally.
pub struct LocalProver;

impl Prover for LocalProver {
    fn new() -> Self {
        Self {}
    }

    async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &Circuit<L, D>,
        input: &CircuitInput<L, D>,
    ) -> (
        ProofWithPublicInputs<L::Field, L::Config, D>,
        CircuitOutput<L, D>,
    ) {
        circuit.prove(input)
    }
}
