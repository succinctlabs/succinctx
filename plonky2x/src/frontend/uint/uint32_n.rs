use std::fmt::Debug;

use array_macro::array;
use ethers::types::U256;
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

impl ValueTrait for U256 {
    fn to_limbs<const N: usize>(self) -> [u32; N] {
        let mut bytes = [0u8; 32];
        self.to_little_endian(&mut bytes);
        let mut ret: [u32; N] = [0; N];
        for i in 0..=N {
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

    fn to_value<const N: usize>(limbs: [u32; N]) -> Self
    where
        [(); N * 4]:,
    {
        let mut bytes = [0u8; N * 4];
        for (i, &limb) in limbs.iter().enumerate() {
            bytes[i * 4..(i + 1) * 4].copy_from_slice(&limb.to_le_bytes());
        }
        Self::from_little_endian(&bytes)
    }

    fn to_big_endian(&self, bytes: &mut [u8]) {
        self.to_big_endian(bytes);
    }

    fn from_big_endian(bytes: &[u8]) -> Self {
        Self::from_big_endian(bytes)
    }
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

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::U32Variable;
    use crate::frontend::uint::AlgebraicVariable;
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    #[test]
    fn test_u32_evm() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let var = U32Variable::constant(&mut builder, 0x12345678);

        let encoded = var.encode(&mut builder);

        let bytes = [0x12, 0x34, 0x56, 0x78];

        for (i, byte) in encoded.iter().enumerate() {
            let expected = ByteVariable::constant(&mut builder, bytes[i]).0;
            byte.0.iter().enumerate().for_each(|(j, &bit)| {
                builder.assert_is_equal(bit.0, expected[j].0);
            });
        }

        let decoded = U32Variable::decode(&mut builder, &encoded[0..4]);
        builder.assert_is_equal(decoded.0, var.0);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_evm_value() {
        type F = GoldilocksField;

        let val = 0x12345678_u32;
        let encoded = U32Variable::encode_value::<F>(val);
        let decoded = U32Variable::decode_value::<F>(&encoded);
        assert_eq!(encoded[0], 0x12);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x56);
        assert_eq!(encoded[3], 0x78);
        assert_eq!(decoded, 0x12345678);
    }

    #[test]
    fn test_u32_add() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u32 = rng.gen();
        let operand_b: u32 = rng.gen();
        // Perform addition without overflow panic
        let expected_result = operand_a.wrapping_add(operand_b);

        let a = U32Variable::constant(&mut builder, operand_a);
        let b = U32Variable::constant(&mut builder, operand_b);
        let result = a.add(&mut builder, &b);
        let expected_result_var = U32Variable::constant(&mut builder, expected_result);

        builder.assert_is_equal(result.0, expected_result_var.0);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_sub() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u32 = rng.gen();
        let operand_b: u32 = rng.gen();
        let expected_result = operand_a.wrapping_sub(operand_b);

        let a = U32Variable::constant(&mut builder, operand_a);
        let b = U32Variable::constant(&mut builder, operand_b);
        let result = a.sub(&mut builder, &b);
        let expected_result_var = U32Variable::constant(&mut builder, expected_result);

        builder.assert_is_equal(result.0, expected_result_var.0);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_mul() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u32 = rng.gen();
        let operand_b: u32 = rng.gen();
        let expected_result = operand_a.wrapping_mul(operand_b);

        let a = U32Variable::constant(&mut builder, operand_a);
        let b = U32Variable::constant(&mut builder, operand_b);
        let result = a.mul(&mut builder, &b);
        let expected_result_var = U32Variable::constant(&mut builder, expected_result);

        builder.assert_is_equal(result.0, expected_result_var.0);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
