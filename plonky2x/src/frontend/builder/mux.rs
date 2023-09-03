use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::prelude::{CircuitBuilder, CircuitVariable, Variable};

trait MuxBuilder {
    fn select_index<T: CircuitVariable>(&mut self, selector: Variable, inputs: &[T]) -> T;
}

impl<F: RichField + Extendable<D>, const D: usize> MuxBuilder for CircuitBuilder<F, D> {
    fn select_index<T: CircuitVariable>(&mut self, selector: Variable, inputs: &[T]) -> T {
        let num_var_in_t = inputs[0].variables().len();
        let input_len = inputs.len();
        let mut res = (0..num_var_in_t).map(|_| self.init::<Variable>()).collect::<Vec<_>>();
        let api = &mut self.api;

        for i in 0..input_len {
            let target_i = api.constant(F::from_canonical_usize(i));
            let whether_select = api.is_equal(target_i, selector.0);

            let vars = inputs[i].variables();
            for j in 0..num_var_in_t {
                res[j] = Variable(api.select(whether_select, vars[j].0, res[j].0));
            }
        }

        T::from_variables(&res)
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
