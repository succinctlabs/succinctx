use anyhow::Result;
use async_trait::async_trait;
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::Prover;
use crate::backend::circuit::{CircuitBuild, PlonkParameters, PublicInput, PublicOutput};

/// A prover that generates proofs locally.
#[derive(Debug, Clone)]
pub struct LocalProver;

#[async_trait]
impl Prover for LocalProver {
    fn new() -> Self {
        Self {}
    }

    async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        input: &PublicInput<L, D>,
    ) -> Result<(
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )> {
        Ok(circuit.prove(input))
    }
}
