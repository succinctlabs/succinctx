use core::fmt::Debug;
use core::marker::PhantomData;

use ethers::providers::{Middleware};
use ethers::types::{Block, H256};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::storage::vars::{EthHeaderVariable, EthHeader};
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::get_provider;

#[derive(Debug, Clone)]
pub struct EthBlockGenerator<F: RichField + Extendable<D>, const D: usize> {
    block_hash: Bytes32Variable,
    pub value: EthHeaderVariable,
    chain_id: u64,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> EthBlockGenerator<F, D> {
    pub fn new(
        builder: &mut CircuitBuilder<F, D>,
        block_hash: Bytes32Variable,
    ) -> EthBlockGenerator<F, D> {
        let chain_id = builder.get_chain_id();
        let value = builder.init::<EthHeaderVariable>();
        EthBlockGenerator {
            block_hash,
            value,
            chain_id,
            _phantom: PhantomData::<F>,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for EthBlockGenerator<F, D>
{
    fn id(&self) -> String {
        "GetBlockByHashGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_hash.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, buffer: &mut GeneratedValues<F>) {
        let block_hash = self.block_hash.get(witness);
        let provider = get_provider(self.chain_id);
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result: Block<H256> = rt.block_on(async {
            provider
                .get_block(block_hash)
                .await
                .expect("Failed to get block from RPC")
        }).expect("No matching block found");
        
        let value = EthHeader {
            parent_hash: result.parent_hash,
            uncle_hash: result.uncles_hash,
            coinbase: result.author.expect("No coinbase"),
            root: result.state_root,
            tx_hash: result.transactions_root,
            receipt_hash: result.receipts_root,
            bloom: H256::from_slice(&result.logs_bloom.expect("No bloom").0),
            difficulty: result.difficulty,
            // TODO: Convert to U64Variable
            number: ethers::types::U256([0, 0, 0, result.number.expect("No block number").as_u64()]),
            // gas_limit: result.gas_limit,
            // gas_used: result.gas_used,
            // time: result.timestamp,
            extra: result.extra_data,
        };
        self.value.set(buffer, value);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        let chain_id_bytes = self.chain_id.to_be_bytes();
        dst.write_all(&chain_id_bytes)?;

        dst.write_target_vec(&self.block_hash.targets())?;
        dst.write_target_vec(&self.value.targets())
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let mut chain_id_bytes = [0u8; 8];
        src.read_exact(&mut chain_id_bytes)?;
        let chain_id = u64::from_be_bytes(chain_id_bytes);

        let block_hash_targets = src.read_target_vec()?;
        let block_hash = Bytes32Variable::from_targets(&block_hash_targets);

        let value_targets = src.read_target_vec()?;
        let value = EthHeaderVariable::from_targets(&value_targets);

        Ok(Self {
            block_hash,
            value,
            chain_id,
            _phantom: PhantomData::<F>,
        })
    }
}
