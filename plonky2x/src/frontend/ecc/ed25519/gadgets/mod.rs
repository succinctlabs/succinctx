pub mod curve;
pub mod eddsa;
use core::marker::PhantomData;

use plonky2::field::types::{Field, PrimeField};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};
use plonky2::util::ceil_div_usize;
use serde::Deserialize;
use serde_with::serde_as;

use self::curve::AffinePointTarget;
use super::curve::curve_types::{AffinePoint, Curve};
use crate::frontend::num::biguint::CircuitBuilderBiguint;
use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::prelude::{
    ArrayVariable, BytesVariable, CircuitBuilder, CircuitVariable, PlonkParameters, Variable,
};

// impl<C: Curve> CircuitVariable for AffinePointTarget<C> {
//     type ValueType<F: RichField> = AffinePoint<C>;

//     fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> Self {
//         let x = builder.init::<NonNativeTarget<C::BaseField>>();
//         let y = builder.init::<NonNativeTarget<C::BaseField>>();

//         AffinePointTarget { x, y }
//     }

//     fn constant<L: PlonkParameters<D>, const D: usize>(
//         builder: &mut CircuitBuilder<L, D>,
//         value: Self::ValueType<L::Field>,
//     ) -> Self {
//     }

//     fn variables(&self) -> Vec<Variable> {}

//     fn from_variables<L: PlonkParameters<D>, const D: usize>(
//         builder: &mut CircuitBuilder<L, D>,
//         variables: &[Variable],
//     ) -> Self {
//         let variable = Self::from_variables_unsafe(variables);
//         variable.assert_is_valid(builder);
//         variable
//     }

//     fn from_variables_unsafe(variables: &[Variable]) -> Self {}

//     fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
//         &self,
//         builder: &mut CircuitBuilder<L, D>,
//     ) {
//     }

//     fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
//         self.0.get(witness)
//     }

//     fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
//         self.0.set(witness, value)
//     }
// }

impl<FF: PrimeField> CircuitVariable for NonNativeTarget<FF> {
    type ValueType<F: RichField> = FF;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        builder.api.add_virtual_nonnative_target()
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        builder.api.constant_nonnative::<FF>(value)
    }

    fn variables(&self) -> Vec<Variable> {
        vec![self.0]
    }

    fn from_variables<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        variables: &[Variable],
    ) -> Self {
        let variable = Self::from_variables_unsafe(variables);
        variable.assert_is_valid(builder);
        variable
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {}

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.0.get(witness)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value)
    }
}
