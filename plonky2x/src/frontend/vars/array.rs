use core::fmt::Debug;
use std::ops::{Index, Range};

use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
/// A variable in the circuit representing a fixed length array of variables.
/// We use this to avoid stack overflow arrays associated with fixed-length arrays.
#[derive(Debug, Clone)]
pub struct ArrayVariable<V: CircuitVariable, const N: usize> {
    data: Vec<V>,
}

impl<V: CircuitVariable, const N: usize> ArrayVariable<V, N> {
    pub fn new(elements: Vec<V>) -> Self {
        assert_eq!(elements.len(), N);
        Self { data: elements }
    }

    pub fn as_slice(&self) -> &[V] {
        &self.data
    }

    pub fn as_vec(&self) -> Vec<V> {
        self.data.clone()
    }
}

impl<V: CircuitVariable, const N: usize> Index<usize> for ArrayVariable<V, N> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<V: CircuitVariable, const N: usize> Index<Range<usize>> for ArrayVariable<V, N> {
    type Output = [V];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl<V: CircuitVariable, const N: usize> From<Vec<V>> for ArrayVariable<V, N> {
    fn from(elements: Vec<V>) -> Self {
        ArrayVariable::new(elements)
    }
}

impl<V: CircuitVariable, const N: usize> CircuitVariable for ArrayVariable<V, N> {
    type ValueType<F: RichField> = Vec<V::ValueType<F>>;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            data: (0..N).map(|_| V::init_unsafe(builder)).collect(),
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Vec<V::ValueType<L::Field>>,
    ) -> Self {
        assert_eq!(value.len(), N);
        Self {
            data: value.into_iter().map(|x| V::constant(builder, x)).collect(),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.data.iter().flat_map(|x| x.variables()).collect()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), N * V::nb_elements());
        let mut res = Vec::new();
        for i in 0..N {
            let start = i * V::nb_elements();
            let end = (i + 1) * V::nb_elements();
            let slice = &variables[start..end];
            res.push(V::from_variables_unsafe(slice));
        }

        Self { data: res }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        for element in self.data.iter() {
            element.assert_is_valid(builder);
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.data.iter().map(|x| x.get(witness)).collect()
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        assert_eq!(value.len(), N);
        for (element, value) in self.data.iter().zip(value) {
            element.set(witness, value);
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Given an `array` of variables, and a dynamic `selector`, returns `array[selector]` as a variable.
    pub fn select_array<V: CircuitVariable>(&mut self, array: &[V], selector: Variable) -> V {
        // The accumulator holds the variable of the selected result
        let mut accumulator = array[0].clone();

        for i in 0..array.len() {
            // Whether the accumulator should be set to the i-th element (if selector_enabled=true)
            // Or should be set to the previous value (if selector_enabled=false)
            let target_i = self.constant::<Variable>(L::Field::from_canonical_usize(i));
            let selector_enabled = self.is_equal(target_i, selector);
            // If selector_enabled, then accum_var gets set to arr_var, otherwise it stays the same
            accumulator = self.select(selector_enabled, array[i].clone(), accumulator);
        }

        accumulator
    }

    /// Given an `array` of variables, and a dynamic `selector`, returns `array[selector]` as a variable using the random access gate.
    /// This should only be used in cases where the CircuitVariable has a very small number of variables, otherwise the `random_access` gate will blow up
    pub fn select_array_random_gate<V: CircuitVariable>(
        &mut self,
        array: &[V],
        selector: Variable,
    ) -> V {
        let num_var_in_t = array[0].variables().len();

        let mut selected_vars = Vec::new();

        for i in 0..num_var_in_t {
            // For each array entry, get the i-th variable
            let mut pos_i_targets: Vec<Target> = (0..array.len())
                .map(|j| array[j].variables()[i].0)
                .collect();
            // Pad the length of pos_vars to the nearest power of 2, random_access constrain len of selected vec to be power of 2
            let padded_len = pos_i_targets.len().next_power_of_two();
            pos_i_targets.resize_with(padded_len, Default::default);

            selected_vars.push(Variable(self.api.random_access(selector.0, pos_i_targets)));
        }

        V::from_variables_unsafe(&selected_vars)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use ethers::types::U256;
    use plonky2::field::types::Field;
    use rand::rngs::OsRng;
    use rand::Rng;

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::U256Variable;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_array_variable() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let x = builder.init::<BoolVariable>();
        let y = builder.init::<BoolVariable>();
        let array = ArrayVariable::<_, 2>::new(vec![x, y]);

        let mut pw = PartialWitness::new();

        x.set(&mut pw, true);
        y.set(&mut pw, false);
        array.set(&mut pw, vec![true, false]);

        let circuit = builder.build();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_select_index() {
        type F = GoldilocksField;
        const INPUT_SIZE: usize = 1000;

        let mut builder = DefaultBuilder::new();
        let b = builder.read::<ArrayVariable<U256Variable, INPUT_SIZE>>();
        let selector = builder.read::<Variable>();
        let result = builder.select_array(b.as_slice(), selector);
        builder.write(result);

        let num_gates = builder.api.num_gates();
        println!("num_gates: {}", num_gates);

        let start = Instant::now();
        let circuit = builder.build();
        let duration = start.elapsed();
        println!("Build time: {:?}", duration);

        let mut input = circuit.input();

        let mut rng = OsRng;
        let mut random_input = [0u64; INPUT_SIZE];
        for elem in random_input.iter_mut() {
            *elem = rng.gen();
        }
        let input_u256: Vec<U256> = random_input
            .iter()
            .map(|x| U256::from(*x))
            .collect::<Vec<_>>();

        input.write::<ArrayVariable<U256Variable, INPUT_SIZE>>(input_u256.clone());
        input.write::<Variable>(F::from_canonical_u16(1));

        let start = Instant::now();
        let (proof, mut output) = circuit.prove(&input);
        println!("Prove time: {:?}", start.elapsed());

        circuit.verify(&proof, &input, &output);

        assert_eq!(output.read::<U256Variable>(), input_u256[1]);
    }

    #[test]
    fn test_select_index_random_gate() {
        type F = GoldilocksField;
        const INPUT_SIZE: usize = 10;

        let mut builder = DefaultBuilder::new();
        let b = builder.read::<ArrayVariable<U256Variable, INPUT_SIZE>>();
        let selector = builder.read::<Variable>();
        let result = builder.select_array_random_gate(b.as_slice(), selector);
        builder.write(result);

        let num_gates = builder.api.num_gates();
        println!("num_gates: {}", num_gates);

        let start = Instant::now();
        let circuit = builder.build();
        let duration = start.elapsed();
        println!("Build time: {:?}", duration);

        let mut input = circuit.input();

        let mut rng = OsRng;
        let mut random_input = [0u64; INPUT_SIZE];
        for elem in random_input.iter_mut() {
            *elem = rng.gen();
        }
        let input_u256: Vec<U256> = random_input
            .iter()
            .map(|x| U256::from(*x))
            .collect::<Vec<_>>();

        input.write::<ArrayVariable<U256Variable, INPUT_SIZE>>(input_u256.clone());
        input.write::<Variable>(F::from_canonical_u16(1));

        let start = Instant::now();
        let (proof, mut output) = circuit.prove(&input);
        println!("Prove time: {:?}", start.elapsed());

        circuit.verify(&proof, &input, &output);

        assert_eq!(output.read::<U256Variable>(), input_u256[1]);
    }
}
