use std::env;

use async_trait::async_trait;
use ethers::types::U256;
use serde::{Deserialize, Serialize};

use crate::frontend::eth::beacon::vars::BeaconValidatorVariable;
use crate::frontend::hint::asynchronous::hint::AsyncHint;
use crate::prelude::{
    BoolVariable, Bytes32Variable, PlonkParameters, U256Variable, U64Variable, ValueStream,
};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconValidatorSubtreesHint<const B: usize, const N: usize> {}

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize, const B: usize, const N: usize> AsyncHint<L, D>
    for BeaconValidatorSubtreesHint<B, N>
{
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let block_root = input_stream.read_value::<Bytes32Variable>();

        let response = client
            .get_validator_subtrees(B, N, hex!(block_root))
            .await
            .expect("failed to get validator subtrees");

        for i in 0..N / B {
            output_stream.write_value::<Bytes32Variable>(bytes32!(response[i]));
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconValidatorSubtreeHint<const B: usize, const N: usize> {}

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize, const B: usize, const N: usize> AsyncHint<L, D>
    for BeaconValidatorSubtreeHint<B, N>
{
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let block_root = input_stream.read_value::<Bytes32Variable>();

        let response = client
            .get_validator_subtree(B, N, hex!(block_root))
            .await
            .expect("failed to get validators root");

        for i in 0..B {
            output_stream.write_value::<BeaconValidatorVariable>(response[i].clone());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconValidatorSubtreePoseidonHint<const B: usize> {}

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize, const B: usize> AsyncHint<L, D>
    for BeaconValidatorSubtreePoseidonHint<B>
{
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let block_root = input_stream.read_value::<Bytes32Variable>();
        let withdrawal_credentials = input_stream.read_value::<Bytes32Variable>();
        let start_index = input_stream.read_value::<U64Variable>();

        let response = client
            .get_validator_batch_witness(hex!(block_root), start_index, start_index + B as u64)
            .expect("failed to get validators subtree");

        let validators = response.iter().collect::<Vec<_>>();
        let records = validators
            .iter()
            .map(|v| {
                (
                    bytes32!(v.withdrawal_credentials) == withdrawal_credentials,
                    U256::from_dec_str(&v.exit_epoch).unwrap(),
                )
            })
            .collect::<Vec<_>>();
        for i in 0..B {
            output_stream.write_value::<(BoolVariable, U256Variable)>(records[i]);
        }
    }
}
