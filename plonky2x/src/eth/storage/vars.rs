use std::fmt::Debug;

use ethers::types::{H256, U256};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{Bytes32Variable, CircuitVariable, FieldSerializable, U256Variable};

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

    fn targets(&self) -> Vec<Target> {
        self.proof.targets()
    }

    fn from_targets(targets: &[Target]) -> Self {
        Self {
            proof: Bytes32Variable::from_targets(targets),
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
    type ValueType<F: RichField> = EthAccount;

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

impl<F: RichField> FieldSerializable<F> for EthProof {
    fn nb_elements() -> usize {
        256
    }

    fn elements(&self) -> Vec<F> {
        self.proof.elements()
    }

    fn from_elements(elements: &[F]) -> Self {
        assert_eq!(
            elements.len(),
            <Self as FieldSerializable<F>>::nb_elements()
        );
        let proof = H256::from_elements(elements);
        EthProof { proof: proof }
    }
}

impl<F: RichField> FieldSerializable<F> for EthAccount {
    fn nb_elements() -> usize {
        256 * 4
    }

    fn elements(&self) -> Vec<F> {
        let mut elements: Vec<F> = Vec::new();
        elements.extend::<Vec<F>>(self.balance.elements());
        elements.extend::<Vec<F>>(self.code_hash.elements());
        elements.extend::<Vec<F>>(self.nonce.elements());
        elements.extend::<Vec<F>>(self.storage_hash.elements());
        elements
    }

    fn from_elements(elements: &[F]) -> Self {
        assert_eq!(
            elements.len(),
            <Self as FieldSerializable<F>>::nb_elements()
        );
        let balance = U256::from_elements(&elements[0..256]);
        let code_hash = H256::from_elements(&elements[256..512]);
        let nonce = U256::from_elements(&elements[512..768]);
        let storage_hash = H256::from_elements(&elements[768..1024]);
        EthAccount {
            balance: balance,
            code_hash: code_hash,
            nonce: nonce,
            storage_hash: storage_hash,
        }
    }
}
