use std::env;

use async_trait::async_trait;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hint::asynchronous::hint::AsyncHint;
use crate::frontend::vars::{Bytes32Variable, ValueStream};
use crate::prelude::ArrayVariable;
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 7;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconGraffitiHint;

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for BeaconGraffitiHint {
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());
        let header_root = input_stream.read_value::<Bytes32Variable>();
        let response = client.get_graffiti(hex!(header_root)).await.unwrap();
        output_stream.write_value::<Bytes32Variable>(bytes32!(response.graffiti));
        output_stream.write_value::<ArrayVariable<Bytes32Variable, DEPTH>>(
            response.proof.iter().map(|p| bytes32!(p)).collect_vec(),
        );
    }
}
