use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::local::LocalProver;
use super::remote::RemoteProver;
use super::{Prover, ProverInputTargets, ProverInputValues};

/// A prover which uses the enviroment variable `PROVER` to decide which prover to use.
pub struct EnviromentProver {}

impl Prover for EnviromentProver {
    fn new() -> Self {
        Self {}
    }

    async fn prove<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        targets: ProverInputTargets<D>,
        values: ProverInputValues<F, C, D>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let prover_type = env::var("PROVER").unwrap_or("local".to_string());
        let proof = if prover_type == "remote" {
            let prover = RemoteProver::new();
            prover.prove(&circuit, targets, values).await
        } else {
            let prover = LocalProver::new();
            prover.prove(&circuit, targets, values).await
        };
        proof
    }

    async fn prove_batch<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        targets: ProverInputTargets<D>,
        values: Vec<ProverInputValues<F, C, D>>,
    ) -> Vec<ProofWithPublicInputs<F, C, D>>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let prover_type = env::var("PROVER").unwrap_or("local".to_string());
        let proofs = if prover_type == "remote" {
            let prover = RemoteProver::new();
            prover.prove_batch(&circuit, targets, values).await
        } else {
            let prover = LocalProver::new();
            prover.prove_batch(&circuit, targets, values).await
        };
        proofs
    }
}
