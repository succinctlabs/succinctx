use anyhow::Result;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::backend::circuit::{CircuitBuild, PlonkParameters, PublicInput, PublicOutput};

/// A prover that generates proofs locally.
#[derive(Debug, Clone)]
pub struct LocalProver;

impl LocalProver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    #[allow(clippy::type_complexity)]
    pub fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        input: &PublicInput<L, D>,
    ) -> Result<(
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        Ok(circuit.prove(input))
    }

    /// Generates a batch of proofs with the given input.
    #[allow(clippy::type_complexity)]
    pub fn batch_prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        inputs: &[PublicInput<L, D>],
    ) -> Result<(
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<PublicOutput<L, D>>,
    )>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut proofs = Vec::new();
        let mut outputs = Vec::new();
        for input in inputs {
            let (proof, output) = self.prove(circuit, input)?;
            proofs.push(proof);
            outputs.push(output);
        }
        Ok((proofs, outputs))
    }
}
