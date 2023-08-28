use std::fmt::Debug;

use ethers::types::U128;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::uint32_n::{U32NVariable, ValueTrait};
use super::AlgebraicVariable;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::ByteVariable;

impl ValueTrait for U128 {
    fn to_limbs<const N: usize>(self) -> [u32; N] {
        let mut bytes = [0u8; 16];
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

const NUM_LIMBS: usize = 4;

/// A variable in the circuit representing a u64 value. Under the hood, it is represented as
/// two U32Variable elements.
#[derive(Debug, Clone, Copy)]
pub struct U128Variable(pub U32NVariable<U128, NUM_LIMBS>);

impl CircuitVariable for U128Variable {
    type ValueType<F: RichField> = U128;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(U32NVariable::constant(builder, value))
    }

    fn variables(&self) -> Vec<Variable> {
        U32NVariable::variables(&self.0)
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self(U32NVariable::from_variables(variables))
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        U32NVariable::get(&self.0, witness)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        U32NVariable::set(&self.0, witness, value)
    }
}

impl EvmVariable for U128Variable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        U32NVariable::encode(&self.0, builder)
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        Self(U32NVariable::decode(builder, bytes))
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        let mut bytes = [0u8; NUM_LIMBS * 4];
        value.to_big_endian(&mut bytes);
        bytes.to_vec()
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        U128::from_big_endian(bytes)
    }
}

impl AlgebraicVariable for U128Variable {
    /// Returns the zero value of the variable.
    fn zero<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::zero(builder))
    }

    /// Returns the one value of the variable.
    fn one<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::one(builder))
    }

    // Adds two variables together.
    fn add<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        Self(U32NVariable::add(&self.0, builder, &other.0))
    }

    // Subtracts two variables.
    fn sub<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        Self(U32NVariable::sub(&self.0, builder, &other.0))
    }

    // Multiplies two variables.
    fn mul<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        Self(U32NVariable::mul(&self.0, builder, &other.0))
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
    use ethers::types::U128;
    use rand::Rng;

    use crate::frontend::uint::uint128::U128Variable;
    use crate::frontend::uint::AlgebraicVariable;
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    #[test]
    fn test_u128_evm() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut var_bytes = vec![];
        for i in 0..16 {
            let byte = ByteVariable::constant(&mut builder, i as u8);
            var_bytes.push(byte);
        }
        let decoded = U128Variable::decode(&mut builder, &var_bytes);
        let encoded = decoded.encode(&mut builder);
        let redecoded = U128Variable::decode(&mut builder, &encoded[0..16]);
        for i in 0..4 {
            builder.assert_is_equal(decoded.0.limbs[i].0, redecoded.0.limbs[i].0);
        }
        for i in 0..16 {
            for j in 0..8 {
                builder.assert_is_equal(encoded[i].0[j].0, var_bytes[i].0[j].0);
            }
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u128_evm_value() {
        type F = GoldilocksField;

        let val = 0x123456789abcdef0_u64;
        let encoded = U128Variable::encode_value::<F>(U128([val, val]));
        let decoded = U128Variable::decode_value::<F>(&encoded);
        assert_eq!(encoded[0], 0x12);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x56);
        assert_eq!(encoded[3], 0x78);
        assert_eq!(encoded[4], 0x9a);
        assert_eq!(encoded[5], 0xbc);
        assert_eq!(encoded[6], 0xde);
        assert_eq!(encoded[7], 0xf0);
        assert_eq!(encoded[8], 0x12);
        assert_eq!(encoded[9], 0x34);
        assert_eq!(encoded[10], 0x56);
        assert_eq!(encoded[11], 0x78);
        assert_eq!(encoded[12], 0x9a);
        assert_eq!(encoded[13], 0xbc);
        assert_eq!(encoded[14], 0xde);
        assert_eq!(encoded[15], 0xf0);

        assert_eq!(decoded, U128([0x123456789abcdef0, 0x123456789abcdef0]));
    }

    #[test]
    fn test_u128_add() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u128 = rng.gen();
        let operand_b: u128 = rng.gen();

        // Perform addition without overflow panic
        let expected_result = operand_a.wrapping_add(operand_b);

        let a = U128Variable::constant(
            &mut builder,
            U128([operand_a as u64, (operand_a >> 64) as u64]),
        );
        let b = U128Variable::constant(
            &mut builder,
            U128([operand_b as u64, (operand_b >> 64) as u64]),
        );
        let result = a.add(&mut builder, &b);
        let expected_result_var = U128Variable::constant(
            &mut builder,
            U128([expected_result as u64, (expected_result >> 64) as u64]),
        );

        for i in 0..4 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u128_sub() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u128 = rng.gen();
        let operand_b: u128 = rng.gen();

        let expected_result = operand_a.wrapping_sub(operand_b);

        let a = U128Variable::constant(
            &mut builder,
            U128([operand_a as u64, (operand_a >> 64) as u64]),
        );
        let b = U128Variable::constant(
            &mut builder,
            U128([operand_b as u64, (operand_b >> 64) as u64]),
        );

        let result = a.sub(&mut builder, &b);
        let expected_result_var = U128Variable::constant(
            &mut builder,
            U128([expected_result as u64, (expected_result >> 64) as u64]),
        );

        for i in 0..4 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u128_mul() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u128 = rng.gen();
        let operand_b: u128 = rng.gen();

        let expected_result = operand_a.wrapping_mul(operand_b);

        let a = U128Variable::constant(
            &mut builder,
            U128([operand_a as u64, (operand_a >> 64) as u64]),
        );
        let b = U128Variable::constant(
            &mut builder,
            U128([operand_b as u64, (operand_b >> 64) as u64]),
        );

        let result = a.mul(&mut builder, &b);
        let expected_result_var = U128Variable::constant(
            &mut builder,
            U128([expected_result as u64, (expected_result >> 64) as u64]),
        );

        for i in 0..4 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
