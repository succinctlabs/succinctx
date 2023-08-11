use super::BuilderAPI;
use crate::vars::{BoolVariable, Bytes32Variable};

// pub struct Bytes32Variable(pub [BoolVariable; 256]);

impl BuilderAPI {
    /// Initialize a new Bytes32Variable.
    pub fn init_bytes32(&mut self) -> Bytes32Variable {
        let mut bits = [BoolVariable::default(); 256]; // Assuming default is available for BoolVariable
        for i in 0..256 {
            bits[i] = self.init_bool();
        }
        Bytes32Variable(bits)
    }

    // TODO I'm not sure if this is a great name for this function
    // Or whether we should put it in another impl
    // TODO, make this generic for bytes and then use .into()
    pub fn assert_is_equal_bytes32(&mut self, i1: Bytes32Variable, i2: Bytes32Variable) {
        for i in 0..256 {
            self.assert_is_equal(i1.0[i].0, i2.0[i].0)
        }
    }
}
