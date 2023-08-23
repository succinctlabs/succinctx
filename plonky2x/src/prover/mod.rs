use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};

pub mod enviroment;
pub mod local;
pub mod remote;
pub mod service;

/// The input targets to the prover. It can either be a list of targets or a list of recursive
/// proof targets.
#[derive(Debug, Clone)]
pub enum ProverInputTargets<const D: usize> {
    Targets(Vec<Target>),
    ProofTargets(Vec<ProofWithPublicInputsTarget<D>>),
}

impl<const D: usize> From<Vec<Target>> for ProverInputTargets<D> {
    fn from(targets: Vec<Target>) -> Self {
        Self::Targets(targets)
    }
}

impl<const D: usize> From<Vec<ProofWithPublicInputsTarget<D>>> for ProverInputTargets<D> {
    fn from(targets: Vec<ProofWithPublicInputsTarget<D>>) -> Self {
        Self::ProofTargets(targets)
    }
}

/// The input values to the prover. It can be either bytes, field elements or recursive proofs.
#[derive(Debug, Clone)]
pub enum ProverInputValues<F, C, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    Bytes(Vec<u8>),
    FieldElements(Vec<F>),
    Proofs(Vec<ProofWithPublicInputs<F, C, D>>),
}

impl<F, C, const D: usize> From<Vec<F>> for ProverInputValues<F, C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    fn from(elements: Vec<F>) -> Self {
        Self::FieldElements(elements)
    }
}

impl<F, C, const D: usize> From<Vec<ProofWithPublicInputs<F, C, D>>> for ProverInputValues<F, C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    fn from(proofs: Vec<ProofWithPublicInputs<F, C, D>>) -> Self {
        Self::Proofs(proofs)
    }
}

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
