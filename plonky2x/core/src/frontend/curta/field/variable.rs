use core::marker::PhantomData;

use curta::chip::field::parameters::FieldParameters;
use curta::chip::utils::digits_to_biguint;
use curta::math::prelude::PrimeField64;
use curta::polynomial::to_u16_le_limbs_polynomial;
use itertools::Itertools;
use num::One;
use num_bigint::BigUint;

use crate::frontend::uint::num::u32::gadgets::multiple_comparison::list_lte_circuit;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FieldVariable<P> {
    limbs: Vec<Variable>,
    _marker: PhantomData<P>,
}

impl<P> FieldVariable<P> {
    pub fn new(limbs: Vec<Variable>) -> Self {
        Self {
            limbs,
            _marker: PhantomData,
        }
    }
}

impl<P: FieldParameters> CircuitVariable for FieldVariable<P> {
    type ValueType<F: RichField> = BigUint;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self::new((0..P::NB_LIMBS).map(|_| builder.init()).collect())
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        // Only support 16-bit limbs.
        assert!(P::NB_BITS_PER_LIMB == 16);

        // Check that each limb is within u16.
        for limb in self.limbs.iter() {
            builder.api.range_check(limb.0, 16);
        }

        // Check that the value is less than the modulus.
        let bytes = (P::modulus() - BigUint::one()).to_bytes_le();

        // Initialize a vector to store the 16-bit limbs.
        let mut modulus_limbs = Vec::new();

        // Iterate over the bytes in 2-byte (16-bit) chunks.
        for i in (0..bytes.len()).step_by(2) {
            // Combine two bytes into a 16-bit limb
            let limb = (bytes[i] as u16) | ((bytes[i + 1] as u16) << 8);
            modulus_limbs.push(Variable::constant(
                builder,
                L::Field::from_canonical_u16(limb),
            ));
        }

        // "list_lte_circuit" expects that both the operands have the same number of limbs.
        // Pad the value limbs and modulus limbs with zeros to make them the same length.
        let mut padded_value_limbs = self.limbs.clone();
        for _ in padded_value_limbs.len()..P::NB_LIMBS {
            padded_value_limbs.push(builder.zero());
        }

        for _ in modulus_limbs.len()..P::NB_LIMBS {
            modulus_limbs.push(builder.zero());
        }

        let cmp = list_lte_circuit(
            &mut builder.api,
            padded_value_limbs.iter().map(|x| x.0).collect_vec(),
            modulus_limbs.iter().map(|x| x.0).collect_vec(),
            P::NB_BITS_PER_LIMB,
        );

        let true_val = builder._true();
        builder.assert_is_equal(cmp.into(), true_val);
    }

    fn nb_elements() -> usize {
        P::NB_LIMBS
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        to_u16_le_limbs_polynomial::<F, P>(&value).as_coefficients()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        let limbs = elements
            .iter()
            .map(|v| v.as_canonical_u64() as u16)
            .collect::<Vec<_>>();
        digits_to_biguint(&limbs)
    }

    fn variables(&self) -> Vec<Variable> {
        self.limbs.clone()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self::new(variables.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use curta::chip::ec::weierstrass::bn254::Bn254BaseField;
    use curta::chip::field::parameters::FieldParameters;
    use num::{One, Zero};
    use num_bigint::BigUint;

    use super::FieldVariable;
    use crate::prelude::{CircuitBuilder, CircuitVariable, DefaultParameters};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_assert_is_valid() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let field_var = builder.read::<FieldVariable<Bn254BaseField>>();
        field_var.assert_is_valid(&mut builder);

        let circuit = builder.build();

        let test_cases = [BigUint::zero(), Bn254BaseField::modulus() - BigUint::one()];

        for test_case in test_cases.iter() {
            let mut inputs = circuit.input();
            inputs.write::<FieldVariable<Bn254BaseField>>(test_case.clone());

            let (proof, output) = circuit.prove(&inputs);
            circuit.verify(&proof, &inputs, &output);
        }
    }

    #[test]
    #[should_panic]
    fn test_assert_is_not_valid() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let field_var = builder.read::<FieldVariable<Bn254BaseField>>();
        field_var.assert_is_valid(&mut builder);

        let circuit = builder.build();

        let value = Bn254BaseField::modulus();
        let mut inputs = circuit.input();
        inputs.write::<FieldVariable<Bn254BaseField>>(value);

        let (proof, output) = circuit.prove(&inputs);
        circuit.verify(&proof, &inputs, &output);
    }
}
