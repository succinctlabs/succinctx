use std::fmt::Debug;

use array_macro::array;
use ethers::types::{Address, H256, U256, U64};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, U256Variable};
use crate::prelude::Variable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EthAccount {
    pub balance: U256,
    pub code_hash: H256,
    pub nonce: U64,
    pub storage_hash: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct EthAccountVariable {
    pub balance: U256Variable,
    pub code_hash: Bytes32Variable,
    pub nonce: U64Variable,
    pub storage_hash: Bytes32Variable,
}

impl CircuitVariable for EthAccountVariable {
    type ValueType<F: RichField> = EthAccount;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            balance: U256Variable::init(builder),
            code_hash: Bytes32Variable::init(builder),
            nonce: U64Variable::init(builder),
            storage_hash: Bytes32Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self {
            balance: U256Variable::constant(builder, value.balance),
            code_hash: Bytes32Variable::constant(builder, value.code_hash),
            nonce: U64Variable::constant(builder, value.nonce),
            storage_hash: Bytes32Variable::constant(builder, value.storage_hash),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.balance.variables());
        vars.extend(self.code_hash.variables());
        vars.extend(self.nonce.variables());
        vars.extend(self.storage_hash.variables());
        vars
    }

    fn from_variables(variables: &[Variable]) -> Self {
        let balance = U256Variable::from_variables(&variables[0..4]);
        let mut offset = 4;
        let code_hash = Bytes32Variable::from_variables(&variables[offset..offset + 32 * 8]);
        offset += 32 * 8;
        let nonce = U64Variable::from_variables(&variables[offset..offset + 1]);
        offset += 1;
        let storage_hash = Bytes32Variable::from_variables(&variables[offset..offset + 32 * 8]);
        Self {
            balance,
            code_hash,
            nonce,
            storage_hash,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        EthAccount {
            balance: self.balance.get(witness),
            code_hash: self.code_hash.get(witness),
            nonce: self.nonce.get(witness),
            storage_hash: self.storage_hash.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.balance.set(witness, value.balance);
        self.code_hash.set(witness, value.code_hash);
        self.nonce.set(witness, value.nonce);
        self.storage_hash.set(witness, value.storage_hash);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EthLog {
    pub address: Address,
    pub topics: [H256; 3],
    pub data_hash: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct EthLogVariable {
    pub address: AddressVariable,
    pub topics: [Bytes32Variable; 3],
    pub data_hash: Bytes32Variable,
}

impl CircuitVariable for EthLogVariable {
    type ValueType<F: RichField> = EthLog;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            address: AddressVariable::init(builder),
            topics: array![_ => Bytes32Variable::init(builder); 3],
            data_hash: Bytes32Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self {
            address: AddressVariable::constant(builder, value.address),
            topics: array![i => Bytes32Variable::constant(builder, value.topics[i]); 3],
            data_hash: Bytes32Variable::constant(builder, value.data_hash),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.address.variables());
        vars.extend(
            self.topics
                .iter()
                .flat_map(|t| t.variables())
                .collect::<Vec<Variable>>(),
        );
        vars.extend(self.data_hash.variables());
        vars
    }

    fn from_variables(variables: &[Variable]) -> Self {
        // TODO: include assertion about how long variables are
        let address = AddressVariable::from_variables(&variables[0..8 * 20]);
        let mut offset = 8 * 20;
        let topics = [
            Bytes32Variable::from_variables(&variables[offset..offset + 32 * 8]),
            Bytes32Variable::from_variables(&variables[offset + 32 * 8..offset + 32 * 8 * 2]),
            Bytes32Variable::from_variables(&variables[offset + 32 * 8 * 2..offset + 32 * 8 * 3]),
        ];
        offset += 32 * 8 * 3;
        let data_hash = Bytes32Variable::from_variables(&variables[offset..offset + 32 * 8]);
        Self {
            address,
            topics,
            data_hash,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        EthLog {
            address: self.address.get(witness),
            topics: [
                self.topics[0].get(witness),
                self.topics[1].get(witness),
                self.topics[2].get(witness),
            ],
            data_hash: self.data_hash.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.address.set(witness, value.address);
        self.topics[0].set(witness, value.topics[0]);
        self.topics[1].set(witness, value.topics[1]);
        self.topics[2].set(witness, value.topics[2]);
        self.data_hash.set(witness, value.data_hash);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EthProof {
    pub proof: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct EthProofVariable {
    pub proof: Bytes32Variable,
}

impl CircuitVariable for EthProofVariable {
    type ValueType<F: RichField> = EthProof;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            proof: Bytes32Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self {
            proof: Bytes32Variable::constant(builder, value.proof),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.proof.variables()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self {
            proof: Bytes32Variable::from_variables(variables),
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        EthProof {
            proof: self.proof.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.proof.set(witness, value.proof);
    }
}
