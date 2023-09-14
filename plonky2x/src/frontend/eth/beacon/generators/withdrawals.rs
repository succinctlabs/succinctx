use core::marker::PhantomData;
use std::env;

use array_macro::array;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 11;

#[derive(Debug, Clone)]
pub struct BeaconWithdrawalsGenerator<L: PlonkParameters<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    pub withdrawals_root: Bytes32Variable,
    pub proof: [Bytes32Variable; DEPTH],
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> BeaconWithdrawalsGenerator<L, D> {
    pub fn new(
        builder: &mut CircuitBuilder<L, D>,
        client: BeaconClient,
        block_root: Bytes32Variable,
    ) -> Self {
        Self {
            client,
            block_root,
            withdrawals_root: builder.init::<Bytes32Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            _phantom: Default::default(),
        }
    }

    pub fn id() -> String {
        "BeaconWithdrawalsGenerator".to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for BeaconWithdrawalsGenerator<L, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.block_root.targets()
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let block_root = self.block_root.get(witness);

        let result = self
            .client
            .get_withdrawals_root(hex!(block_root.as_bytes()).to_string())
            .expect("failed to get validators root");

        self.withdrawals_root
            .set(out_buffer, bytes32!(result.withdrawals_root));
        for i in 0..DEPTH {
            self.proof[i].set(out_buffer, bytes32!(result.proof[i]));
        }
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        dst.write_target_vec(&self.withdrawals_root.targets())?;
        for i in 0..DEPTH {
            dst.write_target_vec(&self.proof[i].targets())?;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let withdrawals_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let mut proof = Vec::new();
        for i in 0..DEPTH {
            proof.push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        Ok(Self {
            client,
            block_root,
            withdrawals_root,
            proof: proof.try_into().unwrap(),
            _phantom: Default::default(),
        })
    }
}
