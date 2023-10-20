use std::fmt::Debug;

use curta::chip::bool;
use itertools::Itertools;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;

use super::uint64::U64Variable;
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::frontend::num::u32::gadgets::multiple_comparison::list_lte_circuit;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::*;

/// A variable in the circuit representing a u32 value.
///
/// Under the hood, it is represented as a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U32Variable(pub Variable);

impl CircuitVariable for U32Variable {
    type ValueType<F: RichField> = u32;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self(Variable::init_unsafe(builder))
    }

    fn variables(&self) -> Vec<Variable> {
        vec![self.0]
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self(variables[0])
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        let bits = builder.api.u32_to_bits_le(U32Target(self.targets()[0]));
        let reconstructed_val = builder.api.le_sum(bits.iter());
        builder.assert_is_equal(self.0, Variable(reconstructed_val))
    }

    fn nb_elements() -> usize {
        1
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        vec![F::from_canonical_u32(value)]
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        let v = Variable::from_elements(&[elements[0]]);
        v.to_canonical_u64() as u32
    }
}

impl EvmVariable for U32Variable {
    fn encode<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<ByteVariable> {
        let mut bits = builder.api.split_le(self.0 .0, 32);
        bits.reverse();
        bits.chunks(8)
            .map(|chunk| {
                let targets = chunk.iter().map(|b| b.target).collect_vec();
                ByteVariable::from_targets(&targets)
            })
            .collect()
    }

    fn decode<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        assert_eq!(bytes.len(), 4);

        let mut bits = bytes.iter().flat_map(|byte| byte.targets()).collect_vec();
        bits.reverse();

        let target = builder
            .api
            .le_sum(bits.into_iter().map(BoolTarget::new_unsafe));
        Self(Variable(target))
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        let mut bytes = vec![0_u8; 4];
        for i in 0..4 {
            bytes[i] = ((value >> ((4 - i - 1) * 8)) & 0xff) as u8;
        }
        bytes
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        assert_eq!(bytes.len(), 4);
        let mut value = 0_u32;
        for i in 0..4 {
            value |= (bytes[i] as u32) << ((4 - i - 1) * 8);
        }
        value
    }
}

impl<L: PlonkParameters<D>, const D: usize> LessThanOrEqual<L, D> for U32Variable {
    fn lte(self, rhs: Self, builder: &mut CircuitBuilder<L, D>) -> BoolVariable {
        BoolVariable(Variable(
            list_lte_circuit(&mut builder.api, self.targets(), rhs.targets(), 32).target,
        ))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Zero<L, D> for U32Variable {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
        let zero = Variable::zero(builder);
        Self(zero)
    }
}

impl<L: PlonkParameters<D>, const D: usize> One<L, D> for U32Variable {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
        let one = Variable::one(builder);
        Self(one)
    }
}

impl<L: PlonkParameters<D>, const D: usize> Mul<L, D> for U32Variable {
    type Output = Self;

    fn mul(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_target = self.0 .0;
        let rhs_target = rhs.0 .0;
        let self_biguint = BigUintTarget {
            limbs: vec![U32Target(self_target)],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![U32Target(rhs_target)],
        };

        let product_biguint = builder.api.mul_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant limb
        let product = product_biguint.limbs[0].0;
        Self(Variable(product))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Add<L, D> for U32Variable {
    type Output = Self;

    fn add(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_target = self.0 .0;
        let rhs_target = rhs.0 .0;
        let self_biguint = BigUintTarget {
            limbs: vec![U32Target(self_target)],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![U32Target(rhs_target)],
        };

        let sum_biguint = builder.api.add_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant limb
        let sum = sum_biguint.limbs[0].0;

        Self(Variable(sum))
    }
}

impl<L: PlonkParameters<D>, const D: usize> Sub<L, D> for U32Variable {
    type Output = Self;

    fn sub(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_target = self.0 .0;
        let rhs_target = rhs.0 .0;
        let self_biguint = BigUintTarget {
            limbs: vec![U32Target(self_target)],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![U32Target(rhs_target)],
        };

        let diff_biguint = builder.api.sub_biguint(&self_biguint, &rhs_biguint);
        let diff = diff_biguint.limbs[0].0;
        Self(Variable(diff))
    }
}

impl U32Variable {
    pub fn to_u64<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> U64Variable {
        let zero = builder.zero::<U32Variable>();
        let result = builder.init::<U64Variable>();
        for i in 0..result.limbs.len() {
            if i == 0 {
                builder.connect(*self, result.limbs[i]);
            } else {
                builder.connect(zero, result.limbs[i]);
            }
        }
        result
    }

    pub fn from_be_bits<L: PlonkParameters<D>, const D: usize>(
        bools: &[BoolVariable],
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        assert!(bools.len() <= 32);
        // We simply sum the bits and don't need to do a range-check, as we know that the sum of 32 bits is always less than 2^32
        let var = builder.api.le_sum(
            bools
                .iter()
                .map(|b| (*b).into())
                .collect::<Vec<BoolTarget>>()
                .into_iter(),
        );
        Self(Variable(var))
    }

    pub fn to_be_bits<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> [BoolVariable; 32] {
        let mut bits = builder.api.split_le(self.0 .0, 32);
        bits.reverse();
        bits.iter()
            .map(|b| BoolVariable(Variable(b.target)))
            .collect_vec()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::U32Variable;
    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_u32_evm() {
        let mut builder = CircuitBuilder::<L, D>::new();

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

        let circuit = builder.build();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_evm_value() {
        let val = 0x12345678_u32;
        let encoded = U32Variable::encode_value::<GoldilocksField>(val);
        let decoded = U32Variable::decode_value::<GoldilocksField>(&encoded);
        assert_eq!(encoded[0], 0x12);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x56);
        assert_eq!(encoded[3], 0x78);
        assert_eq!(decoded, 0x12345678);
    }

    #[test]
    fn test_u32_add() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u32 = rng.gen();
        let operand_b: u32 = rng.gen();
        // Perform addition without overflow panic
        let expected_result = operand_a.wrapping_add(operand_b);

        let a = U32Variable::constant(&mut builder, operand_a);
        let b = U32Variable::constant(&mut builder, operand_b);
        let result = builder.add(a, b);
        let expected_result_var = U32Variable::constant(&mut builder, expected_result);

        builder.assert_is_equal(result.0, expected_result_var.0);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_sub() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u32 = rng.gen();
        let operand_b: u32 = rng.gen();
        let expected_result = operand_a.wrapping_sub(operand_b);

        let a = U32Variable::constant(&mut builder, operand_a);
        let b = U32Variable::constant(&mut builder, operand_b);
        let result = builder.sub(a, b);
        let expected_result_var = U32Variable::constant(&mut builder, expected_result);

        builder.assert_is_equal(result.0, expected_result_var.0);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_mul() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u32 = rng.gen();
        let operand_b: u32 = rng.gen();
        let expected_result = operand_a.wrapping_mul(operand_b);

        let a = U32Variable::constant(&mut builder, operand_a);
        let b = U32Variable::constant(&mut builder, operand_b);
        let result = builder.mul(a, b);
        let expected_result_var = U32Variable::constant(&mut builder, expected_result);

        builder.assert_is_equal(result.0, expected_result_var.0);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
