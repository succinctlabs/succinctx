use std::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::AlgebraicVariable;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::{BoolVariable, ByteVariable, One, Zero};

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U32Variable(pub Variable);

impl CircuitVariable for U32Variable {
    type ValueType<F: RichField> = u32;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(Variable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(Variable::constant(builder, F::from_canonical_u32(value)))
    }

    fn variables(&self) -> Vec<Variable> {
        vec![self.0]
    }

    fn from_variables(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 1);
        Self(variables[0])
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let v = witness.get_target(self.0 .0);
        v.to_canonical_u64() as u32
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        witness.set_target(self.0 .0, F::from_canonical_u32(value));
    }
}

impl EvmVariable for U32Variable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        let mut bytes = vec![];
        let bits = builder.api.split_le(self.0 .0, 32);
        for i in (0..4).rev() {
            let mut arr: [BoolVariable; 8] = [builder._false(); 8];
            let byte = bits[i * 8..(i + 1) * 8].to_vec();
            byte.iter()
                .rev()
                .enumerate()
                .try_fold(&mut arr, |arr, (j, &bit)| {
                    arr[j] = BoolVariable(Variable(bit.target));
                    Some(arr)
                });
            bytes.push(ByteVariable(arr));
        }
        bytes
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        assert_eq!(bytes.len(), 4);
        let mut bits = vec![];
        for byte in bytes.iter() {
            bits.extend_from_slice(&byte.0);
        }
        let target = builder.api.le_sum(
            bits.iter()
                .rev()
                .map(|bit| BoolTarget::new_unsafe(bit.0 .0)),
        );
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

impl AlgebraicVariable for U32Variable {
    /// Returns the zero value of the variable.
    fn zero<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        let zero = Variable::zero(builder);
        Self(zero)
    }

    /// Returns the one value of the variable.
    fn one<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        let one = Variable::one(builder);
        Self(one)
    }

    // Adds two variables together.
    fn add<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        let self_target = self.0 .0;
        let other_target = other.0 .0;
        let self_biguint = BigUintTarget {
            limbs: vec![U32Target(self_target)],
        };
        let other_biguint = BigUintTarget {
            limbs: vec![U32Target(other_target)],
        };

        let sum_biguint = builder.api.add_biguint(&self_biguint, &other_biguint);

        // Get the least significant limb
        let sum = sum_biguint.limbs[0].0;

        Self(Variable(sum))
    }

    // Subtracts two variables.
    fn sub<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        let self_target = self.0 .0;
        let other_target = other.0 .0;
        let self_biguint = BigUintTarget {
            limbs: vec![U32Target(self_target)],
        };
        let other_biguint = BigUintTarget {
            limbs: vec![U32Target(other_target)],
        };

        let sum_biguint = builder.api.sub_biguint(&self_biguint, &other_biguint);
        let sum = sum_biguint.limbs[0].0;
        Self(Variable(sum))
    }

    // Multiplies two variables.
    fn mul<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        let self_target = self.0 .0;
        let other_target = other.0 .0;
        let self_biguint = BigUintTarget {
            limbs: vec![U32Target(self_target)],
        };
        let other_biguint = BigUintTarget {
            limbs: vec![U32Target(other_target)],
        };

        let sum_biguint = builder.api.mul_biguint(&self_biguint, &other_biguint);

        // Get the least significant limb
        let sum = sum_biguint.limbs[0].0;
        Self(Variable(sum))
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
}
