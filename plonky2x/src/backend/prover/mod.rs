mod env;
mod local;
mod remote;
mod service;

use anyhow::Result;
pub use env::EnvProver;
pub use local::LocalProver;
use plonky2::plonk::proof::ProofWithPublicInputs;
pub use remote::RemoteProver;
pub use service::{BatchProofId, ProofId, ProofService};

use super::circuit::{PlonkParameters, PublicOutput};

#[allow(clippy::large_enum_variant)]
pub enum ProverOutput<L: PlonkParameters<D>, const D: usize> {
    Local(
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    ),
    Remote(ProofId),
}

pub enum ProverOutputs<L: PlonkParameters<D>, const D: usize> {
    Local(
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<PublicOutput<L, D>>,
    ),
    Remote(Vec<ProofId>),
}

impl<L: PlonkParameters<D>, const D: usize> ProverOutputs<L, D> {
    #[allow(clippy::type_complexity)]
    pub fn materialize(
        self,
    ) -> Result<(
        Vec<ProofWithPublicInputs<L::Field, L::Config, D>>,
        Vec<PublicOutput<L, D>>,
    )> {
        let (proofs, outputs) = match self {
            ProverOutputs::Local(proofs, outputs) => (proofs, outputs),
            ProverOutputs::Remote(proof_ids) => {
                let service = ProofService::new_from_env();
                let mut proofs = Vec::new();
                let mut outputs = Vec::new();
                for proof_id in proof_ids {
                    let response = service.get::<L, D>(proof_id).unwrap();
                    let (proof, output) = response.result.unwrap().as_proof_and_output();
                    proofs.push(proof);
                    outputs.push(output);
                }
                (proofs, outputs)
            }
        };
        Ok((proofs, outputs))
    }
}
