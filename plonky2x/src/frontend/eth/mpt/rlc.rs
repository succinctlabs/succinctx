use itertools::Itertools;
use plonky2::field::types::Field;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::challenger::RecursiveChallenger;

use crate::prelude::{BoolVariable, ByteVariable, CircuitBuilder, PlonkParameters, Variable};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Generates a commitment for a subarray using RLC.
    fn commit_subarray(
        &mut self,
        arr: &[ByteVariable],
        offset: Variable,
        len: Variable,
        random_value: Variable,
    ) -> Variable {
        let end_idx = self.add(offset, len);
        let mut commitment: Variable = self.zero();
        let mut is_within_subarray = self.zero();

        let one: Variable = self.one();
        let mut current_multiplier = one;
        for idx in 0..arr.len() {
            let idx_target = self.constant(L::Field::from_canonical_usize(idx));
            // is_within_subarray is one if idx is in the range [offset..offset+len].
            let is_at_start_idx = self.is_equal(idx_target, offset);
            is_within_subarray = self.add(is_within_subarray, is_at_start_idx.0);
            let is_at_end_idx = self.is_equal(idx_target, end_idx);
            is_within_subarray = self.sub(is_within_subarray, is_at_end_idx.0);

            let to_be_multiplied = self.select(BoolVariable(is_within_subarray), random_value, one);
            current_multiplier = self.mul(current_multiplier, to_be_multiplied);

            let le_value = arr[idx].to_variable(self);
            let multiplied_value = self.mul(le_value, current_multiplier);
            let random_value_if_in_range = self.mul(is_within_subarray, multiplied_value);
            commitment = self.add(commitment, random_value_if_in_range);
        }

        commitment
    }

    /// Checks subarrays for equality using a random linear combination.
    pub fn subarray_equal(
        &mut self,
        a: &[ByteVariable],
        a_offset: Variable,
        b: &[ByteVariable],
        b_offset: Variable,
        len: Variable,
    ) -> BoolVariable {
        let mut challenger = RecursiveChallenger::<L::Field, PoseidonHash, D>::new(&mut self.api);
        let challenger_seed = [a, b]
            .concat()
            .iter()
            .map(|byte| byte.to_variable(self).0)
            .collect_vec();

        challenger.observe_elements(&challenger_seed);

        let challenge = Variable(challenger.get_challenge(&mut self.api));

        let commitment_for_a = self.commit_subarray(a, a_offset, len, challenge);
        let commitment_for_b = self.commit_subarray(b, b_offset, len, challenge);
        self.is_equal(commitment_for_a, commitment_for_b)
    }

    /// Asserts that subarrays are equal using a random linear combination.
    pub fn assert_subarray_equal(
        &mut self,
        a: &[ByteVariable],
        a_offset: Variable,
        b: &[ByteVariable],
        b_offset: Variable,
        len: Variable,
    ) {
        let subarrays_are_equal = self.subarray_equal(a, a_offset, b, b_offset, len);
        self.assert_is_true(subarrays_are_equal);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::types::Field;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    use crate::frontend::builder::DefaultBuilder;
    use crate::prelude::{ByteVariable, Variable};

    impl Default for ByteVariable {
        fn default() -> ByteVariable {
            unsafe { std::mem::zeroed() }
        }
    }

    #[test]
    pub fn test_subarray_equal_should_succeed() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = DefaultBuilder::new();

        const MAX_LEN: usize = 15;
        let mut a: [ByteVariable; MAX_LEN] = Default::default();
        let mut b: [ByteVariable; MAX_LEN] = Default::default();

        for i in 0..MAX_LEN {
            a[i] = builder.constant::<ByteVariable>((i + 5) as u8);
        }

        for i in 0..MAX_LEN {
            b[i] = builder.constant::<ByteVariable>(i as u8);
        }

        let a_offset: Variable = builder.constant(F::ZERO);
        let b_offset = builder.constant(F::from_canonical_usize(5));
        let len: Variable = builder.constant(F::from_canonical_usize(5));
        builder.assert_subarray_equal(&a, a_offset, &b, b_offset, len);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let input = circuit.input();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output)
    }

    #[test]
    pub fn test_subarray_equal_diff_len_should_succeed() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = DefaultBuilder::new();

        const LEN_A: usize = 15;
        const LEN_B: usize = 10;
        let mut a: [ByteVariable; LEN_A] = Default::default();
        let mut b: [ByteVariable; LEN_B] = Default::default();

        for i in 0..LEN_A {
            a[i] = builder.constant::<ByteVariable>((i + 5) as u8);
        }

        for i in 0..LEN_B {
            b[i] = builder.constant::<ByteVariable>(i as u8);
        }

        let a_offset: Variable = builder.constant(F::ZERO);
        let b_offset = builder.constant(F::from_canonical_usize(5));
        let len: Variable = builder.constant(F::from_canonical_usize(5));
        builder.assert_subarray_equal(&a, a_offset, &b, b_offset, len);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let input = circuit.input();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output)
    }

    #[test]
    pub fn test_subarray_equal_diff_len_inverse_should_succeed() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = DefaultBuilder::new();

        const LEN_A: usize = 10;
        const LEN_B: usize = 15;
        let mut a: [ByteVariable; LEN_A] = Default::default();
        let mut b: [ByteVariable; LEN_B] = Default::default();

        for i in 0..LEN_A {
            a[i] = builder.constant::<ByteVariable>((i + 5) as u8);
        }

        for i in 0..LEN_B {
            b[i] = builder.constant::<ByteVariable>(i as u8);
        }

        let a_offset: Variable = builder.constant(F::ZERO);
        let b_offset = builder.constant(F::from_canonical_usize(5));
        let len: Variable = builder.constant(F::from_canonical_usize(5));
        builder.assert_subarray_equal(&a, a_offset, &b, b_offset, len);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let input = circuit.input();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output)
    }

    #[test]
    #[should_panic]
    pub fn test_subarray_equal_should_fail() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = DefaultBuilder::new();

        const MAX_LEN: usize = 15;
        let mut a: [ByteVariable; MAX_LEN] = Default::default();
        let mut b: [ByteVariable; MAX_LEN] = Default::default();

        for i in 0..MAX_LEN {
            a[i] = builder.constant::<ByteVariable>((i + 5) as u8);
        }

        for i in 0..MAX_LEN {
            b[i] = builder.constant::<ByteVariable>(i as u8);
        }

        // Modify 1 byte here.
        b[6] = builder.constant::<ByteVariable>(0);

        let a_offset = builder.constant(F::ZERO);
        let b_offset = builder.constant(F::from_canonical_usize(5));
        let len: Variable = builder.constant(F::from_canonical_usize(5));
        builder.assert_subarray_equal(&a, a_offset, &b, b_offset, len);

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let input = circuit.input();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output)
    }
}
