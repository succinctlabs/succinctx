use std::fmt::Debug;

use ethers::types::{Address, H256, U256, U64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, U256Variable};
use crate::prelude::Variable;

/// A variable representing the Ethereum Block Header
/// Follow the following struct in go-ethereum
/// https://github.com/ethereum/go-ethereum/blob/b6d4f6b66e99c08f419e6a469259cbde1c8b0582/core/types/block.go#L70
/// https://github.com/gnosis/hashi/blob/main/packages/evm/contracts/adapters/BlockHashOracleAdapter.sol#L24
/// Note that this only includes certain fields in the certain block header
#[derive(Debug, Clone, PartialEq)]
pub struct EthHeader {
    pub parent_hash: H256,
    pub uncle_hash: H256,
    pub coinbase: Address,
    pub root: H256,
    pub tx_hash: H256,
    pub receipt_hash: H256,
    // pub bloom: Bytes,
    pub difficulty: U256,
    pub number: U64,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub time: U256,
    // pub extra: Bytes,
}

#[derive(Debug, Clone, Copy)]
pub struct EthHeaderVariable {
    pub parent_hash: Bytes32Variable,
    pub uncle_hash: Bytes32Variable,
    pub coinbase: AddressVariable,
    pub root: Bytes32Variable,
    pub tx_hash: Bytes32Variable,
    pub receipt_hash: Bytes32Variable,
    // pub bloom: BytesVariable, // TODO: add back once we have arbitrary bytes variables
    pub difficulty: U256Variable,
    pub number: U64Variable,
    pub gas_limit: U256Variable,
    pub gas_used: U256Variable,
    pub time: U256Variable,
    // pub extra: Bytes32Variable, // TODO: add back once we have arbitrary bytes variables
}

impl CircuitVariable for EthHeaderVariable {
    type ValueType<F: RichField> = EthHeader;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            parent_hash: Bytes32Variable::init(builder),
            uncle_hash: Bytes32Variable::init(builder),
            coinbase: AddressVariable::init(builder),
            root: Bytes32Variable::init(builder),
            tx_hash: Bytes32Variable::init(builder),
            receipt_hash: Bytes32Variable::init(builder),
            // bloom: Bytes32Variable::init(builder),
            difficulty: U256Variable::init(builder),
            number: U64Variable::init(builder),
            gas_limit: U256Variable::init(builder),
            gas_used: U256Variable::init(builder),
            time: U256Variable::init(builder),
            // extra: Bytes32Variable::init(builder),
        }
    }

    #[allow(unused_variables)]
    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self {
            parent_hash: Bytes32Variable::constant(builder, value.parent_hash),
            uncle_hash: Bytes32Variable::constant(builder, value.uncle_hash),
            coinbase: AddressVariable::constant(builder, value.coinbase),
            root: Bytes32Variable::constant(builder, value.root),
            tx_hash: Bytes32Variable::constant(builder, value.tx_hash),
            receipt_hash: Bytes32Variable::constant(builder, value.receipt_hash),
            // bloom,
            difficulty: U256Variable::constant(builder, value.difficulty),
            number: U64Variable::constant(builder, value.number),
            gas_limit: U256Variable::constant(builder, value.gas_limit),
            gas_used: U256Variable::constant(builder, value.gas_used),
            time: U256Variable::constant(builder, value.time),
            // extra
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.parent_hash.variables());
        vars.extend(self.uncle_hash.variables());
        vars.extend(self.coinbase.variables());
        vars.extend(self.root.variables());
        vars.extend(self.tx_hash.variables());
        vars.extend(self.receipt_hash.variables());
        // vars.extend(self.bloom.variables());
        vars.extend(self.difficulty.variables());
        vars.extend(self.number.variables());
        vars.extend(self.gas_limit.variables());
        vars.extend(self.gas_used.variables());
        vars.extend(self.time.variables());
        // vars.extend(self.extra.variables());
        vars
    }

    #[allow(unused_variables)]
    fn from_variables(variables: &[Variable]) -> Self {
        // let mut var_buffer = VariableBuffer::new(variables);
        let parent_hash = Bytes32Variable::from_variables(&variables[0..256]);
        let uncle_hash = Bytes32Variable::from_variables(&variables[256..512]);
        let coinbase = AddressVariable::from_variables(&variables[512..672]);
        let root = Bytes32Variable::from_variables(&variables[672..928]);
        let tx_hash = Bytes32Variable::from_variables(&variables[928..1184]);
        let receipt_hash = Bytes32Variable::from_variables(&variables[1184..1440]);
        // let root = var_buffer.read::<Bytes32Variable>();
        let difficulty = U256Variable::from_variables(&variables[1440..1448]);
        let number = U64Variable::from_variables(&variables[1448..1450]);
        let gas_limit = U256Variable::from_variables(&variables[1450..1458]);
        let gas_used = U256Variable::from_variables(&variables[1458..1466]);
        let time = U256Variable::from_variables(&variables[1466..1474]);
        // let extra = var_buffer.read::<Bytes32Variable>();

        Self {
            parent_hash,
            uncle_hash,
            coinbase,
            root,
            tx_hash,
            receipt_hash,
            // bloom,
            difficulty,
            number,
            gas_limit,
            gas_used,
            time,
            // extra
        }
    }

    #[allow(unused_variables)]
    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        EthHeader {
            parent_hash: self.parent_hash.get(witness),
            uncle_hash: self.uncle_hash.get(witness),
            coinbase: self.coinbase.get(witness),
            root: self.root.get(witness),
            tx_hash: self.tx_hash.get(witness),
            receipt_hash: self.receipt_hash.get(witness),
            // bloom: self.bloom.get(witness),
            difficulty: self.difficulty.get(witness),
            number: self.number.get(witness),
            gas_limit: self.gas_limit.get(witness),
            gas_used: self.gas_used.get(witness),
            time: self.time.get(witness),
            // extra: self.extra.get(witness).as_bytes().to_vec().into(),
        }
    }

    #[allow(unused_variables)]
    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.parent_hash.set(witness, value.parent_hash);
        self.uncle_hash.set(witness, value.uncle_hash);
        self.coinbase.set(witness, value.coinbase);
        self.root.set(witness, value.root);
        self.tx_hash.set(witness, value.tx_hash);
        self.receipt_hash.set(witness, value.receipt_hash);
        // self.bloom.set(witness, value.bloom);
        self.difficulty.set(witness, value.difficulty);
        self.number.set(witness, value.number);
        self.gas_limit.set(witness, value.gas_limit);
        self.gas_used.set(witness, value.gas_used);
        self.time.set(witness, value.time);
        // self.extra.set(witness, H256::from_slice(value.extra.0.as_ref()));
    }
}
