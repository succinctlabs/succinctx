use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

pub mod enviroment;
pub mod local;
pub mod remote;
pub mod service;

/// Basic methods for proving circuits that are shared between both local and remote
/// implementations.
pub trait Prover {
    /// Creates a new instance of the prover.
    fn new() -> Self;

    /// Generates a proof with the given input.
    async fn prove<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        targets: ProverInputTargets<D>,
        values: ProverInputValues<F, C, D>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>;

    /// Generates a batch of proofs with the given input.
    async fn prove_batch<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        targets: ProverInputTargets<D>,
        values: Vec<ProverInputValues<F, C, D>>,
    ) -> Vec<ProofWithPublicInputs<F, C, D>>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>;
}
