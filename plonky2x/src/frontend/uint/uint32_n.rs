use std::fmt::Debug;

use array_macro::array;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{CircuitVariable, EvmVariable, U32Variable, Variable};
use crate::prelude::*;

pub trait Uint<const N: usize>: Debug + Clone + Copy + Sync + Send + 'static {
    fn to_little_endian(&self, bytes: &mut [u8]);

    fn from_little_endian(slice: &[u8]) -> Self;

    fn to_big_endian(&self, bytes: &mut [u8]);

    fn from_big_endian(slice: &[u8]) -> Self;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    fn to_u32_limbs(self) -> [u32; N] {
        let mut bytes = vec![0u8; N * 4];
        self.to_little_endian(&mut bytes);
        let mut ret: [u32; N] = [0; N];
        for i in 0..N {
            let byte_offset = i * 4;
            ret[i] = u32::from_le_bytes([
                bytes[byte_offset],
                bytes[byte_offset + 1],
                bytes[byte_offset + 2],
                bytes[byte_offset + 3],
            ])
        }
        ret
    }

    fn from_u32_limbs(limbs: [u32; N]) -> Self {
        let bytes = limbs
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<_>>();
        Self::from_little_endian(&bytes)
    }
}

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U32NVariable<U: Uint<N>, const N: usize> {
    pub limbs: [U32Variable; N],
    _marker: std::marker::PhantomData<U>,
}

impl<U: Uint<N>, const N: usize> CircuitVariable for U32NVariable<U, N> {
    type ValueType<F: RichField> = U;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            limbs: array![_ => U32Variable::init_unsafe(builder); N],
            _marker: core::marker::PhantomData,
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        let limbs = U::to_u32_limbs(value);
        Self {
            limbs: array![i => U32Variable::constant(builder, limbs[i]); N],
            _marker: core::marker::PhantomData,
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.limbs.iter().map(|x| x.0).collect()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), N);
        Self {
            limbs: array![i => U32Variable(variables[i]); N],
            _marker: core::marker::PhantomData,
        }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        for limb in self.limbs.iter() {
            limb.assert_is_valid(builder);
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let mut value_limbs: [u32; N] = [0; N];
        for i in 0..N {
            value_limbs[i] = self.limbs[i].get(witness);
        }

        U::from_u32_limbs(value_limbs)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: U) {
        let limbs = U::to_u32_limbs(value);
        for i in 0..N {
            self.limbs[i].set(witness, limbs[i]);
        }
    }
}

impl<U: Uint<N>, const N: usize> EvmVariable for U32NVariable<U, N> {
    fn encode<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<ByteVariable> {
        self.limbs
            .iter()
            .rev()
            .flat_map(|x| x.encode(builder))
            .collect::<Vec<_>>()
    }

    fn decode<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        assert_eq!(bytes.len(), N * 4);
        let mut limbs = [U32Variable::init_unsafe(builder); N];
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
        let mut bytes = vec![0u8; N * 4];
        U::to_big_endian(&value, &mut bytes);
        bytes
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        U::from_big_endian(bytes)
    }
}

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> Zero<L, D>
    for U32NVariable<U, N>
{
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
        let zero = U32Variable::zero(builder);
        Self {
            limbs: [zero; N],
            _marker: core::marker::PhantomData,
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> One<L, D>
    for U32NVariable<U, N>
{
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
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

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> Add<L, D>
    for U32NVariable<U, N>
{
    type Output = Self;

    fn add(self, rhs: U32NVariable<U, N>, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
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
        assert_eq!(self_targets.len(), rhs_targets.len());
        assert_eq!(self_targets.len(), N);

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

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> Sub<L, D>
    for U32NVariable<U, N>
{
    type Output = Self;

    fn sub(self, rhs: U32NVariable<U, N>, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
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
        assert_eq!(self_targets.len(), rhs_targets.len());
        assert_eq!(self_targets.len(), N);

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

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> Mul<L, D>
    for U32NVariable<U, N>
{
    type Output = Self;

    fn mul(self, rhs: U32NVariable<U, N>, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
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
        assert_eq!(self_targets.len(), rhs_targets.len());
        assert_eq!(self_targets.len(), N);

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

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> Div<L, D>
    for U32NVariable<U, N>
{
    type Output = Self;

    fn div(self, rhs: U32NVariable<U, N>, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
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
        assert_eq!(self_targets.len(), rhs_targets.len());
        assert_eq!(self_targets.len(), N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let rhs_biguint = BigUintTarget { limbs: rhs_targets };

        let product_biguint = builder.api.div_biguint(&self_biguint, &rhs_biguint);

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

impl<L: PlonkParameters<D>, const D: usize, U: Uint<N>, const N: usize> Rem<L, D>
    for U32NVariable<U, N>
{
    type Output = Self;

    fn rem(self, rhs: U32NVariable<U, N>, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
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
        assert_eq!(self_targets.len(), rhs_targets.len());
        assert_eq!(self_targets.len(), N);

        let self_biguint = BigUintTarget {
            limbs: self_targets,
        };
        let rhs_biguint = BigUintTarget { limbs: rhs_targets };

        let product_biguint = builder.api.rem_biguint(&self_biguint, &rhs_biguint);

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

#[cfg(test)]
mod tests {
    use ethers::types::{U128, U256, U64};
    use rand::rngs::OsRng;
    use rand::Rng;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::uint::uint32_n::{U32NVariable, Uint};
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    fn test_u32n_evm<U: Uint<N>, const N: usize>() {
        let num_bytes = N * 4;
        let mut builder = CircuitBuilder::<L, D>::new();
        let mut var_bytes = vec![];
        for i in 0..(num_bytes) {
            let byte = ByteVariable::constant(&mut builder, i as u8);
            var_bytes.push(byte);
        }
        let decoded: U32NVariable<U, N> = U32NVariable::decode(&mut builder, &var_bytes);
        let encoded = decoded.encode(&mut builder);
        let redecoded = U32NVariable::decode(&mut builder, &encoded[0..num_bytes]);

        builder.assert_is_equal(decoded, redecoded);
        for i in 0..(num_bytes) {
            builder.assert_is_equal(var_bytes[i], encoded[i]);
        }

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_uint_evm() {
        test_u32n_evm::<U64, 2>();
        test_u32n_evm::<U128, 4>();
        test_u32n_evm::<U256, 8>();
    }

    fn test_u32n_evm_value<U: Uint<N>, const N: usize>() {
        type F = GoldilocksField;

        let limbs = [OsRng.gen::<u32>(); N];
        let num = U::from_u32_limbs(limbs);
        let encoded = U32NVariable::encode_value::<F>(num);
        let decoded: U = U32NVariable::decode_value::<F>(&encoded);

        assert_eq!(decoded.to_u32_limbs(), num.to_u32_limbs());
    }

    #[test]
    fn test_uint_evm_value() {
        test_u32n_evm_value::<U64, 2>();
        test_u32n_evm_value::<U128, 4>();
        test_u32n_evm_value::<U256, 8>();
    }

    fn test_u32n_add<U: Uint<N>, const N: usize>() {
        let mut rng = OsRng;

        let a = U::from_u32_limbs([rng.gen(); N]);
        let b = U::from_u32_limbs([rng.gen(); N]);

        let (expected_value, _) = a.overflowing_add(b);

        let mut builder = CircuitBuilder::<L, D>::new();

        let a = U32NVariable::constant(&mut builder, a);
        let b = U32NVariable::constant(&mut builder, b);
        let result = builder.add(a, b);
        let expected_result_var = U32NVariable::constant(&mut builder, expected_value);

        builder.assert_is_equal(result, expected_result_var);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_uint_add() {
        test_u32n_add::<U64, 2>();
        test_u32n_add::<U128, 4>();
        test_u32n_add::<U256, 8>();
    }

    fn test_u256_sub<U: Uint<N>, const N: usize>() {
        let _num_bytes = N * 4;

        let mut rng = OsRng;

        let a = U::from_u32_limbs([rng.gen(); N]);
        let b = U::from_u32_limbs([rng.gen(); N]);

        let (expected_value, _) = a.overflowing_sub(b);

        let mut builder = CircuitBuilder::<L, D>::new();

        let a = U32NVariable::constant(&mut builder, a);
        let b = U32NVariable::constant(&mut builder, b);
        let result = builder.sub(a, b);
        let expected_result_var = U32NVariable::constant(&mut builder, expected_value);

        builder.assert_is_equal(result, expected_result_var);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_uint_sub() {
        test_u256_sub::<U64, 2>();
        test_u256_sub::<U128, 4>();
        test_u256_sub::<U256, 8>();
    }

    fn test_u256_mul<U: Uint<N>, const N: usize>() {
        const D: usize = 2;

        let mut rng = OsRng;

        let a = U::from_u32_limbs([rng.gen(); N]);
        let b = U::from_u32_limbs([rng.gen(); N]);

        let (expected_value, _) = a.overflowing_mul(b);

        let mut builder = CircuitBuilder::<L, D>::new();

        let a = U32NVariable::constant(&mut builder, a);
        let b = U32NVariable::constant(&mut builder, b);
        let result = builder.mul(a, b);
        let expected_result_var = U32NVariable::constant(&mut builder, expected_value);

        builder.assert_is_equal(result, expected_result_var);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_uint_mul() {
        test_u256_mul::<U64, 2>();
        test_u256_mul::<U128, 4>();
        test_u256_mul::<U256, 8>();
    }
}
