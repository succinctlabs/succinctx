mod generators;
mod vars;

use self::vars::BeaconValidatorVariable;
use crate::builder::CircuitBuilder;
use crate::vars::BoolVariable;

/// An API for methods relating to the consensus layer state of Ethereum.
pub struct BeaconAPI<'a> {
    pub api: &'a mut CircuitBuilder,
    pub consensus_rpc: String,
}

impl<'a> BeaconAPI<'a> {
    // Create a new BeaconAPI.
    pub fn new(api: &'a mut CircuitBuilder, consensus_rpc: String) -> Self {
        Self { api, consensus_rpc }
    }

    /// Initialize a new BeaconValidatorVariable.
    pub fn init_validator(&mut self) -> BeaconValidatorVariable {
        BeaconValidatorVariable {
            pubkey: self.api.init_bls_pubkey(),
            withdrawal_credentials: self.api.init_bytes32(),
            effective_balance: self.api.init_u256(),
            slashed: self.api.init_bool(),
            activation_eligibility_epoch: self.api.init_u256(),
            activation_epoch: self.api.init_u256(),
            exit_epoch: self.api.init_u256(),
            withdrawable_epoch: self.api.init_u256(),
        }
    }

    pub fn get_validator(
        &mut self,
        _header_root: [BoolVariable; 256],
        _idx: usize,
    ) -> BeaconValidatorVariable {
        let validator = self.init_validator();
        validator
    }
}
