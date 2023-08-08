use super::BuilderAPI;
use crate::vars::Bytes32Variable;

impl BuilderAPI {
    /// Initialize a new Bytes32Variable.
    pub fn init_bytes32(&mut self) -> Bytes32Variable {
        Bytes32Variable([self.init_bool(); 256])
    }
}
