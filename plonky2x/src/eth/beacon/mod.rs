mod generators;
mod validator;

use self::validator::BeaconValidatorVariable;
use crate::builder::BuilderAPI;
use crate::vars::BoolVariable;

/// An API for methods relating to the consensus layer state of Ethereum.
pub struct BeaconAPI {
    pub api: BuilderAPI,
    pub consensus_rpc: String,
}

impl BeaconAPI {
    // Create a new BeaconAPI.
    pub fn new(api: BuilderAPI, consensus_rpc: String) -> Self {
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
