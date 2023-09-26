use core::marker::PhantomData;
use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const CLOSE_SLOT_BLOCK_ROOT_DEPTH: usize = 18;
const FAR_SLOT_HISTORICAL_SUMMARY_DEPTH: usize = 33;
const FAR_SLOT_BLOCK_ROOT_DEPTH: usize = 14;

#[derive(Debug, Clone)]
pub struct BeaconHistoricalBlockGenerator<F: RichField + Extendable<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    target_slot: U64Variable,
    pub target_block_root: Bytes32Variable,
    pub close_slot_block_root_proof: Vec<Bytes32Variable>,
    pub far_slot_block_root_proof: Vec<Bytes32Variable>,
    pub far_slot_historical_summary_root: Bytes32Variable,
    pub far_slot_historical_summary_proof: Vec<Bytes32Variable>,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BeaconHistoricalBlockGenerator<F, D> {
    pub fn new<L: PlonkParameters<D>>(
        builder: &mut CircuitBuilder<L, D>,
        client: BeaconClient,
        block_root: Bytes32Variable,
        target_slot: U64Variable,
    ) -> Self {
        Self {
            client,
            block_root,
            target_slot,
            target_block_root: builder.init::<Bytes32Variable>(),
            far_slot_historical_summary_root: builder.init::<Bytes32Variable>(),
            close_slot_block_root_proof: (0..CLOSE_SLOT_BLOCK_ROOT_DEPTH)
                .map(|_| builder.init::<Bytes32Variable>())
                .collect::<Vec<_>>(),
            far_slot_block_root_proof: (0..FAR_SLOT_BLOCK_ROOT_DEPTH)
                .map(|_| builder.init::<Bytes32Variable>())
                .collect::<Vec<_>>(),
            far_slot_historical_summary_proof: (0..FAR_SLOT_HISTORICAL_SUMMARY_DEPTH)
                .map(|_| builder.init::<Bytes32Variable>())
                .collect::<Vec<_>>(),
            _phantom: Default::default(),
        }
    }

    pub fn id() -> String {
        "BeaconHistoricalBlockGenerator".to_string()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BeaconHistoricalBlockGenerator<F, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_root.targets());
        targets.extend(self.target_slot.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = self.block_root.get(witness);
        let target_slot = self.target_slot.get(witness);

        let result = self
            .client
            .get_historical_block(
                hex!(block_root.as_bytes()).to_string(),
                target_slot.as_u64(),
            )
            .expect("failed to get validators root");

        self.target_block_root
            .set(out_buffer, bytes32!(result.target_block_root));
        for i in 0..CLOSE_SLOT_BLOCK_ROOT_DEPTH {
            self.close_slot_block_root_proof[i]
                .set(out_buffer, bytes32!(result.close_slot_block_root_proof[i]));
        }
        for i in 0..FAR_SLOT_BLOCK_ROOT_DEPTH {
            self.far_slot_block_root_proof[i]
                .set(out_buffer, bytes32!(result.far_slot_block_root_proof[i]));
        }
        self.far_slot_historical_summary_root.set(
            out_buffer,
            bytes32!(result.far_slot_historical_summary_root),
        );
        for i in 0..FAR_SLOT_HISTORICAL_SUMMARY_DEPTH {
            self.far_slot_historical_summary_proof[i].set(
                out_buffer,
                bytes32!(result.far_slot_historical_summary_proof[i]),
            );
        }
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        dst.write_target_vec(&self.target_slot.targets())?;
        dst.write_target_vec(&self.target_block_root.targets())?;
        for i in 0..CLOSE_SLOT_BLOCK_ROOT_DEPTH {
            dst.write_target_vec(&self.close_slot_block_root_proof[i].targets())?;
        }
        for i in 0..FAR_SLOT_BLOCK_ROOT_DEPTH {
            dst.write_target_vec(&self.far_slot_block_root_proof[i].targets())?;
        }
        dst.write_target_vec(&self.far_slot_historical_summary_root.targets())?;
        for i in 0..FAR_SLOT_HISTORICAL_SUMMARY_DEPTH {
            dst.write_target_vec(&self.far_slot_historical_summary_proof[i].targets())?;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let target_slot = U64Variable::from_targets(&src.read_target_vec()?);
        let target_block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let mut close_slot_block_root_proof = Vec::new();
        for i in 0..CLOSE_SLOT_BLOCK_ROOT_DEPTH {
            close_slot_block_root_proof
                .push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        let mut far_slot_block_root_proof = Vec::new();
        for i in 0..FAR_SLOT_BLOCK_ROOT_DEPTH {
            far_slot_block_root_proof.push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        let far_slot_historical_summary_root =
            Bytes32Variable::from_targets(&src.read_target_vec()?);
        let mut far_slot_historical_summary_proof = Vec::new();
        for i in 0..FAR_SLOT_HISTORICAL_SUMMARY_DEPTH {
            far_slot_historical_summary_proof
                .push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        Ok(Self {
            client,
            block_root,
            target_slot,
            target_block_root,
            close_slot_block_root_proof,
            far_slot_block_root_proof,
            far_slot_historical_summary_root,
            far_slot_historical_summary_proof,
            _phantom: Default::default(),
        })
    }
}
