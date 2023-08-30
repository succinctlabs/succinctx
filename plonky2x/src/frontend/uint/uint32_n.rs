use std::fmt::Debug;

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{CircuitVariable, EvmVariable, U32Variable, Variable};
use crate::prelude::*;

pub trait ValueTrait {
    fn to_limbs<const N: usize>(self) -> [u32; N];

    fn to_value<const N: usize>(limbs: [u32; N]) -> Self
    where
        [(); N * 4]:;

    fn to_big_endian(&self, bytes: &mut [u8]);

    fn from_big_endian(slice: &[u8]) -> Self;
}

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U32NVariable<
    VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static,
    const N: usize,
> {
    pub limbs: [U32Variable; N],
    _marker: std::marker::PhantomData<VT>,
}

impl<VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static, const N: usize> CircuitVariable
    for U32NVariable<VT, N>
where
    [(); N * 4]:,
{
    type ValueType<F: RichField> = VT;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            limbs: array![_ => U32Variable::init(builder); N],
            _marker: core::marker::PhantomData,
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        let limbs = VT::to_limbs::<N>(value);
        Self {
            limbs: array![i => U32Variable::constant(builder, limbs[i]); N],
            _marker: core::marker::PhantomData,
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.limbs.iter().map(|x| x.0).collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), N);
        Self {
            limbs: array![i => U32Variable(variables[i]); N],
            _marker: core::marker::PhantomData,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let mut value_limbs: [u32; N] = [0; N];
        for i in 0..N {
            value_limbs[i] = self.limbs[i].get(witness);
        }

        VT::to_value(value_limbs)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: VT) {
        let limbs = VT::to_limbs::<N>(value);
        for i in 0..N {
            self.limbs[i].set(witness, limbs[i]);
        }
    }
}

impl<VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static, const N: usize> EvmVariable
    for U32NVariable<VT, N>
where
    [(); N * 4]:,
{
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        self.limbs
            .iter()
            .rev()
            .flat_map(|x| x.encode(builder))
            .collect::<Vec<_>>()
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        assert_eq!(bytes.len(), N * 4);
        let mut limbs = [U32Variable::init(builder); N];
        for i in 0..N {
            limbs[i] = U32Variable::decode(builder, &bytes[i * 4..(i + 1) * 4]);
        }
        limbs.reverse();
        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        let mut bytes = [0u8; N * 4];
        VT::to_big_endian(&value, &mut bytes);
        bytes.to_vec()
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        VT::from_big_endian(bytes)
    }
}

impl<
        F: RichField + Extendable<D>,
        const D: usize,
        VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static,
        const N: usize,
    > Zero<F, D> for U32NVariable<VT, N>
{
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self {
        let zero = U32Variable::zero(builder);
        Self {
            limbs: [zero; N],
            _marker: core::marker::PhantomData,
        }
    }
}

impl<
        F: RichField + Extendable<D>,
        const D: usize,
        VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static,
        const N: usize,
    > One<F, D> for U32NVariable<VT, N>
{
    fn one(builder: &mut CircuitBuilder<F, D>) -> Self {
        let zero = U32Variable::zero(builder);
        let one = U32Variable::one(builder);

        let mut limbs = [zero; N];
        limbs[0] = one;
        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<
        F: RichField + Extendable<D>,
        const D: usize,
        VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static,
        const N: usize,
    > Mul<F, D> for U32NVariable<VT, N>
{
    type Output = Self;

    fn mul(self, rhs: U32NVariable<VT, N>, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_targets = self
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        let rhs_targets = rhs
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        assert!(self_targets.len() == rhs_targets.len());
        assert!(self_targets.len() == N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let rhs_biguint = BigUintTarget { limbs: rhs_targets };

        let product_biguint = builder.api.mul_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant limb
        let mut limbs: [U32Variable; N] = Self::zero(builder).limbs;
        for i in 0..N {
            limbs[i] = U32Variable(Variable(product_biguint.limbs[i].0));
        }

        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<
        F: RichField + Extendable<D>,
        const D: usize,
        VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static,
        const N: usize,
    > Add<F, D> for U32NVariable<VT, N>
{
    type Output = Self;

    fn add(self, rhs: U32NVariable<VT, N>, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_targets = self
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        let rhs_targets = rhs
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        assert!(self_targets.len() == rhs_targets.len());
        assert!(self_targets.len() == N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let rhs_biguint = BigUintTarget { limbs: rhs_targets };

        let sum_biguint = builder.api.add_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant limbs
        let mut limbs: [U32Variable; N] = Self::zero(builder).limbs;
        for i in 0..N {
            limbs[i] = U32Variable(Variable(sum_biguint.limbs[i].0));
        }

        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<
        F: RichField + Extendable<D>,
        const D: usize,
        VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static,
        const N: usize,
    > Sub<F, D> for U32NVariable<VT, N>
{
    type Output = Self;

    fn sub(self, rhs: U32NVariable<VT, N>, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_targets = self
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        let rhs_targets = rhs
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        assert!(self_targets.len() == rhs_targets.len());
        assert!(self_targets.len() == N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let rhs_biguint = BigUintTarget { limbs: rhs_targets };

        let diff_biguint = builder.api.sub_biguint(&self_biguint, &rhs_biguint);

        let mut limbs: [U32Variable; N] = Self::zero(builder).limbs;
        for i in 0..N {
            limbs[i] = U32Variable(Variable(diff_biguint.limbs[i].0));
        }

        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }
}
