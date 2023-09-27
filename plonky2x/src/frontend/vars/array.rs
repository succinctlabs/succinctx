use core::fmt::Debug;
use std::ops::{Index, Range};

use itertools::Itertools;
use plonky2::field::types::{Field, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::challenger::RecursiveChallenger;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};
use serde::{Deserialize, Serialize};

use super::{BoolVariable, ByteVariable, CircuitVariable, ValueStream, Variable, VariableStream};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{Add, Mul, Sub};

/// A variable in the circuit representing a fixed length array of variables.
/// We use this to avoid stack overflow arrays associated with fixed-length arrays.
#[derive(Debug, Clone)]
pub struct ArrayVariable<V: CircuitVariable, const N: usize> {
    pub data: Vec<V>,
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

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
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

    pub fn get_fixed_subarray<const MAX_ARRAY_SIZE: usize, const SUB_ARRAY_SIZE: usize>(
        &mut self,
        array: &ArrayVariable<Variable, MAX_ARRAY_SIZE>,
        array_size: Variable,
        start_idx: Variable,
        seed: &[ByteVariable],
    ) -> ArrayVariable<Variable, SUB_ARRAY_SIZE> {
        // TODO:  Need to add check that array_size is less than MAX_ARRAY_SIZE.
        // TODO:  Need to add check that start_idx + SUB_ARRAY_SIZE is less than array_size.
        const MIN_SEED_BITS: usize = 120; // TODO: Seed it with 120 bits.  Need to figure out if this is enough bits of security.

        let mut input_stream = VariableStream::new();
        for i in 0..MAX_ARRAY_SIZE {
            input_stream.write(&array[i]);
        }
        input_stream.write(&array_size);
        input_stream.write(&start_idx);

        let hint = SubArrayExtractorHint {
            max_array_size: MAX_ARRAY_SIZE,
            sub_array_size: SUB_ARRAY_SIZE,
        };
        let output_stream = self.hint(input_stream, hint);

        let mut sub_array_element = Vec::new();
        for _i in 0..SUB_ARRAY_SIZE {
            sub_array_element.push(output_stream.read::<Variable>(self));
        }

        let sub_array = ArrayVariable::<Variable, SUB_ARRAY_SIZE>::from(sub_array_element);

        let mut seed_targets = Vec::new();
        let mut challenger = RecursiveChallenger::<L::Field, PoseidonHash, D>::new(&mut self.api);

        // Need to get chunks of 7 since the max value of F is slightly less then 64 bits.
        let mut seed_bit_len = 0;
        for seed_chunk in seed.to_vec().chunks(7) {
            let seed_element_bits = seed_chunk
                .iter()
                .flat_map(|x| x.as_bool_targets())
                .collect_vec();
            let seed_element = self.api.le_sum(seed_element_bits.iter());
            seed_bit_len += seed_element_bits.len();
            seed_targets.push(seed_element);
        }

        assert!(seed_bit_len >= MIN_SEED_BITS);

        challenger.observe_elements(seed_targets.as_slice());

        for _i in 0..2 {
            let challenges = challenger
                .get_n_challenges(&mut self.api, SUB_ARRAY_SIZE)
                .iter()
                .map(|x| Variable::from(*x))
                .collect_vec();
            let sub_array_size = self.constant(L::Field::from_canonical_usize(SUB_ARRAY_SIZE));
            let end_idx = self.add(start_idx, sub_array_size);
            let mut within_sub_array = self.zero::<Variable>();
            let one = self.one();

            let mut accumulator1 = self.zero::<Variable>();
            let mut j_target = self.zero();
            for j in 0..MAX_ARRAY_SIZE {
                let at_start_idx = self.is_equal(j_target, start_idx);
                within_sub_array = within_sub_array.add(at_start_idx.variables()[0], self);
                let at_end_idx = self.is_equal(j_target, end_idx);
                within_sub_array = within_sub_array.sub(at_end_idx.variables()[0], self);

                let mut subarray_idx = j_target.sub(start_idx, self);
                subarray_idx = subarray_idx.mul(within_sub_array, self);

                let challenge = self.select_array_random_gate(&challenges, subarray_idx);
                let mut product = self.mul(array[j], challenge);
                product = within_sub_array.mul(product, self);
                accumulator1 = accumulator1.add(product, self);

                j_target = j_target.add(one, self);
            }

            let mut accumulator2 = self.zero();
            for j in 0..SUB_ARRAY_SIZE {
                let product = self.mul(sub_array[j], challenges[j]);
                accumulator2 = self.add(accumulator2, product);
            }

            self.assert_is_equal(accumulator1, accumulator2);
        }

        sub_array
    }

    pub fn array_contains<V: CircuitVariable>(&mut self, array: &[V], element: V) -> BoolVariable {
        let mut accumulator = self.constant::<Variable>(L::Field::from_canonical_usize(0));

        for i in 0..array.len() {
            let element_equal = self.is_equal(array[i].clone(), element.clone());
            accumulator = self.add(accumulator, element_equal.0);
        }

        let one = self.constant::<Variable>(L::Field::from_canonical_usize(1));
        self.le(one, accumulator)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubArrayExtractorHint {
    max_array_size: usize,
    sub_array_size: usize,
}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for SubArrayExtractorHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let mut array_elements = Vec::new();

        for _i in 0..self.max_array_size {
            let element = input_stream.read_value::<Variable>();
            array_elements.push(element);
        }

        let array_size = input_stream.read_value::<Variable>().to_canonical_u64();
        let start_idx = input_stream.read_value::<Variable>().to_canonical_u64();
        let end_idx = start_idx + self.sub_array_size as u64;

        assert!(array_size <= self.max_array_size as u64);
        assert!(end_idx <= array_size);

        for i in 0..self.sub_array_size {
            let element = array_elements[start_idx as usize + i];
            output_stream.write_value::<Variable>(element);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use ethers::types::U256;
    use log::debug;
    use plonky2::field::types::Field;
    use rand::rngs::OsRng;
    use rand::Rng;

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::U256Variable;
    use crate::prelude::*;
    use crate::utils;

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
        utils::setup_logger();
        type F = GoldilocksField;
        const INPUT_SIZE: usize = 1000;

        let mut builder = DefaultBuilder::new();
        let b = builder.read::<ArrayVariable<U256Variable, INPUT_SIZE>>();
        let selector = builder.read::<Variable>();
        let result = builder.select_array(b.as_slice(), selector);
        builder.write(result);

        let num_gates = builder.api.num_gates();
        debug!("num_gates: {}", num_gates);

        let start = Instant::now();
        let circuit = builder.build();
        let duration = start.elapsed();
        debug!("Build time: {:?}", duration);

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
        debug!("Prove time: {:?}", start.elapsed());

        circuit.verify(&proof, &input, &output);

        assert_eq!(output.read::<U256Variable>(), input_u256[1]);
    }

    #[test]
    fn test_select_index_random_gate() {
        utils::setup_logger();
        type F = GoldilocksField;
        const INPUT_SIZE: usize = 10;

        let mut builder = DefaultBuilder::new();
        let b = builder.read::<ArrayVariable<U256Variable, INPUT_SIZE>>();
        let selector = builder.read::<Variable>();
        let result = builder.select_array_random_gate(b.as_slice(), selector);
        builder.write(result);

        let num_gates = builder.api.num_gates();
        debug!("num_gates: {}", num_gates);

        let start = Instant::now();
        let circuit = builder.build();
        let duration = start.elapsed();
        debug!("Build time: {:?}", duration);

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
        debug!("Prove time: {:?}", start.elapsed());

        circuit.verify(&proof, &input, &output);

        assert_eq!(output.read::<U256Variable>(), input_u256[1]);
    }

    #[test]
    fn test_get_fixed_subarray() {
        utils::setup_logger();
        type F = GoldilocksField;
        const MAX_ARRAY_SIZE: usize = 100;
        const SUB_ARRAY_SIZE: usize = 10;
        const START_IDX: usize = 15;

        let mut builder = DefaultBuilder::new();

        let array = builder.read::<ArrayVariable<Variable, MAX_ARRAY_SIZE>>();
        let array_size = builder.read::<Variable>();
        let start_idx = builder.constant(F::from_canonical_usize(START_IDX));
        let seed = builder.read::<Bytes32Variable>();
        let result = builder.get_fixed_subarray::<MAX_ARRAY_SIZE, SUB_ARRAY_SIZE>(
            &array,
            array_size,
            start_idx,
            &seed.as_bytes(),
        );
        builder.write(result);

        let circuit = builder.build();

        // The last 20 elements are dummy
        let mut rng = OsRng;
        let mut array_input = [F::default(); MAX_ARRAY_SIZE];
        for elem in array_input.iter_mut() {
            *elem = F::from_canonical_u64(rng.gen());
        }
        let array_size_input = F::from_canonical_usize(80);

        let mut seed_input = [0u8; 15];
        for elem in seed_input.iter_mut() {
            *elem = rng.gen();
        }

        let mut input = circuit.input();
        input.write::<ArrayVariable<Variable, MAX_ARRAY_SIZE>>(array_input.to_vec());
        input.write::<Variable>(array_size_input);
        input.write::<Bytes32Variable>(bytes32!(
            "0x7c38fc8356aa20394c7f538e3cee3f924e6d9252494c8138d1a6aabfc253118f"
        ));

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let expected_sub_array = array_input[START_IDX..START_IDX + SUB_ARRAY_SIZE].to_vec();
        assert_eq!(
            output.read::<ArrayVariable<Variable, SUB_ARRAY_SIZE>>(),
            expected_sub_array
        );
    }
}
