use std::fmt::Debug;

use itertools::Itertools;
use plonky2::iop::target::BoolTarget;

use crate::frontend::uint::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::uint::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::frontend::uint::num::u32::gadgets::multiple_comparison::list_lte_circuit;
use crate::frontend::vars::EvmVariable;
use crate::prelude::*;

/// A variable in the circuit representing a u32 value.
///
/// Under the hood, it is represented as a single field element.
#[derive(Debug, Clone, Copy)]
#[allow(clippy::manual_non_exhaustive)]
pub struct U32Variable {
    pub variable: Variable,
    /// This private field is here to force all instantiations to go the methods below.
    _private: (),
}

impl CircuitVariable for U32Variable {
    type ValueType<F: RichField> = u32;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            variable: Variable::init_unsafe(builder),
            _private: (),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        vec![self.variable]
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self {
            variable: variables[0],
            _private: (),
        }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        let bits = builder.api.u32_to_bits_le(U32Target::from(*self));
        let reconstructed_val = builder.api.le_sum(bits.iter());
        builder.assert_is_equal(self.variable, Variable(reconstructed_val))
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
        let mut bits = builder.api.split_le(self.variable.0, 32);
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

        // Convert into an array of BoolTargets.  The assert above will guarantee that this vector
        // will have a size of 32.
        let mut bits = bytes.iter().flat_map(|byte| byte.targets()).collect_vec();
        bits.reverse();

        // Sum up the BoolTargets into a single target.
        let target = builder
            .api
            .le_sum(bits.into_iter().map(BoolTarget::new_unsafe));

        // Target is composed of 32 bool targets, so it will be within U32Variable's range.
        Self::from_variables_unsafe(&[Variable(target)])
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

impl From<U32Target> for U32Variable {
    fn from(v: U32Target) -> Self {
        // U32Target's range is the same as U32Variable's.
        Self::from_variables_unsafe(&[Variable(v.target)])
    }
}

impl<L: PlonkParameters<D>, const D: usize> LessThanOrEqual<L, D> for U32Variable {
    #[must_use]
    fn lte(self, rhs: Self, builder: &mut CircuitBuilder<L, D>) -> BoolVariable {
        list_lte_circuit(&mut builder.api, self.targets(), rhs.targets(), 32).into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Zero<L, D> for U32Variable {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
        let zero = Variable::zero(builder);

        // "zero" is within u32.
        Self::from_variables_unsafe(&[zero])
    }
}

impl<L: PlonkParameters<D>, const D: usize> One<L, D> for U32Variable {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
        let one = Variable::one(builder);

        // "one" is within u32.
        Self::from_variables_unsafe(&[one])
    }
}

impl<L: PlonkParameters<D>, const D: usize> Mul<L, D> for U32Variable {
    type Output = Self;

    fn mul(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_biguint = BigUintTarget {
            limbs: vec![self.into()],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![rhs.into()],
        };

        let product_biguint = builder.api.mul_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant u32 limb.
        product_biguint.limbs[0].into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Div<L, D> for U32Variable {
    type Output = Self;

    fn div(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_biguint = BigUintTarget {
            limbs: vec![self.into()],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![rhs.into()],
        };

        let quotient_biguint = builder.api.div_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant u32 limb.
        quotient_biguint.limbs[0].into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Add<L, D> for U32Variable {
    type Output = Self;

    fn add(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_biguint = BigUintTarget {
            limbs: vec![self.into()],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![rhs.into()],
        };

        let sum_biguint = builder.api.add_biguint(&self_biguint, &rhs_biguint);

        // Get the least significant limb
        sum_biguint.limbs[0].into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Sub<L, D> for U32Variable {
    type Output = Self;

    fn sub(self, rhs: U32Variable, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let self_biguint = BigUintTarget {
            limbs: vec![self.into()],
        };
        let rhs_biguint = BigUintTarget {
            limbs: vec![rhs.into()],
        };

        let diff_biguint = builder.api.sub_biguint(&self_biguint, &rhs_biguint);
        diff_biguint.limbs[0].into()
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
                .rev()
                .map(|b| (*b).into())
                .collect::<Vec<BoolTarget>>()
                .into_iter(),
        );
        // It's okay to use from_targets which is unsafe, for the same reason as above.
        Self::from_targets(&[var])
    }

    pub fn to_be_bits<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> [BoolVariable; 32] {
        let mut bits = builder.api.split_le(self.variable.0, 32);
        bits.reverse();
        bits.iter()
            .map(|b| (*b).into())
            .collect_vec()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_to_u64() {
        setup_logger();
        let mut builder = CircuitBuilder::<L, D>::new();
        let var = U32Variable::constant(&mut builder, 0x12345678);
        let var_u64 = var.to_u64(&mut builder);
        builder.watch(&var_u64, "var_u64");
        let circuit = builder.build();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_evm() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let var = U32Variable::constant(&mut builder, 0x12345678);

        let encoded = var.encode(&mut builder);

        let bytes = [0x12, 0x34, 0x56, 0x78];

        for (i, byte) in encoded.iter().enumerate() {
            let expected = ByteVariable::constant(&mut builder, bytes[i]).0;
            byte.0.iter().enumerate().for_each(|(j, &bit)| {
                builder.assert_is_equal(bit.variable, expected[j].variable);
            });
        }

        let decoded = U32Variable::decode(&mut builder, &encoded[0..4]);
        builder.assert_is_equal(decoded.variable, var.variable);

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

        builder.assert_is_equal(result.variable, expected_result_var.variable);

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

        builder.assert_is_equal(result.variable, expected_result_var.variable);

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

        builder.assert_is_equal(result.variable, expected_result_var.variable);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
