use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::Prover;

struct LocalProver {}

impl Prover for LocalProver {
    fn new() -> Self {
        Self {}
    }

    async fn prove<F, C, const D: usize>(
        &self,
        circuit: CircuitData<F, C, D>,
        witness: PartialWitness<F>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        circuit.prove(witness).unwrap()
    }
}
