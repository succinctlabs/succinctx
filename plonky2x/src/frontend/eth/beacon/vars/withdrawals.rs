use std::fmt::Debug;

use ethers::types::H256;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::prelude::Variable;

#[derive(Debug, Clone, Copy)]
pub struct BeaconWithdrawalsValue {
    pub block_root: H256,
    pub withdrawals_root: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct BeaconWithdrawalsVariable {
    pub block_root: Bytes32Variable,
    pub withdrawals_root: Bytes32Variable,
}

impl CircuitVariable for BeaconWithdrawalsVariable {
    type ValueType<F: RichField> = BeaconWithdrawalsValue;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            withdrawals_root: Bytes32Variable::init(builder),
            block_root: Bytes32Variable::init(builder),
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self {
            block_root: Bytes32Variable::constant(builder, value.block_root),
            withdrawals_root: Bytes32Variable::constant(builder, value.withdrawals_root),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.block_root
            .variables()
            .into_iter()
            .chain(self.withdrawals_root.variables())
            .collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        let block_root = Bytes32Variable::from_variables(&variables[0..256]);
        let validators_root = Bytes32Variable::from_variables(&variables[256..512]);
        Self {
            block_root,
            withdrawals_root: validators_root,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        BeaconWithdrawalsValue {
            block_root: self.block_root.get(witness),
            withdrawals_root: self.withdrawals_root.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.withdrawals_root.set(witness, value.withdrawals_root);
        self.block_root.set(witness, value.block_root);
    }
}
