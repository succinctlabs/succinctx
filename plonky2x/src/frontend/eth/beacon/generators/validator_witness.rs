use std::env;

use serde::{Deserialize, Serialize};

use crate::frontend::eth::beacon::vars::BeaconValidatorVariable;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::ValueStream;
use crate::prelude::{Bytes32Variable, PlonkParameters};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconValidatorWitnessGenerator {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for BeaconValidatorWitnessGenerator {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap());
        let header_root = input_stream.read_value::<Bytes32Variable>();
        let validator_index = input_stream.read_value::<U64Variable>();

        let response = client
            .get_validator_witness(hex!(header_root), validator_index.as_u64())
            .unwrap();

        output_stream.write_value::<BeaconValidatorVariable>(response.validator);
    }
}
