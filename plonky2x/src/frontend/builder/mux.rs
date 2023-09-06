use std::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::mpt::reference::generators::MuxGenerator;
use crate::frontend::vars::{ArrayVariable, CircuitVariable, Variable};

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn mux<V: CircuitVariable, const N: usize>(
        &mut self,
        array: ArrayVariable<V, N>,
        selector: Variable,
    ) -> V {
        let generator = MuxGenerator {
            input: array,
            select: selector,
            output: self.init::<V>(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(&generator);
        generator.output
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::{PartialWitness, Witness, WitnessWrite};

    use super::*;
    use crate::prelude::BoolVariable;

    #[test]
    fn test_mux() {
        type F = GoldilocksField;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let b = builder.init::<ArrayVariable<BoolVariable, 3>>();
        let selector = builder.init::<Variable>();
        let result = builder.mux(b, selector);

        // let mut pw = PartialWitness::new();
        // b.set(&mut pw, vec![true, false, true]);
        // selector.set(&mut pw, 1);

        // // let value = pw.try_get_target(b.0 .0).unwrap();
        // assert_eq!(GoldilocksField::ONE, value);
    }
}
