use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::prelude::{CircuitBuilder, CircuitVariable, Variable};

trait MuxBuilder {
    fn select_index<T: CircuitVariable>(&mut self, selector: Variable, inputs: &[T]) -> T;
}

impl<F: RichField + Extendable<D>, const D: usize> MuxBuilder for CircuitBuilder<F, D> {
    fn select_index<T: CircuitVariable>(&mut self, selector: Variable, inputs: &[T]) -> T {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use ethers::types::U256;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    use rand::rngs::OsRng;
    use rand::Rng;

    use crate::frontend::builder::mux::MuxBuilder;
    use crate::frontend::vars::U256Variable;
    use crate::prelude::{CircuitBuilder, CircuitVariable, GoldilocksField, Variable};

    #[test]
    fn test_mux() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let five = builder.constant::<Variable>(F::from_noncanonical_u64(5));

        // random inputs
        let mut rng = OsRng;
        let inputs = rng.gen::<[u64; 10]>();
        let input_variables = inputs
            .iter()
            .map(|x| U256Variable::constant(&mut builder, U256::from(*x)))
            .collect::<Vec<_>>();

        let output = builder.select_index(five, &input_variables[..]);

        builder.assert_is_equal(output, input_variables[5]);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
