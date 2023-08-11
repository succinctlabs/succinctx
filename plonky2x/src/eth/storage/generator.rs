use core::marker::PhantomData;

<<<<<<< HEAD
use ethers::types::{Address, H256, EIP1186ProofResponse};
=======
>>>>>>> main
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
<<<<<<< HEAD
use tokio::runtime::Runtime;

use ethers::providers::{Http, Provider, Middleware};

use crate::vars::{Bytes32Variable, WitnessMethods, WitnessWriteMethods, BytesVariable};
use crate::eth::types::{AddressVariable};
use crate::eth::utils::{u256_to_h256_be};
=======

use ethers::providers::{Http, Provider};

use crate::vars::{Bytes32Variable, WitnessMethods};
use crate::eth::types::{AddressVariable};
>>>>>>> main
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
        let mut res = BytesVariable::<20>::from(self.address).to_targets();
        res.append(&mut BytesVariable::<32>::from(self.storage_key).to_targets());
        res
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        println!("Running generator once");
        println!("address {:?}", self.address);

        let address = Address::from(witness.get_bytes_be(self.address.into()));
        println!("address {:?}", self.address);
        let location = H256::from(witness.get_bytes_be(self.storage_key.into()));
        let get_proof_closure = || -> EIP1186ProofResponse {
            let rt = Runtime::new().unwrap();
            rt.block_on(async { self.provider.get_proof(address, vec![location], Some(self.block_number.into())).await.unwrap() } )
        };
        let storage_result: EIP1186ProofResponse = get_proof_closure();
        println!("address {:?}", address);
        println!("proof response {:?}", storage_result);


        // let mut bytes32_value = [0u8; 32];
        // storageResult.storage_proof[0].value.to_big_endian(&mut bytes32_value);
        let h256_value = u256_to_h256_be(storage_result.storage_proof[0].value);
        println!("Value from witness gen {:?}", h256_value);
        out_buffer.set_from_bytes_be(self.value.into(), *h256_value.as_fixed_bytes());
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
