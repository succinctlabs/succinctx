use core::time::Duration;
use std::env;

use plonky2::plonk::proof::ProofWithPublicInputs;
use reqwest::Client;
use tokio::time::sleep;

use super::Prover;
use crate::backend::circuit::io::{CircuitInput, CircuitOutput};
use crate::backend::circuit::Circuit;
use crate::backend::config::PlonkParameters;

/// A prover that generates proofs remotely on another machine. To use the remote prover,
/// the RELEASE_ID enviroment variable must be available so that the remote machine knows
/// which release to use.
pub struct RemoteProver {
    pub client: Client,
}

impl Prover for RemoteProver {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn prove<L: PlonkParameters<D>, const D: usize>(
        &self,
        circuit: &Circuit<L, D>,
        input: &CircuitInput<L, D>,
    ) -> (
        ProofWithPublicInputs<L::Field, L::Config, D>,
        CircuitOutput<L, D>,
    ) {
       
    }
}
