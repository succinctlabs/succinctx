use std::marker::PhantomData;

use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::challenger::RecursiveChallenger;

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
        // TODO: instead of using the SubarrayEqualGenerator below that doesn't actually check anything, implement an RLC check here
        let generator: SubarrayEqualGenerator<L, D> = SubarrayEqualGenerator {
            a: a.to_vec(),
            a_offset,
            b: b.to_vec(),
            b_offset,
            len,
            _phantom: PhantomData::<L>,
        };
        self.add_simple_generator(generator);

        // The following methods might be helpful
        let mut challenger = RecursiveChallenger::<L::Field, PoseidonHash, D>::new(&mut self.api);
        let challenger_seed = Vec::new(); // TODO: have to "seed" the challenger with some random inputs from the circuit
        challenger.observe_elements(&challenger_seed);

        let random_variables = challenger.get_n_challenges(&mut self.api, 1);
        let random_variable = random_variables[0];

        // To convert from a Target to a Variable, just use Variable(my_target) to get a Variable

        // TODO: now compute a commitment to a[a_offset:a_offset+len]
        // TODO: now compute a commitment to b[b_offset:b_offset+len]
    }
}

pub(crate) mod tests {
    // TODO add a test for subarray_equal
}
