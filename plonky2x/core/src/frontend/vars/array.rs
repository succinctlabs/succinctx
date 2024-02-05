use core::fmt::Debug;
use std::ops::{Index, Range};

use itertools::Itertools;
use plonky2::field::types::{Field, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::challenger::RecursiveChallenger;
use plonky2::iop::target::Target;
use serde::{Deserialize, Serialize};

use super::{ByteVariable, CircuitVariable, ValueStream, Variable, VariableStream};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::hint::simple::hint::Hint;

/// A variable in the circuit representing a fixed length array of variables.
/// We use this to avoid stack overflow arrays associated with fixed-length arrays.
#[derive(Debug, Clone, Eq, PartialEq)]
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

    fn nb_elements() -> usize {
        N * V::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        value
            .into_iter()
            .flat_map(|x| V::elements(x))
            .collect::<Vec<_>>()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(elements.len(), N * V::nb_elements());
        let mut res = Vec::new();
        for i in 0..N {
            let start = i * V::nb_elements();
            let end = (i + 1) * V::nb_elements();
            let slice = &elements[start..end];
            res.push(V::from_elements(slice));
        }

        res
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Given `array` of variables and dynamic `selector`, returns `array[selector]` as a variable.
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

    /// Given an `array` of variables, and a dynamic `selector`, returns `array[selector]` as a
    /// variable using the random access gate. This should only be used in cases where the
    /// CircuitVariable has a very small number of variables, otherwise the `random_access` gate
    /// will blow up.
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
            // Pad the length of pos_vars to the nearest power of 2, random_access constrains the
            // length of the selected vec to be power of 2
            let padded_len = pos_i_targets.len().next_power_of_two();
            pos_i_targets.resize_with(padded_len, Default::default);

            selected_vars.push(Variable(self.api.random_access(selector.0, pos_i_targets)));
        }

        // "selected_vars" will effectively contain the "variables()" representation of the
        // selected element within "array", and that representation is safe (as long as all the
        // elements in "array" are safe).
        // So we can simply reconstruct the selected element from "selected_vars" without needing
        // any checks.
        V::from_variables_unsafe(&selected_vars)
    }

    /// Given an `array` of ByteVariable's, a dynamic `index` start_idx, and a commitment to the
    /// `array`, 'seed', return `array[start_idx..start_idx+sub_array_size]` as an `array`.
    /// `seed` is used to generate randomness for the proof, and must contain a valid commitment to
    /// array_bytes (i.e. either the bytes themselves or a hash of the bytes). This function
    /// generates a Fiat-Shamir seed from the supplied commitmnet to the array and subarray.
    pub fn get_fixed_subarray<const ARRAY_SIZE: usize, const SUB_ARRAY_SIZE: usize>(
        &mut self,
        array_bytes: &ArrayVariable<ByteVariable, ARRAY_SIZE>,
        start_idx: Variable,
        seed: &[ByteVariable],
    ) -> ArrayVariable<ByteVariable, SUB_ARRAY_SIZE> {
        let mut input_stream = VariableStream::new();
        input_stream.write(array_bytes);
        input_stream.write(&start_idx);
        let hint = SubArrayExtractorHint {
            array_size: ARRAY_SIZE,
            sub_array_size: SUB_ARRAY_SIZE,
        };
        let output_stream = self.hint(input_stream, hint);
        let sub_array = output_stream.read::<ArrayVariable<ByteVariable, SUB_ARRAY_SIZE>>(self);

        // The final seed is generated from the seed (which is a commitment to the array)
        // concatenated to the sub_array. seed is Vec<ByteVariable> because it enables packing
        // 7 ByteVariable's into a single Variable, which is useful for the seed's poseidon hashing.
        let mut final_seed = seed.to_vec();
        final_seed.extend_from_slice(sub_array.as_slice());

        // extract_subarray expect the array and subarray to contain variables, so convert
        // the bytes to variables (with each variable containing a single byte).
        let array_variables = ArrayVariable::<Variable, ARRAY_SIZE>::from(
            array_bytes
                .as_slice()
                .iter()
                .map(|x| x.to_variable(self))
                .collect_vec(),
        );
        let subarray_variables = ArrayVariable::<Variable, SUB_ARRAY_SIZE>::from(
            sub_array
                .as_slice()
                .iter()
                .map(|x| x.to_variable(self))
                .collect_vec(),
        );

        self.extract_subarray(
            &array_variables,
            &subarray_variables,
            start_idx,
            &final_seed,
        );
        sub_array
    }

    /// Verify that sub_array is a valid subarray of the array given the start_idx.
    ///
    /// The security of each challenge is log2(field_size) - log2(array_size), so the total security
    /// is (log2(field_size) - log2(array_size)) * num_loops.
    ///
    /// This function does the following to extract the subarray:
    ///     1) Generate a random challenge for each loop, which is referred to as r.
    ///     2) If within the subarray, multiply subarray[i] by r^i and add to the accumulator.
    ///         a) i is the index within the subarray.
    ///         b) r^i is the challenge raised to the power of i.
    ///     3) If outside of the subarray, don't add to the accumulator.
    ///     4) Assert that the accumulator is equal to the accumulator from the given subarray.
    pub fn extract_subarray<const ARRAY_SIZE: usize, const SUB_ARRAY_SIZE: usize>(
        &mut self,
        array: &ArrayVariable<Variable, ARRAY_SIZE>,
        sub_array: &ArrayVariable<Variable, SUB_ARRAY_SIZE>,
        start_idx: Variable,
        seed: &[ByteVariable],
    ) {
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
        // Seed with at least 120 bits. TODO: Check if this is enough bits of security.
        const MIN_SEED_BITS: usize = 120;

        assert!(seed_bit_len >= MIN_SEED_BITS);

        challenger.observe_elements(seed_targets.as_slice());

        const NUM_LOOPS: usize = 3;

        let challenges = challenger
            .get_n_challenges(&mut self.api, NUM_LOOPS)
            .iter()
            .map(|x| Variable::from(*x))
            .collect_vec();

        // Loop 3 times to increase the security of the proof.
        // The security of each loop is log2(field_size) - log2(array_size).
        // Ex. For array size 2^14, and field size 2^64, each loop provides 50 bits of security.
        for i in 0..NUM_LOOPS {
            let sub_array_size = self.constant(L::Field::from_canonical_usize(SUB_ARRAY_SIZE));
            let end_idx = self.add(start_idx, sub_array_size);

            let false_v = self._false();
            let true_v = self._true();
            let mut within_sub_array = false_v;
            let one: Variable = self.one();

            let mut accumulator1 = self.zero::<Variable>();
            let mut subarray_size: Variable = self.zero();

            // r is the source of randomness from the challenger for this loop.
            let mut r = one;
            for j in 0..ARRAY_SIZE {
                let idx = self.constant::<Variable>(L::Field::from_canonical_usize(j));

                // If at the start_idx, then set within_sub_array to true.
                let at_start_idx = self.is_equal(idx, start_idx);
                within_sub_array = self.select(at_start_idx, true_v, within_sub_array);

                // If at the end_idx, then set within_sub_array to false.
                let at_end_idx = self.is_equal(idx, end_idx);
                within_sub_array = self.select(at_end_idx, false_v, within_sub_array);

                subarray_size = self.add(subarray_size, within_sub_array.variable);

                // If within the subarray, multiply the current r by the challenge.
                let multiplier = self.select(within_sub_array, challenges[i], one);
                // For subarray[i], the multiplier should be r^i. i is the index within the subarray.
                // The value of r outside of the subarray is not used.
                r = self.mul(r, multiplier);

                // Multiply the current r by the current array element.
                let temp_accum = self.mul(r, array[j]);
                // If outside of the subarray, don't add to the accumulator.
                let temp_accum = self.mul(within_sub_array.variable, temp_accum);

                accumulator1 = self.add(accumulator1, temp_accum);
            }

            // Assert that the returned subarray's length is == SUB_ARRAY_SIZE.
            let expected_subarray_size =
                self.constant(L::Field::from_canonical_usize(SUB_ARRAY_SIZE));
            self.assert_is_equal(subarray_size, expected_subarray_size);

            let mut accumulator2 = self.zero();
            let mut r = one;
            for j in 0..SUB_ARRAY_SIZE {
                r = self.mul(r, challenges[i]);
                let product = self.mul(r, sub_array[j]);
                accumulator2 = self.add(accumulator2, product);
            }

            self.assert_is_equal(accumulator1, accumulator2);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubArrayExtractorHint {
    array_size: usize,
    sub_array_size: usize,
}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for SubArrayExtractorHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let mut array_elements = Vec::new();

        for _i in 0..self.array_size {
            let element = input_stream.read_value::<ByteVariable>();
            array_elements.push(element);
        }

        let start_idx = input_stream.read_value::<Variable>().to_canonical_u64();
        let end_idx = start_idx + self.sub_array_size as u64;

        assert!(end_idx <= self.array_size as u64);

        for i in 0..self.sub_array_size {
            let element = array_elements[start_idx as usize + i];
            output_stream.write_value::<ByteVariable>(element);
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
        const ARRAY_SIZE: usize = 12800;
        const SUB_ARRAY_SIZE: usize = 3200;
        const START_IDX: usize = 400;

        let mut builder = DefaultBuilder::new();

        let array = builder.read::<ArrayVariable<ByteVariable, ARRAY_SIZE>>();
        let start_idx = builder.constant(F::from_canonical_usize(START_IDX));
        let seed = builder.read::<Bytes32Variable>();
        let result = builder.get_fixed_subarray::<ARRAY_SIZE, SUB_ARRAY_SIZE>(
            &array,
            start_idx,
            &seed.as_bytes(),
        );
        builder.write(result);

        let circuit = builder.build();

        // The last 20 elements are dummy
        let mut rng = OsRng;
        let mut array_input = [0u8; ARRAY_SIZE];
        for elem in array_input.iter_mut() {
            *elem = rng.gen();
        }

        let mut seed_input = [0u8; 15];
        for elem in seed_input.iter_mut() {
            *elem = rng.gen();
        }

        let mut input = circuit.input();
        input.write::<ArrayVariable<ByteVariable, ARRAY_SIZE>>(array_input.to_vec());
        input.write::<Bytes32Variable>(bytes32!(
            "0x7c38fc8356aa20394c7f538e3cee3f924e6d9252494c8138d1a6aabfc253118f"
        ));

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let expected_sub_array = array_input[START_IDX..START_IDX + SUB_ARRAY_SIZE].to_vec();
        assert_eq!(
            output.read::<ArrayVariable<ByteVariable, SUB_ARRAY_SIZE>>(),
            expected_sub_array
        );
    }

    #[test]
    #[should_panic]
    fn test_get_fixed_subarray_bad_case() {
        utils::setup_logger();
        type F = GoldilocksField;
        const ARRAY_SIZE: usize = 12800;
        const SUB_ARRAY_SIZE: usize = 3200;
        const START_IDX: usize = 12000;

        let mut builder = DefaultBuilder::new();

        let array = builder.read::<ArrayVariable<ByteVariable, ARRAY_SIZE>>();
        let start_idx = builder.constant(F::from_canonical_usize(START_IDX));
        let seed = builder.read::<Bytes32Variable>();
        let result = builder.get_fixed_subarray::<ARRAY_SIZE, SUB_ARRAY_SIZE>(
            &array,
            start_idx,
            &seed.as_bytes(),
        );
        builder.write(result);

        let circuit = builder.build();

        // The last 20 elements are dummy
        let mut rng = OsRng;
        let mut array_input = [0u8; ARRAY_SIZE];
        for elem in array_input.iter_mut() {
            *elem = rng.gen();
        }

        let mut seed_input = [0u8; 15];
        for elem in seed_input.iter_mut() {
            *elem = rng.gen();
        }

        let mut input = circuit.input();
        input.write::<ArrayVariable<ByteVariable, ARRAY_SIZE>>(array_input.to_vec());
        input.write::<Bytes32Variable>(bytes32!(
            "0x7c38fc8356aa20394c7f538e3cee3f924e6d9252494c8138d1a6aabfc253118f"
        ));

        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
