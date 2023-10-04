use core::marker::PhantomData;

use curta::chip::ec::weierstrass::bn254::Bn254;
use curta::chip::field::parameters::FieldParameters;
use curta::chip::utils::digits_to_biguint;
use curta::math::prelude::PrimeField64;
use curta::polynomial::to_u16_le_limbs_polynomial;
use ethers::types::U256;
use num_bigint::BigUint;

use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::uint::uint32::U32Variable;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FieldVariable<P> {
    pub limbs: Vec<Variable>,
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

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        let limb_values = to_u16_le_limbs_polynomial::<L::Field, P>(&value).as_coefficients();

        let limbs = limb_values
            .into_iter()
            .map(|limb| builder.constant(limb))
            .collect();
        Self::new(limbs)
    }

    fn variables(&self) -> Vec<Variable> {
        self.limbs.clone()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self::new(variables.to_vec())
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let limbs = self
            .limbs
            .iter()
            .map(|v| v.get(witness).as_canonical_u64() as u16)
            .collect::<Vec<_>>();
        digits_to_biguint(&limbs)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        let limb_values = to_u16_le_limbs_polynomial::<F, P>(&value).as_coefficients();

        for (limb, value) in self.limbs.iter().zip(limb_values) {
            limb.set(witness, value);
        }
    }

    // fn elements<L: PlonkParameters<D>, const D: usize>(
    //     value: Self::ValueType<L::Field>,
    // ) -> Vec<L::Field> {
    //     to_u16_le_limbs_polynomial::<L::Field, P>(&value).as_coefficients()
    // }

    // fn from_elements<L: PlonkParameters<D>, const D: usize>(
    //     elements: &[L::Field],
    // ) -> Self::ValueType<L::Field> {
    //     field_limbs_to_biguint(elements)
    // }
}

impl FieldVariable<Bn254> {
    pub fn as_u32_limbs<L: PlonkParameters<D>, const D: usize>(&self) -> Vec<U32Variable> {
        self.limbs.iter().map(|v| U32Variable(*v)).collect()
    }

    pub fn to_u256<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> U256Variable {
        assert_eq!(self.limbs.len(), 16);
        let limbs_u32 = self.as_u32_limbs::<L, D>();
        let mut sum = builder.zero::<U256Variable>();
        for i in 0..limbs_u32.len() {
            let limb_u256 = limbs_u32[i].to_u256(builder);
            let digit = builder.constant::<U256Variable>(U256::from(2u64).pow((i * 16).into()));
            let powered_term = builder.mul(limb_u256, digit);
            sum = builder.add(sum, powered_term);
        }
        sum
    }
}
