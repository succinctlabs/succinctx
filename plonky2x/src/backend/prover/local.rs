use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::Prover;
use crate::backend::circuit::io::{CircuitInput, CircuitOutput};
use crate::backend::circuit::Circuit;

/// A prover that generates proofs locally.
pub struct LocalProver {}

impl Prover for LocalProver {
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
        circuit.prove(input)
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
        inputs.iter().map(|input| circuit.prove(input)).collect()
    }
}
