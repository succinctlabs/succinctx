use std::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::mpt::generators::MuxGenerator;
use crate::frontend::vars::{ArrayVariable, CircuitVariable, Variable};

/// Given an ArrayVariable and a selector that is a Variable, returns the
/// `selector` element of the `array`.
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
    use curta::math::prelude::Field;
    use plonky2::field::goldilocks_field::GoldilocksField;

    use super::*;
    use crate::prelude::{BoolVariable, PoseidonGoldilocksConfig};

    #[test]
    fn test_mux() {
        type F = GoldilocksField;
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;

        let mut builder = CircuitBuilder::<F, D>::new();
        let b = builder.read::<ArrayVariable<BoolVariable, 3>>();
        let selector = builder.read::<Variable>();
        let result = builder.mux(b, selector);
        builder.write(result);

        let circuit = builder.build::<C>();
        let mut input = circuit.input();
        input.write::<ArrayVariable<BoolVariable, 3>>(vec![true, false, true]);
        input.write::<Variable>(F::from_canonical_u16(1));
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
