use super::BuilderAPI;
use crate::vars::{BoolVariable, U256Variable};

impl BuilderAPI {
    /// Initialize a new U256Variable.
    pub fn init_u256(&mut self) -> U256Variable {
        let mut bits = [BoolVariable::default(); 256];
        for i in 0..256 {
            bits[i] = self.init_bool();
        }
        U256Variable(bits)
    }
}
