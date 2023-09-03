use core::marker::PhantomData;

use curta::math::prelude::PrimeField64;
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
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::prelude::Variable;
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::eth::BLSPubkey;
use crate::utils::hex;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum BeaconValidatorBalanceInput {
    IndexConst(u64),
    IndexVariable(Variable),
    PubkeyConst(BLSPubkey),
    PubkeyVariable(BLSPubkeyVariable),
}

#[derive(Debug, Clone)]
pub struct BeaconValidatorBalanceGenerator<F: RichField + Extendable<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    input: BeaconValidatorBalanceInput,
    balance: U256Variable,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BeaconValidatorBalanceGenerator<F, D> {
    pub fn new_with_index_const(
        builder: &mut CircuitBuilder<F, D>,
        block_root: Bytes32Variable,
        validator_idx: u64,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconValidatorBalanceInput::IndexConst(validator_idx),
            balance: builder.init::<U256Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn new_with_index_variable(
        builder: &mut CircuitBuilder<F, D>,
        block_root: Bytes32Variable,
        validator_idx: Variable,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            input: BeaconValidatorBalanceInput::IndexVariable(validator_idx),
            balance: builder.init::<U256Variable>(),
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
            input: BeaconValidatorBalanceInput::PubkeyConst(pubkey),
            balance: builder.init::<U256Variable>(),
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
            input: BeaconValidatorBalanceInput::PubkeyVariable(pubkey),
            balance: builder.init::<U256Variable>(),
            _phantom: PhantomData,
        }
    }

    pub fn out(&self) -> U256Variable {
        self.balance
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BeaconValidatorBalanceGenerator<F, D>
{
    fn id(&self) -> String {
        "BeaconValidatorBalanceGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_root.targets());
        match self.input {
            BeaconValidatorBalanceInput::IndexConst(_) => {}
            BeaconValidatorBalanceInput::IndexVariable(ref idx) => {
                targets.extend(idx.targets());
            }
            BeaconValidatorBalanceInput::PubkeyConst(_) => {}
            BeaconValidatorBalanceInput::PubkeyVariable(ref pubkey) => {
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
                    BeaconValidatorBalanceInput::IndexConst(idx) => {
                        self.client
                            .get_validator_balance(hex!(block_root), *idx)
                            .await
                    }
                    BeaconValidatorBalanceInput::IndexVariable(idx) => {
                        let idx = idx.get(witness).as_canonical_u64();
                        self.client
                            .get_validator_balance(hex!(block_root), idx)
                            .await
                    }
                    BeaconValidatorBalanceInput::PubkeyConst(pubkey) => {
                        let pubkey = hex!(pubkey.0);
                        self.client
                            .get_validator_balance_by_pubkey(hex!(block_root), pubkey)
                            .await
                    }
                    BeaconValidatorBalanceInput::PubkeyVariable(pubkey) => {
                        let pubkey = hex!(pubkey.get(witness));
                        self.client
                            .get_validator_balance_by_pubkey(hex!(block_root), pubkey)
                            .await
                    }
                }
            })
            .unwrap();
        self.balance.set(out_buffer, result);
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
