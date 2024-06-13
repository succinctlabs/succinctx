use std::env;

use anyhow::Result;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use super::local::LocalProver;
use super::{ProverOutput, ProverOutputs, RemoteProver};
use crate::backend::circuit::{CircuitBuild, CircuitSerializer, PlonkParameters, PublicInput};

/// A prover that can generate proofs locally or remotely based on the env variable `PROVER` which
/// can either be `remote` or `local`.
pub struct EnvProver;

impl EnvProver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn prove<L: PlonkParameters<D>, S: CircuitSerializer, const D: usize>(
        &self,
        circuit_id: &str,
        input: &PublicInput<L, D>,
    ) -> Result<ProverOutput<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        if env::var("PROVER").unwrap_or("local".to_string()) == "remote" {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { RemoteProver::new().prove(circuit_id, input).await })
        } else {
            let gate_serializer = S::gate_registry::<L, D>();
            let generator_serializer = S::generator_registry::<L, D>();
            let build_dir = env::var("BUILD_DIR").unwrap_or_else(|_| "./build".to_string());
            let circuit_path = format!("{}/{}.circuit", build_dir, circuit_id);
            let circuit =
                CircuitBuild::<L, D>::load(&circuit_path, &gate_serializer, &generator_serializer)
                    .unwrap();
            LocalProver::new().prove(&circuit, input)
        }
    }

    pub fn batch_prove<L: PlonkParameters<D>, S: CircuitSerializer, const D: usize>(
        &self,
        circuit_id: &str,
        inputs: &[PublicInput<L, D>],
    ) -> Result<ProverOutputs<L, D>>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        if env::var("PROVER").unwrap_or("local".to_string()) == "remote" {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { RemoteProver::new().batch_prove(circuit_id, inputs).await })
        } else {
            let gate_serializer = S::gate_registry::<L, D>();
            let generator_serializer = S::generator_registry::<L, D>();
            let build_dir = env::var("BUILD_DIR").unwrap_or_else(|_| "./build".to_string());
            let circuit_path = format!("{}/{}.circuit", build_dir, circuit_id);
            let circuit =
                CircuitBuild::<L, D>::load(&circuit_path, &gate_serializer, &generator_serializer)
                    .unwrap();
            LocalProver::new().batch_prove(&circuit, inputs)
        }
    }
}
