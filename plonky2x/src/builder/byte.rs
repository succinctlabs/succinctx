use crate::builder::BuilderAPI;
use crate::vars::ByteVariable;

impl BuilderAPI {
    pub fn zero_byte(&mut self) -> ByteVariable {
        let zero = self.zero();
        zero.into()
    }
}
