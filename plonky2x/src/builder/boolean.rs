use crate::builder::BuilderAPI;
use crate::vars::BoolVariable;

impl BuilderAPI {
    pub fn init_bool(&mut self) -> BoolVariable {
        self.api.add_virtual_target().into()
    }

    pub fn f(&mut self) -> BoolVariable {
        let zero = self.zero();
        zero.into()
    }

    pub fn t(&mut self) -> BoolVariable {
        let one = self.one();
        one.into()
    }

    /// Computes the or of two bits or i1 | i2.
    pub fn or(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        self.add(i1.0, i2.0).into()
    }

    /// Computes the and of two bits or i1 & i2.
    pub fn and(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        self.mul(i1.0, i2.0).into()
    }

    /// Computes the xor of two bits or i1 ^ i2.
    pub fn xor(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        let a_plus_b = self.add(i1.0, i2.0);
        let two_a_b = self.mul(i1.0, i2.0);
        self.sub(a_plus_b, two_a_b).into()
    }

    /// Computes the not of a bit or !i1.
    pub fn not(&mut self, i1: BoolVariable) -> BoolVariable {
        let one = self.one();
        self.sub(one, i1.0).into()
    }
}
