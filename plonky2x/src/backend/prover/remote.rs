use plonky2::plonk::proof::ProofWithPublicInputs;
use reqwest::Client;

use super::Prover;
use crate::backend::circuit::input::{CircuitInput, CircuitOutput};
use crate::backend::circuit::{Circuit, PlonkParameters};
use crate::backend::function::io::BytesInput;
use crate::frontend::builder::CircuitIO;

/// A prover that generates proofs remotely on another machine.
///
/// The RELEASE_ID enviroment variable must be available so that the remote machine knows which
/// release to use.
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
        let release_id = std::env::var("RELEASE_ID").expect("RELEASE_ID not set");
        match input.io {
            CircuitIO::Evm(io) => {
                let data = BytesInput { input: io.input };
            }
        };
        todo!()
    }
}
