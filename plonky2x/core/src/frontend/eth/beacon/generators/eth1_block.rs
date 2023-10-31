use std::env;

use async_trait::async_trait;
use ethers::types::{H256, U256};
use serde::{Deserialize, Serialize};

use crate::frontend::hint::asynchronous::hint::AsyncHint;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{U256Variable, ValueStream};
use crate::prelude::{ArrayVariable, Bytes32Variable, PlonkParameters};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::eth::beaconchain::BeaconchainAPIClient;
use crate::utils::{bytes32, hex};

/// Input: (eth1_block_number: u256)
/// Output: (slot: u64)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eth1BlockToSlotHint {}

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for Eth1BlockToSlotHint {
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let client = BeaconchainAPIClient::new(
            env::var("BEACONCHAIN_API_URL_1").unwrap(),
            env::var("BEACONCHAIN_API_KEY_1").unwrap(),
        );
        let eth1_block = input_stream.read_value::<U256Variable>();

        // Convert block numbers to slots
        let execution_blocks = client.get_execution_blocks(&[eth1_block]).await.unwrap();
        let slot = execution_blocks[0].pos_consensus.slot;
        output_stream.write_value::<U64Variable>(slot);
    }
}

const DEPTH: usize = 11;

/// Input: (block_root: bytes32)
/// Output: (proof: ArrayVariable<Bytes32Variable, DEPTH>, eth1_block_number: u256)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconExecutionPayloadHint {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BeaconExecutionPayloadHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());

        let block_root = input_stream.read_value::<Bytes32Variable>();
        let execution_payload = client
            .get_execution_payload(hex!(block_root.to_fixed_bytes()))
            .unwrap();

        let proof = execution_payload
            .proof
            .iter()
            .map(|h| bytes32!(h))
            .collect::<Vec<H256>>();
        let eth1_block_number = U256::from_dec_str(&execution_payload.block_number).unwrap();

        output_stream.write_value::<ArrayVariable<Bytes32Variable, DEPTH>>(proof);
        output_stream.write_value::<U256Variable>(eth1_block_number);
    }
}
