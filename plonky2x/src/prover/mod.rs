use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

pub mod local;
pub mod remote;

pub trait Prover {
    fn new() -> Self;

    async fn prove<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        input: Vec<F>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>;

    async fn prove_reduce<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        input: Vec<ProofWithPublicInputs<F, C, D>>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>;
}
