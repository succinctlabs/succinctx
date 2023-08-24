use core::fmt::Debug;
use core::marker::PhantomData;

use ethers::providers::{JsonRpcClient, Middleware, Provider};
use ethers::types::EIP1186ProofResponse;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
use tokio::runtime::Runtime;

use super::vars::{EthAccountVariable, EthProofVariable};
use crate::builder::CircuitBuilder;
use crate::eth::utils::u256_to_h256_be;
use crate::eth::vars::AddressVariable;
use crate::vars::{Bytes32Variable, CircuitVariable};

#[derive(Debug, Clone)]
pub struct EthStorageProofGenerator<
    P: Clone + JsonRpcClient + 'static,
    F: RichField + Extendable<D>,
    const D: usize,
> {
    pub address: AddressVariable,
    pub storage_key: Bytes32Variable,
    pub account: EthAccountVariable,
    pub account_proof: EthProofVariable,
    pub storage_proof: EthProofVariable,
    pub value: Bytes32Variable,
    pub block_number: u64,
    pub provider: Provider<P>,
    _phantom: PhantomData<F>,
}

impl<P: Clone + JsonRpcClient + 'static, F: RichField + Extendable<D>, const D: usize>
    EthStorageProofGenerator<P, F, D>
{
    pub fn new(
        builder: &mut CircuitBuilder<F, D>,
        provider: Provider<P>,
        address: AddressVariable,
        storage_key: Bytes32Variable,
        block_number: u64,
    ) -> EthStorageProofGenerator<P, F, D> {
        let account = builder.init::<EthAccountVariable>();
        let account_proof = builder.init::<EthProofVariable>();
        let storage_proof = builder.init::<EthProofVariable>();
        let value = builder.init::<Bytes32Variable>();
        EthStorageProofGenerator {
            address,
            storage_key,
            account,
            account_proof,
            storage_proof,
            value,
            block_number,
            provider,
            _phantom: PhantomData::<F>,
        }
    }
}

impl<P: Clone + JsonRpcClient + 'static, F: RichField + Extendable<D>, const D: usize>
    SimpleGenerator<F, D> for EthStorageProofGenerator<P, F, D>
{
    fn id(&self) -> String {
        "GetStorageProofGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        vec![self.address.targets(), self.storage_key.targets()]
            .into_iter()
            .flatten()
            .collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, buffer: &mut GeneratedValues<F>) {
        let address = self.address.value(witness);
        let location = self.storage_key.value(witness);
        let get_proof_closure = || -> EIP1186ProofResponse {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                self.provider
                    .get_proof(address, vec![location], Some(self.block_number.into()))
                    .await
                    .unwrap()
            })
        };
        let storage_result: EIP1186ProofResponse = get_proof_closure();
        let value = u256_to_h256_be(storage_result.storage_proof[0].value);
        self.value.set(buffer, value);
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
