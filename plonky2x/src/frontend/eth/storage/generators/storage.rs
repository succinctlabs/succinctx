use core::fmt::Debug;
use core::marker::PhantomData;

use ethers::providers::Middleware;
use ethers::types::EIP1186ProofResponse;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::utils::u256_to_h256_be;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::get_provider;

#[derive(Debug, Clone)]
pub struct EthStorageProofGenerator<F: RichField + Extendable<D>, const D: usize> {
    address: AddressVariable,
    storage_key: Bytes32Variable,
    block_hash: Bytes32Variable,
    pub value: Bytes32Variable,
    chain_id: u64,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> EthStorageProofGenerator<F, D> {
    pub fn new(
        builder: &mut CircuitBuilder<F, D>,
        address: AddressVariable,
        storage_key: Bytes32Variable,
        block_hash: Bytes32Variable,
    ) -> EthStorageProofGenerator<F, D> {
        let chain_id = builder.get_chain_id();
        let value = builder.init::<Bytes32Variable>();
        EthStorageProofGenerator {
            address,
            storage_key,
            block_hash,
            value,
            chain_id,
            _phantom: PhantomData::<F>,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for EthStorageProofGenerator<F, D>
{
    fn id(&self) -> String {
        "GetStorageProofGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.address.targets());
        targets.extend(self.storage_key.targets());
        targets.extend(self.block_hash.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, buffer: &mut GeneratedValues<F>) {
        println!("Running generator at the top");
        let address = self.address.get(witness);
        let location = self.storage_key.get(witness);
        let block_hash = self.block_hash.get(witness);
        println!("Done with getting all variable");
        let provider = get_provider(self.chain_id);
        println!("Done with getting provider");
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result: EIP1186ProofResponse = rt.block_on(async {
            provider
                .get_proof(address, vec![location], Some(block_hash.into()))
                .await
                .expect("Failed to get proof")
        });
        println!("Done with getting result");
        let value = u256_to_h256_be(result.storage_proof[0].value);
        self.value.set(buffer, value);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        println!("Serializing generator");
        let chain_id_bytes = self.chain_id.to_be_bytes();
        dst.write_usize(8)?;
        dst.write_all(&chain_id_bytes)?;
        println!("Serializing generator after chain id");

        dst.write_target_vec(&self.block_hash.targets())?;
        dst.write_target_vec(&self.address.targets())?;
        dst.write_target_vec(&self.storage_key.targets())?;
        dst.write_target_vec(&self.value.targets())
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        println!("deserializing generator");
        let mut chain_id_bytes = [0u8; 8];
        src.read_exact(&mut chain_id_bytes)?;
        let chain_id = u64::from_be_bytes(chain_id_bytes);

        let block_hash_targets = src.read_target_vec()?;
        let block_hash = Bytes32Variable::from_targets(&block_hash_targets);

        let address_targets = src.read_target_vec()?;
        let address = AddressVariable::from_targets(&address_targets);

        let storage_key_targets = src.read_target_vec()?;
        let storage_key = Bytes32Variable::from_targets(&storage_key_targets);

        let value_targets = src.read_target_vec()?;
        let value = Bytes32Variable::from_targets(&value_targets);

        Ok(Self {
            address,
            storage_key,
            block_hash,
            value,
            chain_id,
            _phantom: PhantomData::<F>,
        })
    }
}
