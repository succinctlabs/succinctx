use core::marker::PhantomData;

use ethers::providers::{Http, Provider};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use super::types::{AccountVariable, ProofVariable};
use crate::eth::vars::AddressVariable;
use crate::vars::{Bytes32Variable, ReadableWitness};

#[derive(Debug)]
pub struct GetStorageProofGenerator<F: RichField + Extendable<D>, const D: usize> {
    address: AddressVariable,
    storage_key: Bytes32Variable,
    account_value: AccountVariable,
    account_proof: ProofVariable,
    storage_proof: ProofVariable,
    value: Bytes32Variable,
    block_number: u64,
    provider: Provider<Http>,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> GetStorageProofGenerator<F, D> {
    pub fn new(
        address: AddressVariable,
        storage_key: Bytes32Variable,
        account_value: AccountVariable,
        account_proof: ProofVariable,
        storage_proof: ProofVariable,
        value: Bytes32Variable,
        block_number: u64,
        provider: Provider<Http>,
    ) -> GetStorageProofGenerator<F, D> {
        return GetStorageProofGenerator {
            address,
            storage_key,
            account_value,
            account_proof,
            storage_proof,
            value: value,
            block_number,
            provider,
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

    fn run_once(&self, witness: &PartitionWitness<F>, _out_buffer: &mut GeneratedValues<F>) {
        // witness.get_bits_le(self.address.into());
        // let address = Address::from(self.address.get_bytes_le(witness));
        // let location = H256::from(self.storage_key.get_bytes_le(witness));
        // let get_proof_closure = || -> EIP1186ProofResponse {
        //     let rt = Runtime::new().unwrap();
        //     rt.block_on(async { self.provider.get_proof(address, vec![location], Some(self.block_number.into())).await.unwrap() } )
        // };
        // let storageResult: EIP1186ProofResponse = get_proof_closure();

        // let mut bytes32_value = [0u8; 32];
        // storageResult.storage_proof[0].value.to_big_endian(&mut bytes32_value);
        // self.value.set_from_bytes(bytes32_value, out_buffer);
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
