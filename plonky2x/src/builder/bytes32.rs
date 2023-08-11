use super::BuilderAPI;
use crate::vars::{BoolVariable, Bytes32Variable};

impl BuilderAPI {
    /// Initialize a new Bytes32Variable.
    pub fn init_bytes32(&mut self) -> Bytes32Variable {
        let mut bytes = [BoolVariable::default(); 256];
        for i in 0..256 {
            bytes[i] = self.init_bool();
        }
        Bytes32Variable(bytes)
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
