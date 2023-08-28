use std::fmt::Debug;

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::AlgebraicVariable;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{CircuitVariable, EvmVariable, U32Variable, Variable};
use crate::prelude::ByteVariable;

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

impl<VT: ValueTrait + Debug + Clone + Copy + Sync + Send + 'static, const N: usize>
    AlgebraicVariable for U32NVariable<VT, N>
where
    [(); N * 4]:,
{
    /// Returns limbs of all zeros
    fn zero<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        let zero = U32Variable::zero(builder);
        Self {
            limbs: [zero; N],
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the one value of the variable.
    fn one<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        let zero = U32Variable::zero(builder);
        let one = U32Variable::one(builder);

        let mut limbs = [zero; N];
        limbs[0] = one;
        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }

    // Adds two variables together.
    fn add<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        let self_targets = self
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        let other_targets = other
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        assert!(self_targets.len() == other_targets.len());
        assert!(self_targets.len() == N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let other_biguint = BigUintTarget {
            limbs: other_targets,
        };

        let sum_biguint = builder.api.add_biguint(&self_biguint, &other_biguint);

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

    // Subtracts two variables.
    fn sub<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        let self_targets = self
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        let other_targets = other
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        assert!(self_targets.len() == other_targets.len());
        assert!(self_targets.len() == N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let other_biguint = BigUintTarget {
            limbs: other_targets,
        };

        let diff_biguint = builder.api.sub_biguint(&self_biguint, &other_biguint);

        let mut limbs: [U32Variable; N] = Self::zero(builder).limbs;
        for i in 0..N {
            limbs[i] = U32Variable(Variable(diff_biguint.limbs[i].0));
        }

        Self {
            limbs,
            _marker: core::marker::PhantomData,
        }
    }

    // Multiplies two variables.
    fn mul<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        let self_targets = self
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        let other_targets = other
            .limbs
            .iter()
            .map(|x| U32Target(x.0 .0))
            .collect::<Vec<_>>();
        assert!(self_targets.len() == other_targets.len());
        assert!(self_targets.len() == N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let other_biguint = BigUintTarget {
            limbs: other_targets,
        };

        let product_biguint = builder.api.mul_biguint(&self_biguint, &other_biguint);

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

    // Negates a variable.
    fn neg<F: RichField + Extendable<D>, const D: usize>(
        &self,
        _builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        todo!()
    }
}
