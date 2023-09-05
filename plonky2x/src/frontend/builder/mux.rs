use plonky2::{field::extension::Extendable, iop::target::BoolTarget};
use plonky2::hash::hash_types::RichField;

use crate::prelude::{CircuitBuilder, CircuitVariable, Variable};

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn select_index<T: CircuitVariable>(&mut self, selector: Variable, inputs: &[T]) -> T {
        // Serialized variables for the circuit variable.
        let num_var_in_t = inputs[0].variables().len();
        let input_len = inputs.len();

        let mut res = inputs[0].variables();

        for i in 0..input_len {
            let target_i = self.constant::<Variable>(F::from_canonical_usize(i));
            let diff = self.sub(target_i, selector);
            let whether_select = self.is_zero(diff);

            let vars = inputs[i].variables();
            for j in 0..num_var_in_t {
                res[j] = Variable(self.api.select(BoolTarget::new_unsafe(whether_select.0.0), vars[j].0, res[j].0));
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

    #[test]
    fn test_mux_val_equal() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let inputs = [1,2,3,3,2];

        let mut builder = CircuitBuilder::<F, D>::new();
        let index_one = builder.constant::<Variable>(F::from_noncanonical_u64(1));
        let index_two = builder.constant::<Variable>(F::from_noncanonical_u64(2));

        let input_variables = inputs
            .iter()
            .map(|x| U256Variable::constant(&mut builder, U256::from(*x)))
            .collect::<Vec<_>>();

        let output1 = builder.select_index(index_one, &input_variables[..]);
        let output2 = builder.select_index(index_two, &input_variables[..]);

        builder.assert_is_equal(output1, input_variables[4]);
        builder.assert_is_equal(output2, input_variables[3]);


        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_mux_fake_proof_fail_verify() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let inputs = [1,2,3,3,2];

        let mut builder = CircuitBuilder::<F, D>::new();
        let index_one = builder.constant::<Variable>(F::from_noncanonical_u64(1));

        let input_variables = inputs
            .iter()
            .map(|x| U256Variable::constant(&mut builder, U256::from(*x)))
            .collect::<Vec<_>>();

        let output = builder.select_index(index_one, &input_variables[..]);
      
        builder.assert_is_equal(output, input_variables[0]);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
         // This verification should fail
        //  assert!(circuit.data.verify(proof).is_err());
    }
}
