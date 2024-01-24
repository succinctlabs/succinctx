use std::marker::PhantomData;

use super::generators::SubarrayEqualGenerator;
use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, PlonkParameters, Variable};

// Checks that a[a_offset:a_offset+len] = b[b_offset:b_offset+len]
pub fn subarray_equal(a: &[u8], a_offset: usize, b: &[u8], b_offset: usize, len: usize) -> u8 {
    for i in 0..len {
        if a[a_offset + i] != b[b_offset + i] {
            return 0;
        }
    }
    1
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    #[allow(unused_variables, dead_code)]
    pub fn subarray_equal(
        &mut self,
        a: &[ByteVariable],
        a_offset: Variable,
        b: &[ByteVariable],
        b_offset: Variable,
        len: Variable,
    ) -> BoolVariable {
        todo!();
    }

    #[allow(unused_variables, dead_code)]
    pub fn assert_subarray_equal(
        &mut self,
        a: &[ByteVariable],
        a_offset: Variable,
        b: &[ByteVariable],
        b_offset: Variable,
        len: Variable,
    ) {
        // TODO: implement
        let generator: SubarrayEqualGenerator<L, D> = SubarrayEqualGenerator {
            a: a.to_vec(),
            a_offset,
            b: b.to_vec(),
            b_offset,
            len,
            _phantom: PhantomData::<L>,
        };
        self.add_simple_generator(generator);
    }
}

pub(crate) mod tests {
    // TODO add a test for subarray_equal
}
