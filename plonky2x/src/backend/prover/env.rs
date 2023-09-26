use std::env;

use anyhow::Result;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use super::local::LocalProver;
use super::{ProverOutput, ProverOutputs, RemoteProver};
use crate::backend::circuit::{CircuitBuild, PlonkParameters, PublicInput};

/// A prover that can generate proofs locally or remotely based on the env variable `PROVER` which
/// can either be `remote` or `local`.
pub struct EnvProver;

impl EnvProver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        input: &PublicInput<L, D>,
    ) -> Result<ProverOutput<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        if env::var("PROVER").unwrap_or("local".to_string()) == "remote" {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { RemoteProver::new().prove(circuit, input).await })
        } else {
            LocalProver::new().prove(circuit, input)
        }
    }

    pub fn batch_prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        inputs: &[PublicInput<L, D>],
    ) -> Result<ProverOutputs<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        if env::var("PROVER").unwrap_or("local".to_string()) == "remote" {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { RemoteProver::new().batch_prove(circuit, inputs).await })
        } else {
            LocalProver::new().batch_prove(circuit, inputs)
        }
    }
}
