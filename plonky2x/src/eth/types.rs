use crate::builder::BuilderAPI;
use crate::vars::{BoolVariable, BytesVariable};


#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub [BoolVariable; 512]);
// impl_variable_methods!(BLSPubkeyVariable, 512);

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub [BoolVariable; 160]);

impl From<AddressVariable> for BytesVariable<20> {
    fn from(value: AddressVariable) -> Self {
        let mut result: [[BoolVariable; 8]; 20] = Default::default();

        for (i, item) in value.0.iter().enumerate() {
            result[i / 8][i % 8] = *item;
        }

        BytesVariable::<20>(result)
    }
}

impl BuilderAPI {
    /// Initialize a new BLSPubkeyVariable.
    pub fn init_bls_pubkey(&mut self) -> BLSPubkeyVariable {
        BLSPubkeyVariable([self.init_bool(); 512])
    }
}
