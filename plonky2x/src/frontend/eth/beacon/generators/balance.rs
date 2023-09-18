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
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::eth::BLSPubkey;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 39;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum BeaconBalanceInput {
    IndexConst(u64),
    IndexVariable(U64Variable),
    PubkeyConst(BLSPubkey),
    PubkeyVariable(BLSPubkeyVariable),
}

#[derive(Debug, Clone)]
pub struct BeaconBalanceGenerator<L: PlonkParameters<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    input: BeaconBalanceInput,
    pub balance: U64Variable,
    pub balance_leaf: Bytes32Variable,
    pub proof: [Bytes32Variable; DEPTH],
    pub gindex: U64Variable,
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> BeaconBalanceGenerator<L, D> {
    pub fn new_with_index_const(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        validator_idx: u64,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconBalanceInput::IndexConst(validator_idx),
            balance: builder.init_unsafe::<U64Variable>(),
            balance_leaf: builder.init_unsafe::<Bytes32Variable>(),
            proof: array![_ => builder.init_unsafe::<Bytes32Variable>(); DEPTH],
            gindex: builder.init::<U64Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn new_with_index_variable(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        validator_idx: U64Variable,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconBalanceInput::IndexVariable(validator_idx),
            balance: builder.init::<U64Variable>(),
            balance_leaf: builder.init::<Bytes32Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            gindex: builder.init::<U64Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn new_with_pubkey_const(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        pubkey: BLSPubkey,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconBalanceInput::PubkeyConst(pubkey),
            balance: builder.init::<U64Variable>(),
            balance_leaf: builder.init::<Bytes32Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            gindex: builder.init::<U64Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn new_with_pubkey_variable(
        builder: &mut CircuitBuilder<L, D>,
        block_root: Bytes32Variable,
        pubkey: BLSPubkeyVariable,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconBalanceInput::PubkeyVariable(pubkey),
            balance: builder.init::<U64Variable>(),
            balance_leaf: builder.init::<Bytes32Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            gindex: builder.init::<U64Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn id() -> String {
        "BeaconBalanceGenerator".to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for BeaconBalanceGenerator<L, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_root.targets());
        match self.input {
            BeaconBalanceInput::IndexConst(_) => {}
            BeaconBalanceInput::IndexVariable(ref idx) => {
                targets.extend(idx.targets());
            }
            BeaconBalanceInput::PubkeyConst(_) => {}
            BeaconBalanceInput::PubkeyVariable(ref pubkey) => {
                targets.extend(pubkey.targets());
            }
        }
        targets
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let block_root = self.block_root.get(witness);
        let result = match &self.input {
            BeaconBalanceInput::IndexConst(idx) => self
                .client
                .get_validator_balance_v2(hex!(block_root), *idx)
                .unwrap(),
            BeaconBalanceInput::IndexVariable(idx) => {
                let idx = idx.get(witness);
                self.client
                    .get_validator_balance_v2(hex!(block_root), idx.as_u64())
                    .unwrap()
            }
            BeaconBalanceInput::PubkeyConst(pubkey) => {
                let pubkey = hex!(pubkey.0);
                self.client
                    .get_validator_balance_by_pubkey_v2(hex!(block_root), pubkey)
                    .unwrap()
            }
            BeaconBalanceInput::PubkeyVariable(pubkey) => {
                let pubkey = hex!(pubkey.get(witness));
                self.client
                    .get_validator_balance_by_pubkey_v2(hex!(block_root), pubkey)
                    .unwrap()
            }
        };
        self.balance.set(out_buffer, result.balance.into());
        self.balance_leaf
            .set(out_buffer, bytes32!(result.balance_leaf));
        for i in 0..DEPTH {
            self.proof[i].set(out_buffer, bytes32!(result.proof[i]));
        }
        self.gindex.set(
            out_buffer,
            result.gindex.to_string().parse::<u64>().unwrap().into(),
        );
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        match &self.input {
            BeaconBalanceInput::IndexConst(idx) => {
                dst.write_usize(0)?;
                dst.write_usize(*idx as usize)?;
            }
            BeaconBalanceInput::IndexVariable(idx) => {
                dst.write_usize(1)?;
                dst.write_target_vec(&idx.targets())?;
            }
            BeaconBalanceInput::PubkeyConst(pubkey) => {
                dst.write_usize(2)?;
                dst.write_all(&pubkey.0)?;
            }
            BeaconBalanceInput::PubkeyVariable(ref pubkey) => {
                dst.write_usize(3)?;
                dst.write_target_vec(&pubkey.targets())?;
            }
        }
        dst.write_target_vec(&self.balance.targets())?;
        dst.write_target_vec(&self.balance_leaf.targets())?;
        for i in 0..DEPTH {
            dst.write_target_vec(&self.proof[i].targets())?;
        }
        dst.write_target_vec(&self.gindex.targets())?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let input_type = src.read_usize()?;
        let input = if input_type == 0 {
            let idx = src.read_usize()?;
            BeaconBalanceInput::IndexConst(idx as u64)
        } else if input_type == 1 {
            let idx = U64Variable::from_targets(&src.read_target_vec()?);
            BeaconBalanceInput::IndexVariable(idx)
        } else if input_type == 2 {
            let mut pubkey = [0u8; 48];
            src.read_exact(&mut pubkey)?;
            BeaconBalanceInput::PubkeyConst(BLSPubkey(pubkey))
        } else if input_type == 3 {
            let pubkey = BLSPubkeyVariable::from_targets(&src.read_target_vec()?);
            BeaconBalanceInput::PubkeyVariable(pubkey)
        } else {
            panic!("invalid input type")
        };
        let balance = U64Variable::from_targets(&src.read_target_vec()?);
        let balance_leaf = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let mut proof = Vec::new();
        for i in 0..DEPTH {
            proof.push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        let gindex = U64Variable::from_targets(&src.read_target_vec()?);
        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        Ok(Self {
            client,
            block_root,
            input,
            balance,
            balance_leaf,
            proof: proof.try_into().unwrap(),
            gindex,
            _phantom: PhantomData,
        })
    }
}
