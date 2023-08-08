use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, EIP1186ProofResponse};


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
    provider: Provider<Http>,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> GetStorageProofGenerator<F,D> {
    pub fn new(
        address: AddressVariable, 
        storage_key: Bytes32Variable, 
        account_value: AccountVariable, 
        account_proof: ProofVariable, 
        storage_proof: ProofVariable, 
        value: Bytes32Variable, 
        block_number: u64,
        provider: Provider<Http>
    ) -> GetStorageProofGenerator<F, D> {
        return GetStorageProofGenerator{
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

    fn get_storage(&self) -> EIP1186ProofResponse {
        // TODO construct blocking runtime
        let result = self.provider.get_proof(address, vec![location], Some(self.block_number.into())).await?;
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let address_bits = self
            .address.0.to_vec().iter().map(|x| witness.get_target(x.0) == F::ONE);
        let address = Address::from(le_bits_to_bytes::<20>(address_bits));
        let location = H256::from(le_bits_to_bytes::<32>(address_bits));
        let storageResult: EIP1186ProofResponse = self::get_storage(address, location);

        EIP1186ProofResponse
        // Now load the result in the variables
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
