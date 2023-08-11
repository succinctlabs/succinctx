use super::vars::{AddressVariable, BLSPubkeyVariable};
use crate::builder::BuilderAPI;
use crate::vars::BytesVariable;

impl BuilderAPI {
    /// Initialize a new BLSPubkeyVariable.
    pub fn init_bls_pubkey(&mut self) -> BLSPubkeyVariable {
        BLSPubkeyVariable(BytesVariable::<48>::new())
    }

    /// Initialize a new AddressVariable.
    pub fn init_address(&mut self) -> AddressVariable {
        AddressVariable(BytesVariable::<20>::new())
    }
}
