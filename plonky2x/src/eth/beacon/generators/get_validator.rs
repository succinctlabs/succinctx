use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::eth::beacon::validator::BeaconValidatorWitnessWrite;
use crate::eth::beacon::BeaconValidatorVariable;
use crate::ethutils::beacon::BeaconClient;
use crate::vars::{Bytes32Variable, ReadableWitness};

#[derive(Debug)]
struct GetBeaconValidatorGenerator<F: RichField + Extendable<D>, const D: usize> {
    block_root: Bytes32Variable,
    validator_idx: u64,
    validator: BeaconValidatorVariable,
    client: BeaconClient,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for GetBeaconValidatorGenerator<F, D>
{
    fn id(&self) -> String {
        "GetBeaconValidatorGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        vec![]
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = witness.get_hex_string(self.block_root.into());
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt
            .block_on(self.client.get_validator(block_root, self.validator_idx))
            .unwrap();
        let value = result.validator;
        out_buffer.set_validator(self.validator, value);
        // witness.get_bits
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}
