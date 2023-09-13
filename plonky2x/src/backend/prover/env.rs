use std::env;

use anyhow::Result;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::local::LocalProver;
use super::remote::RemoteProver;
use super::Prover;
use crate::backend::circuit::{CircuitBuild, PlonkParameters, PublicInput, PublicOutput};

/// A prover that can generate proofs locally or remotely based on the env variable `PROVER` which
/// can either be `remote` or `local`.
pub struct EnvProver;

impl Prover for EnvProver {
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
    )>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        if env::var("PROVER").unwrap_or("local".to_string()) == "remote" {
            RemoteProver::new().prove(circuit, input).await
        } else {
            LocalProver::new().prove(circuit, input).await
        }
    }

    async fn batch_prove<L: PlonkParameters<D>, const D: usize>(
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
        if env::var("PROVER").unwrap_or("local".to_string()) == "remote" {
            RemoteProver::new().batch_prove(circuit, inputs).await
        } else {
            LocalProver::new().batch_prove(circuit, inputs).await
        }
    }
}
