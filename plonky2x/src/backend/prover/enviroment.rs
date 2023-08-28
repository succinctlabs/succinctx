use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::local::LocalProver;
use super::remote::RemoteProver;
use super::Prover;
use crate::backend::circuit::io::{CircuitInput, CircuitOutput};
use crate::backend::circuit::Circuit;

/// A prover which uses the enviroment variable `PROVER` to decide which prover to use.
pub struct EnviromentProver {}

impl Prover for EnviromentProver {
    fn new() -> Self {
        Self {}
    }

    async fn prove<F, C, const D: usize>(
        &self,
        circuit: &Circuit<F, C, D>,
        input: &CircuitInput<F, D>,
    ) -> (ProofWithPublicInputs<F, C, D>, CircuitOutput<F, D>)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let prover_type = env::var("PROVER").unwrap_or("local".to_string());
        if prover_type == "remote" {
            let prover = RemoteProver::new();
            prover.prove(circuit, input).await
        } else {
            let prover = LocalProver::new();
            prover.prove(circuit, input).await
        }
    }

    async fn prove_batch<F, C, const D: usize>(
        &self,
        circuit: &Circuit<F, C, D>,
        inputs: Vec<CircuitInput<F, D>>,
    ) -> Vec<(ProofWithPublicInputs<F, C, D>, CircuitOutput<F, D>)>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let prover_type = env::var("PROVER").unwrap_or("local".to_string());
        if prover_type == "remote" {
            let prover = RemoteProver::new();
            prover.prove_batch(circuit, inputs).await
        } else {
            let prover = LocalProver::new();
            prover.prove_batch(circuit, inputs).await
        }
    }
}
