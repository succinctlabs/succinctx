use core::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, Range};

use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::mpt::generators::MuxGenerator;
/// A variable in the circuit representing a fixed length array of variables.
/// We use this to avoid stack overflow arrays associated with fixed-length arrays.
#[derive(Debug, Clone)]
pub struct ArrayVariable<V: CircuitVariable, const N: usize> {
    elements: Vec<V>,
}

impl<V: CircuitVariable, const N: usize> ArrayVariable<V, N> {
    pub fn new(elements: Vec<V>) -> Self {
        assert_eq!(elements.len(), N);
        Self { elements }
    }

    pub fn as_slice(&self) -> &[V] {
        &self.elements
    }

    pub fn as_vec(&self) -> Vec<V> {
        self.elements.clone()
    }
}

impl<V: CircuitVariable, const N: usize> Index<usize> for ArrayVariable<V, N> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<V: CircuitVariable, const N: usize> Index<Range<usize>> for ArrayVariable<V, N> {
    type Output = [V];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.elements[range]
    }
}

impl<V: CircuitVariable, const N: usize> From<Vec<V>> for ArrayVariable<V, N> {
    fn from(elements: Vec<V>) -> Self {
        ArrayVariable::new(elements)
    }
}

impl<V: CircuitVariable, const N: usize> CircuitVariable for ArrayVariable<V, N> {
    type ValueType<F: RichField> = Vec<V::ValueType<F>>;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            elements: (0..N).map(|_| V::init(builder)).collect(),
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Vec<V::ValueType<L::Field>>,
    ) -> Self {
        assert_eq!(value.len(), N);
        Self {
            elements: value.into_iter().map(|x| V::constant(builder, x)).collect(),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.elements.iter().flat_map(|x| x.variables()).collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), N * V::nb_elements());
        let mut res = Vec::new();
        for i in 0..N {
            let start = i * V::nb_elements();
            let end = (i + 1) * V::nb_elements();
            let slice = &variables[start..end];
            res.push(V::from_variables(slice));
        }

        Self { elements: res }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.elements.iter().map(|x| x.get(witness)).collect()
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        assert_eq!(value.len(), N);
        for (element, value) in self.elements.iter().zip(value) {
            element.set(witness, value);
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Given an `array` of variables, and a dynamic `selector`, returns `array[selector]` as a variable.
    pub fn select_index<V: CircuitVariable, const N: usize>(
        &mut self,
        array: ArrayVariable<V, N>,
        selector: Variable,
    ) -> V {
        // The accumulator holds a Vec<Variable> corresponding the variables of the selected result
        let mut accumulator = array[0].variables();

        for i in 0..N {
            // Whether the accumulator should be set to the i-th element (if selector_enabled=true)
            // Or should be set to the previous value (if selector_enabled=false)
            let target_i = self.constant::<Variable>(L::Field::from_canonical_usize(i));
            let selector_enabled = self.is_equal(target_i, selector);

            for (accum_var, arr_var) in accumulator.iter_mut().zip(array[i].variables()) {
                // If selector_enabled, then accum_var gets set to arr_var, otherwise it stays the same
                *accum_var = Variable(self.api.select(
                    BoolTarget::new_unsafe(selector_enabled.0 .0),
                    arr_var.0,
                    accum_var.0,
                ));
            }
        }

        V::from_variables(&accumulator)
    }

    /// Given an `array` of variables, and a dynamic `selector`, returns `array[selector]` as a variable using the random access gate.
    pub fn select_index_random_gate<V: CircuitVariable, const N: usize>(
        &mut self,
        array: ArrayVariable<V, N>,
        selector: Variable,
    ) -> V {
        let num_var_in_t = array[0].variables().len();

        if num_var_in_t > 80 {
            panic!("Cannot use random access gate with Variable composed of > 80 FieldVariables because of width of circuit");
        }

        let mut selected_vars = Vec::new();

        for i in 0..num_var_in_t {
            // For each array entry, get the i-th variable
            let mut pos_i_targets: Vec<Target> =
                (0..N).map(|j| array[j].variables()[i].0).collect();
            // Pad the length of pos_vars to the nearest power of 2, random_access constrain len of selected vec to be power of 2
            let padded_len = pos_i_targets.len().next_power_of_two();
            pos_i_targets.resize_with(padded_len, Default::default);

            selected_vars.push(Variable(self.api.random_access(selector.0, pos_i_targets)));
        }

        V::from_variables(&selected_vars)
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
    fn test_select_index() {
        type F = GoldilocksField;
        const INPUT_SIZE: usize = 1000;

        let mut builder = DefaultBuilder::new();
        let b = builder.read::<ArrayVariable<U256Variable, INPUT_SIZE>>();
        let selector = builder.read::<Variable>();
        let result = builder.select_index(b, selector);
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
        let result = builder.select_index_random_gate(b, selector);
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
