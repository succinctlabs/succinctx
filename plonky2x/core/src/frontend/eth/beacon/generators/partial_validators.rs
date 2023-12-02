use std::env;

use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::vars::{Bytes32Variable, ValueStream};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const VALIDATOR_REGISTRY_LIMIT_LOG2: usize = 40;
const DEPTH: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconPartialValidatorsHint<const B: usize> {}

impl<L: PlonkParameters<D>, const D: usize, const B: usize> Hint<L, D>
    for BeaconPartialValidatorsHint<B>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let header_root = input_stream.read_value::<Bytes32Variable>();
        let response = client
            .get_partial_validators_root(hex!(header_root), B)
            .unwrap();
        output_stream.write_value::<Bytes32Variable>(bytes32!(response.partial_validators_root));
        let nb_branches =
            DEPTH + (VALIDATOR_REGISTRY_LIMIT_LOG2 + 1 - ((B as f64).log2().ceil() as usize));
        assert_eq!(response.proof.len(), nb_branches);
        for i in 0..nb_branches {
            output_stream.write_value::<Bytes32Variable>(bytes32!(response.proof[i]));
        }
    }
}
