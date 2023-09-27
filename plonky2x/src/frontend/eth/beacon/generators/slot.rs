use std::env;

use ethers::types::{H256, U64};
use serde::{Deserialize, Serialize};

use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::ValueStream;
use crate::prelude::{ArrayVariable, Bytes32Variable, PlonkParameters};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 3;

/// Input: (block_root: bytes32)
/// Output: (proof: ArrayVariable<Bytes32Variable, DEPTH>, slot: u64)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconSlotHint {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BeaconSlotHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());

        let block_root = input_stream.read_value::<Bytes32Variable>();
        let slot_number_res = client.get_slot_number(hex!(block_root)).unwrap();

        let proof = slot_number_res
            .proof
            .iter()
            .map(|h| bytes32!(h))
            .collect::<Vec<H256>>();
        let slot = U64::from(slot_number_res.slot);

        output_stream.write_value::<ArrayVariable<Bytes32Variable, DEPTH>>(proof);
        output_stream.write_value::<U64Variable>(slot);
    }
}
