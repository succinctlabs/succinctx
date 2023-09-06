use std::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::SubarrayEqualGenerator;
use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, Variable};

// Checks that a[a_offset:a_offset+len] = b[b_offset:b_offset+len]
pub fn subarray_equal(a: &[u8], a_offset: usize, b: &[u8], b_offset: usize, len: usize) -> u8 {
    for i in 0..len {
        if a[a_offset + i] != b[b_offset + i] {
            return 0;
        }
    }
    1
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
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
        let generator = SubarrayEqualGenerator {
            a: a.to_vec(),
            a_offset,
            b: b.to_vec(),
            b_offset,
            len,
            _phantom: PhantomData,
        };
        self.add_simple_generator(&generator);
        // TODO: implement
        // Pass for now so that circuit builder doesn't complaint
    }
}

pub(crate) mod tests {
    // TODO add a test for subarray_equal
}
