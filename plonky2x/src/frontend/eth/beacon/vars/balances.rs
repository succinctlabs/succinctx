use std::fmt::Debug;

use ethers::types::H256;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, Variable};
use crate::prelude::FieldVariable;

#[derive(Debug, Clone, Copy)]
pub struct BeaconBalancesValue {
    pub block_root: H256,
    pub balances_root: H256,
}

#[derive(Debug, Clone, Copy)]
pub struct BeaconBalancesVariable {
    pub block_root: Bytes32Variable,
    pub balances_root: Bytes32Variable,
}

impl Variable for BeaconBalancesVariable {
    type ValueType<F: RichField> = BeaconBalancesValue;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            balances_root: Bytes32Variable::init(builder),
            block_root: Bytes32Variable::init(builder),
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self {
            block_root: Bytes32Variable::constant(builder, value.block_root),
            balances_root: Bytes32Variable::constant(builder, value.balances_root),
        }
    }

    fn variables(&self) -> Vec<FieldVariable> {
        self.block_root
            .variables()
            .into_iter()
            .chain(self.balances_root.variables())
            .collect()
    }

    fn from_variables(variables: &[FieldVariable]) -> Self {
        let block_root = Bytes32Variable::from_variables(&variables[0..256]);
        let validators_root = Bytes32Variable::from_variables(&variables[256..512]);
        Self {
            block_root,
            balances_root: validators_root,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        BeaconBalancesValue {
            block_root: self.block_root.get(witness),
            balances_root: self.balances_root.get(witness),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.balances_root.set(witness, value.balances_root);
        self.block_root.set(witness, value.block_root);
    }
}
