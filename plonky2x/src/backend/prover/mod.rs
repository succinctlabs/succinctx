mod env;
mod local;
mod remote;
mod service;

use anyhow::Result;
pub use env::EnvProver;
pub use local::LocalProver;
use plonky2::plonk::proof::ProofWithPublicInputs;
pub use remote::RemoteProver;
pub use service::ProofService;

use super::circuit::{CircuitBuild, PlonkParameters, PublicInput, PublicOutput};

/// Basic methods for generating proofs from circuits.
pub trait Prover {
    /// Creates a new instance of the prover.
    fn new() -> Self;

    /// Generates a proof with the given input.
    async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        input: &PublicInput<L, D>,
    ) -> Result<(
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )>;

    /// Generates a batch of proofs with the given input.
    async fn batch_prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &CircuitBuild<L, D>,
        inputs: &[PublicInput<L, D>],
    ) -> Result<(
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<PublicOutput<L, D>>,
    )> {
        let mut proofs = Vec::new();
        let mut outputs = Vec::new();
        for input in inputs {
            let (proof, output) = self.prove(circuit, input).await?;
            proofs.push(proof);
            outputs.push(output);
        }
        Ok((proofs, outputs))
    }
}
