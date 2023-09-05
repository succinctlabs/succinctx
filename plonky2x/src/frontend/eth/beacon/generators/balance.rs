use core::marker::PhantomData;

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
use tokio::runtime::Runtime;

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
pub struct BeaconBalanceGenerator<F: RichField + Extendable<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    input: BeaconBalanceInput,
    pub balance: U64Variable,
    pub balance_leaf: Bytes32Variable,
    pub proof: [Bytes32Variable; DEPTH],
    pub gindex: U64Variable,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BeaconBalanceGenerator<F, D> {
    pub fn new_with_index_const(
        builder: &mut CircuitBuilder<F, D>,
        block_root: Bytes32Variable,
        validator_idx: u64,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconBalanceInput::IndexConst(validator_idx),
            balance: builder.init::<U64Variable>(),
            balance_leaf: builder.init::<Bytes32Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            gindex: builder.init::<U64Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn new_with_index_variable(
        builder: &mut CircuitBuilder<F, D>,
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
        builder: &mut CircuitBuilder<F, D>,
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
        builder: &mut CircuitBuilder<F, D>,
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
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BeaconBalanceGenerator<F, D>
{
    fn id(&self) -> String {
        "BeaconBalanceGenerator".to_string()
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

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = self.block_root.get(witness);
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result = rt
            .block_on(async {
                match &self.input {
                    BeaconBalanceInput::IndexConst(idx) => {
                        self.client
                            .get_validator_balance_v2(hex!(block_root), *idx)
                            .await
                    }
                    BeaconBalanceInput::IndexVariable(idx) => {
                        let idx = idx.get(witness);
                        self.client
                            .get_validator_balance_v2(hex!(block_root), idx.as_u64())
                            .await
                    }
                    BeaconBalanceInput::PubkeyConst(pubkey) => {
                        let pubkey = hex!(pubkey.0);
                        self.client
                            .get_validator_balance_by_pubkey_v2(hex!(block_root), pubkey)
                            .await
                    }
                    BeaconBalanceInput::PubkeyVariable(pubkey) => {
                        let pubkey = hex!(pubkey.get(witness));
                        self.client
                            .get_validator_balance_by_pubkey_v2(hex!(block_root), pubkey)
                            .await
                    }
                }
            })
            .unwrap();
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
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}
