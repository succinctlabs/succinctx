use std::fmt::Debug;

use ethers::types::H256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::prelude::Variable;

/// The value type for `BeaconValidatorsVariable`. Note that this struct does not have a natural
/// representation of the beacon validators. Instead it stores commitments to the underlying
/// data and some auxiliary information, such as the block root. This unnatural respresentation
/// is why we don't store the struct in the `ethutils` package.
#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorsValue {
    pub block_root: H256,
    pub validators_root: H256,
}

/// The container which holds all beacon validators at specific block root as variable in the
/// circuit. Note that under the hood, we only store the commitment to the validators. To access
/// the underlying data, we witness merkle proofs.
#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorsVariable {
    pub block_root: Bytes32Variable,
    pub validators_root: Bytes32Variable,
}

impl CircuitVariable for BeaconValidatorsVariable {
    type ValueType<F: RichField> = BeaconValidatorsValue;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            validators_root: Bytes32Variable::init(builder),
            block_root: Bytes32Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self {
            block_root: Bytes32Variable::constant(builder, value.block_root),
            validators_root: Bytes32Variable::constant(builder, value.validators_root),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.block_root
            .variables()
            .into_iter()
            .chain(self.validators_root.variables())
            .collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        let block_root = Bytes32Variable::from_variables(&variables[0..32]);
        let validators_root = Bytes32Variable::from_variables(&variables[32..64]);
        Self {
            block_root,
            validators_root,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        BeaconValidatorsValue {
            block_root: self.block_root.get(witness),
            validators_root: self.validators_root.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.validators_root.set(witness, value.validators_root);
        self.block_root.set(witness, value.block_root);
    }
}
