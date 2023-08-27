use std::fmt::Debug;

use ethers::types::{Address, Bytes, H256, U256};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, U256Variable, U64Variable};
use crate::prelude::Variable;

/// A variable representing the Ethereum Block Header
/// Follow the following struct in go-ethereum
/// https://github.com/ethereum/go-ethereum/blob/b6d4f6b66e99c08f419e6a469259cbde1c8b0582/core/types/block.go#L70
/// https://github.com/gnosis/hashi/blob/main/packages/evm/contracts/adapters/BlockHashOracleAdapter.sol#L24
/// Note that this only includes certain fields in the certain block header
#[derive(Debug, Clone)]
pub struct EthHeader {
    pub parent_hash: H256,
    pub uncle_hash: H256,
    pub coinbase: Address,
    pub root: H256,
    pub tx_hash: H256,
    pub receipt_hash: H256,
    pub bloom: H256,
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub time: u64,
    pub extra: Bytes,
}

#[derive(Debug, Clone, Copy)]
pub struct EthHeaderVariable {
    pub parent_hash: Bytes32Variable,
    pub uncle_hash: Bytes32Variable,
    pub coinbase: AddressVariable,
    pub root: Bytes32Variable,
    pub tx_hash: Bytes32Variable,
    pub receipt_hash: Bytes32Variable,
    pub bloom: Bytes32Variable,
    pub difficulty: U256Variable,
    pub number: U256Variable,
    pub gas_limit: U64Variable,
    pub gas_used: U64Variable,
    pub time: U64Variable,
    pub extra: Bytes32Variable,
}

impl CircuitVariable for EthHeaderVariable {
    type ValueType<F: RichField> = EthHeader;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            parent_hash: Bytes32Variable::init(builder),
            uncle_hash: Bytes32Variable::init(builder),
            coinbase: AddressVariable::init(builder),
            root: Bytes32Variable::init(builder),
            tx_hash: Bytes32Variable::init(builder),
            receipt_hash: Bytes32Variable::init(builder),
            bloom: Bytes32Variable::init(builder),
            difficulty: U256Variable::init(builder),
            number: U256Variable::init(builder),
            gas_limit: U64Variable::init(builder),
            gas_used: U64Variable::init(builder),
            time: U64Variable::init(builder),
            extra: Bytes32Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        todo!()
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.parent_hash.variables());
        vars.extend(self.uncle_hash.variables());
        vars.extend(self.coinbase.variables());
        vars.extend(self.root.variables());
        vars.extend(self.tx_hash.variables());
        vars.extend(self.receipt_hash.variables());
        vars.extend(self.bloom.variables());
        vars.extend(self.difficulty.variables());
        vars.extend(self.number.variables());
        vars.extend(self.gas_limit.variables());
        vars.extend(self.gas_used.variables());
        vars.extend(self.time.variables());
        vars.extend(self.extra.variables());
        vars
    }

    fn from_variables(variables: &[Variable]) -> Self {
        todo!()
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        todo!()
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        todo!()
    }
}
