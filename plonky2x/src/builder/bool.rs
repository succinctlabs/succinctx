use plonky2::iop::target::BoolTarget;

use crate::builder::API;
use crate::vars::{BoolVariable, ByteTarget};

impl API {
    fn or(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        let a = BoolTarget::new_unsafe(i1.value);
        let b = BoolTarget::new_unsafe(i2.value);
        BoolVariable::from_target(self.api.or(a, b).target)
    }

    fn and(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        let a = BoolTarget::new_unsafe(i1.value);
        let b = BoolTarget::new_unsafe(i2.value);
        BoolVariable::from_target(self.api.and(a, b).target)
    }

    // fn xor(&self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
    //     let a = BoolTarget::new_unsafe(i1.value);
    //     let b = BoolTarget::new_unsafe(i2.value);
    //     BoolVariable::from_target(self.api.xor(a, b))
    // }

    fn not(&mut self, i1: BoolVariable) -> BoolVariable {
        let a = BoolTarget::new_unsafe(i1.value);
        BoolVariable::from_target(self.api.not(a).target)
    }

    // fn to_binary_le(&self, i1: BoolVariable) -> ByteTarget {
    //     ByteTarget::from_target(self.api.to_binary_le(i1.value))
    // }
}
