use std::env;

use ethers::types::U64;
use serde::{Deserialize, Serialize};

use crate::frontend::eth::beacon::vars::{BeaconHeaderValue, BeaconHeaderVariable};
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::vars::ValueStream;
use crate::prelude::{Bytes32Variable, PlonkParameters};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

/// Input: (block_root: bytes32)
/// Output: (slot: u64, proposerIndex: u64, parentRoot: bytes32, stateRoot: bytes32, bodyRoot: bytes32)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconHeaderHint {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BeaconHeaderHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());
        let block_root = input_stream.read_value::<Bytes32Variable>();

        let header = client.get_header(hex!(block_root.as_bytes())).unwrap();

        let beacon_header = BeaconHeaderValue::<L::Field> {
            slot: U64::from_dec_str(header.slot.as_str()).unwrap(),
            proposer_index: U64::from_dec_str(header.proposer_index.as_str()).unwrap(),
            parent_root: bytes32!(header.parent_root),
            state_root: bytes32!(header.state_root),
            body_root: bytes32!(header.body_root),
        };

        output_stream.write_value::<BeaconHeaderVariable>(beacon_header);
    }
}
