use std::env;

use ethers::types::{H160, U256, U64};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::frontend::eth::beacon::vars::{BeaconWithdrawalValue, BeaconWithdrawalVariable};
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::vars::ValueStream;
use crate::prelude::{ArrayVariable, Bytes32Variable, PlonkParameters};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::hex;

const MAX_WITHDRAWALS_PER_PAYLOAD: usize = 16;

/// Input: (block_root: bytes32)
/// Output: (withdrawals: ArrayVariable<BeaconWithdrawalVariable, MAX_WITHDRAWALS_PER_PAYLOAD>)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconAllWithdrawalsHint {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BeaconAllWithdrawalsHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());

        let block_root = input_stream.read_value::<Bytes32Variable>();
        let withdrawals_res = client.get_withdrawals(hex!(block_root)).unwrap();
        let withdrawals = withdrawals_res
            .withdrawals
            .iter()
            .map(|w| BeaconWithdrawalValue {
                index: U64::from(w.index),
                validator_index: U64::from(w.validator_index),
                address: H160::from_slice(
                    &hex::decode(w.address.strip_prefix("0x").unwrap()).unwrap(),
                ),
                amount: U256::from_big_endian(&w.amount.to_bytes_be().1),
            })
            .collect::<Vec<BeaconWithdrawalValue<L::Field>>>();

        debug!("Withdrawals: {:?}", withdrawals);

        output_stream
            .write_value::<ArrayVariable<BeaconWithdrawalVariable, MAX_WITHDRAWALS_PER_PAYLOAD>>(
                withdrawals,
            )
    }
}
