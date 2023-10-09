use core::marker::PhantomData;

use curta::chip::field::parameters::FieldParameters;
use curta::chip::utils::digits_to_biguint;
use curta::math::prelude::PrimeField64;
use curta::polynomial::to_u16_le_limbs_polynomial;
use num_bigint::BigUint;

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
        _builder: &mut CircuitBuilder<L, D>,
    ) {
        todo!("range checks on field variable")
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
