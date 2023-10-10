use anyhow::Result;
use log::debug;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use super::{ProverOutput, ProverOutputs};
use crate::backend::circuit::{CircuitBuild, PlonkParameters, PublicInput};

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
    ) -> Result<ProverOutput<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let (proof, output) = circuit.prove(input);
        Ok(ProverOutput::Local(proof, output))
    }

    /// Generates a batch of proofs with the given input.
    #[allow(clippy::type_complexity)]
    pub fn batch_prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        inputs: &[PublicInput<L, D>],
    ) -> Result<ProverOutputs<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut outputs = Vec::new();
        for input in inputs {
            debug!("batch_prove: circuit_id={}", circuit.id());
            let output = self.prove(circuit, input)?;
            outputs.push(output);
        }
        let (proofs, outputs) = outputs
            .into_iter()
            .map(|x| match x {
                ProverOutput::Local(proof, output) => (proof, output),
                _ => unreachable!(),
            })
            .unzip();
        Ok(ProverOutputs::Local(proofs, outputs))
    }
}
