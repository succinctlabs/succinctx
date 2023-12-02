use std::env;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hint::asynchronous::hint::AsyncHint;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{ArrayVariable, ValueStream};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

pub const CLOSE_SLOT_BLOCK_ROOT_DEPTH: usize = 21;
pub const FAR_SLOT_HISTORICAL_SUMMARY_DEPTH: usize = 33;
pub const FAR_SLOT_BLOCK_ROOT_DEPTH: usize = 14;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconHistoricalBlockHint {}

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for BeaconHistoricalBlockHint {
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let block_root = input_stream.read_value::<Bytes32Variable>();
        let target_slot = input_stream.read_value::<U64Variable>();

        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let result = client
            .get_historical_block(hex!(block_root.as_bytes()).to_string(), target_slot)
            .await
            .expect("failed to get validators root");

        output_stream.write_value::<Bytes32Variable>(bytes32!(result.target_block_root));
        output_stream.write_value::<ArrayVariable<Bytes32Variable, CLOSE_SLOT_BLOCK_ROOT_DEPTH>>(
            result
                .close_slot_block_root_proof
                .iter()
                .map(|x| bytes32!(*x))
                .collect(),
        );
        output_stream.write_value::<ArrayVariable<Bytes32Variable, FAR_SLOT_BLOCK_ROOT_DEPTH>>(
            result
                .far_slot_block_root_proof
                .iter()
                .map(|x| bytes32!(*x))
                .collect(),
        );
        output_stream
            .write_value::<Bytes32Variable>(bytes32!(result.far_slot_historical_summary_root));
        output_stream
            .write_value::<ArrayVariable<Bytes32Variable, FAR_SLOT_HISTORICAL_SUMMARY_DEPTH>>(
                result
                    .far_slot_historical_summary_proof
                    .iter()
                    .map(|x| bytes32!(*x))
                    .collect(),
            );
    }
}
