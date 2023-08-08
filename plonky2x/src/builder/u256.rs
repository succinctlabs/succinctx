use super::BuilderAPI;
use crate::vars::U256Variable;

impl BuilderAPI {
    /// Initialize a new U256Variable.
    pub fn init_u256(&mut self) -> U256Variable {
        U256Variable([self.init_bool(); 256])
    }
}
