use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;

use crate::eth::types::BLSPubkeyVariable;
use crate::eth::witness::EthWriteableWitness;
use crate::ethutils::beacon::BeaconValidator;
use crate::vars::{BoolVariable, Bytes32Variable, U256Variable, WitnessWriteMethods};

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

pub trait BeaconValidatorWitnessWrite<F: Field>: WitnessWriteMethods<F> {
    fn set_validator(&mut self, variable: BeaconValidatorVariable, value: BeaconValidator);
}

impl<F: Field> BeaconValidatorWitnessWrite<F> for GeneratedValues<F> {
    fn set_validator(&mut self, variable: BeaconValidatorVariable, value: BeaconValidator) {
        let bytes = hex::decode(value.pubkey.clone().drain(2..)).unwrap();
        self.set_bls_pubkey(variable.pubkey, bytes.into());
    }
}
