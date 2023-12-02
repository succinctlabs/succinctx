use std::env;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::vars::{Bytes32Variable, ValueStream};
use crate::prelude::ArrayVariable;
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const NB_BLOCK_ROOTS: usize = 8192;
const DEPTH: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconBlockRootsHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BeaconBlockRootsHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let header_root = input_stream.read_value::<Bytes32Variable>();
        let response = client.get_block_roots(hex!(header_root)).unwrap();
        output_stream.write_value::<Bytes32Variable>(bytes32!(response.block_roots_root));
        output_stream.write_value::<ArrayVariable<Bytes32Variable, DEPTH>>(
            response.proof.iter().map(|p| bytes32!(p)).collect_vec(),
        );
        output_stream.write_value::<ArrayVariable<Bytes32Variable, NB_BLOCK_ROOTS>>(
            response
                .block_roots
                .iter()
                .map(|p| bytes32!(p))
                .collect_vec(),
        );
    }
}
