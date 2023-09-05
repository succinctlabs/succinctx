use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, Variable};

// Checks that a[a_offset:a_offset+len] = b[b_offset:b_offset+len]
pub fn subarray_equal(a: &[u8], a_offset: usize, b: &[u8], b_offset: usize, len: usize) -> u8 {
    for i in 0..len {
        if a[a_offset + i] != b[b_offset + i] {
            return 0;
        }
    }
    return 1;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    fn subarray_equal(
        &mut self,
        a: &[ByteVariable],
        a_offset: Variable,
        b: &[ByteVariable],
        b_offset: Variable,
        len: Variable,
    ) -> BoolVariable {
        todo!();
    }
}

pub(crate) mod tests {
    // TODO add a test for subarray_equal
}
