use std::env;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, ValueStream};
use crate::prelude::ArrayVariable;
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconHeadersFromOffsetRangeHint<const B: usize>;

impl<L: PlonkParameters<D>, const D: usize, const B: usize> Hint<L, D>
    for BeaconHeadersFromOffsetRangeHint<B>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let client = BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap());
        let header_root = input_stream.read_value::<Bytes32Variable>();
        let start_offset = input_stream.read_value::<U64Variable>();
        let end_offset = input_stream.read_value::<U64Variable>() + 1;
        let response = client
            .get_headers_from_offset_range(hex!(header_root), start_offset, end_offset)
            .unwrap();
        output_stream.write_value::<ArrayVariable<Bytes32Variable, B>>(
            response.headers.iter().map(|h| bytes32!(h)).collect_vec(),
        );
    }
}
