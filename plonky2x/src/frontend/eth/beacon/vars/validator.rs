use std::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::vars::{BoolVariable, Bytes32Variable, CircuitVariable, U256Variable};
use crate::prelude::Variable;
use crate::utils::eth::beacon::BeaconValidator;
use crate::utils::{bytes, bytes32, hex};

#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorVariable {
    pub pubkey: BLSPubkeyVariable,
    pub withdrawal_credentials: Bytes32Variable,
    pub effective_balance: U256Variable,
    pub slashed: BoolVariable,
    pub activation_eligibility_epoch: U256Variable,
    pub activation_epoch: U256Variable,
    pub exit_epoch: U256Variable,
    pub withdrawable_epoch: U256Variable,
}

impl CircuitVariable for BeaconValidatorVariable {
    type ValueType<F: RichField> = BeaconValidator;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            pubkey: BLSPubkeyVariable::init(builder),
            withdrawal_credentials: Bytes32Variable::init(builder),
            effective_balance: U256Variable::init(builder),
            slashed: BoolVariable::init(builder),
            activation_eligibility_epoch: U256Variable::init(builder),
            activation_epoch: U256Variable::init(builder),
            exit_epoch: U256Variable::init(builder),
            withdrawable_epoch: U256Variable::init(builder),
        }
    }

    #[allow(unused_variables)]
    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        todo!()
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.pubkey.variables());
        vars.extend(self.withdrawal_credentials.variables());
        vars.extend(self.effective_balance.variables());
        vars.extend(self.slashed.variables());
        vars.extend(self.activation_eligibility_epoch.variables());
        vars.extend(self.activation_epoch.variables());
        vars.extend(self.exit_epoch.variables());
        vars.extend(self.withdrawable_epoch.variables());
        vars
    }

    #[allow(unused_variables)]
    fn from_variables(variables: &[Variable]) -> Self {
        todo!()
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        BeaconValidator {
            pubkey: hex!(self.pubkey.get(witness)),
            withdrawal_credentials: hex!(self.withdrawal_credentials.get(witness)),
            effective_balance: self.effective_balance.get(witness).as_u64(),
            slashed: self.slashed.get(witness),
            activation_eligibility_epoch: self.activation_eligibility_epoch.get(witness).as_u64(),
            activation_epoch: self.activation_epoch.get(witness).as_u64(),
            exit_epoch: Some(self.exit_epoch.get(witness).as_u64()),
            withdrawable_epoch: Some(self.withdrawable_epoch.get(witness).as_u64()),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.pubkey.set(witness, bytes!(value.pubkey));
        self.withdrawal_credentials
            .set(witness, bytes32!(value.withdrawal_credentials));
        self.effective_balance
            .set(witness, value.effective_balance.into());
        self.slashed.set(witness, value.slashed);
        self.activation_eligibility_epoch
            .set(witness, value.activation_eligibility_epoch.into());
        self.activation_epoch
            .set(witness, value.activation_epoch.into());
        self.exit_epoch
            .set(witness, value.exit_epoch.unwrap_or(0).into());
        self.withdrawable_epoch
            .set(witness, value.withdrawable_epoch.unwrap_or(0).into());
    }
}
