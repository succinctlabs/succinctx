use ethers::types::{H256, U256};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{BoolVariable, Bytes32Variable, CircuitVariable, U256Variable};

#[derive(Debug, Clone, Copy)]
pub struct EthProof {
    pub proof: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct EthProofVariable {
    pub proof: Bytes32Variable,
}

impl CircuitVariable for EthProofVariable {
    type ValueType<F> = EthProof;

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

    fn targets(&self) -> Vec<Target> {
        self.proof.targets()
    }

    fn from_targets(targets: &[Target]) -> Self {
        Self {
            proof: Bytes32Variable::from_targets(targets),
        }
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        EthProof {
            proof: self.proof.value(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.proof.set(witness, value.proof);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EthAccount {
    pub balance: U256,
    pub code_hash: H256,
    pub nonce: U256,
    pub storage_hash: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct EthAccountVariable {
    pub balance: U256Variable,
    pub code_hash: Bytes32Variable,
    pub nonce: U256Variable,
    pub storage_hash: Bytes32Variable,
}

impl CircuitVariable for EthAccountVariable {
    type ValueType<F> = EthAccount;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            balance: U256Variable::init(builder),
            code_hash: Bytes32Variable::init(builder),
            nonce: U256Variable::init(builder),
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
            nonce: U256Variable::constant(builder, value.nonce),
            storage_hash: Bytes32Variable::constant(builder, value.storage_hash),
        }
    }

    fn targets(&self) -> Vec<Target> {
        vec![
            self.balance.targets(),
            self.code_hash.targets(),
            self.nonce.targets(),
            self.storage_hash.targets(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[allow(unused_variables)]
    fn from_targets(targets: &[Target]) -> Self {
        todo!()
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        EthAccount {
            balance: self.balance.value(witness),
            code_hash: self.code_hash.value(witness),
            nonce: self.nonce.value(witness),
            storage_hash: self.storage_hash.value(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.balance.set(witness, value.balance);
        self.code_hash.set(witness, value.code_hash);
        self.nonce.set(witness, value.nonce);
        self.storage_hash.set(witness, value.storage_hash);
    }
}

impl EthAccountVariable {
    pub fn serialize(&self) -> Vec<BoolVariable> {
        vec![]
    }
}
