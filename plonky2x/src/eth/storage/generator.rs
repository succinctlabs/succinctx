use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::vars::{BoolVariable, Bytes32Variable, U256Variable};
use crate::eth::types::{AddressVariable};
use super::types::{AccountVariable, ProofVariable};


#[derive(Debug)]
pub struct GetStorageProofGenerator<F: RichField + Extendable<D>, const D: usize> {
    address: AddressVariable,
    storage_key: Bytes32Variable,
    account_value: AccountVariable,
    account_proof: ProofVariable,
    storage_proof: ProofVariable,
    value: Bytes32Variable,
    block_number: u64,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> GetStorageProofGenerator<F,D> {
    pub fn new(address: AddressVariable, storage_key: Bytes32Variable, account_value: AccountVariable, account_proof: ProofVariable, storage_proof: ProofVariable, value: Bytes32Variable, block_number: u64) -> GetStorageProofGenerator<F, D> {
        return GetStorageProofGenerator{
            address,
            storage_key,
            account_value,
            account_proof,
            storage_proof,
            value: value,
            block_number,
            _phantom: PhantomData::<F>,
        };
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for GetStorageProofGenerator<F, D>
{
    fn id(&self) -> String {
        "GetStorageProofGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        // TODO: inlcude address, storage_key
        vec![]
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {

        // let header_root_bits = self
        //     .header_root
        //     .map(|x| witness.get_target(x.0 .0) == F::ONE);
        // let header_root = hex::encode(le_bits_to_bytes::<256>(header_root_bits));
        // println!("{}", header_root);
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
