use std::fmt::Debug;

use ethers::types::U64;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::uint32_n::{EthersUint, U32NVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::*;

const NUM_LIMBS: usize = 2;

impl EthersUint<NUM_LIMBS> for U64 {
    fn to_little_endian(&self, bytes: &mut [u8]) {
        self.to_little_endian(bytes);
    }

    fn from_little_endian(slice: &[u8]) -> Self {
        Self::from_little_endian(slice)
    }

    fn to_big_endian(&self, bytes: &mut [u8]) {
        self.to_big_endian(bytes);
    }

    fn from_big_endian(slice: &[u8]) -> Self {
        Self::from_big_endian(slice)
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }
}

/// A variable in the circuit representing a u64 value. Under the hood, it is represented as
/// two U32Variable elements.
#[derive(Debug, Clone, Copy)]
pub struct U64Variable(pub U32NVariable<U64, NUM_LIMBS>);

impl CircuitVariable for U64Variable {
    type ValueType<F: RichField> = U64;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(U32NVariable::<U64, 2>::constant(builder, value))
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

impl EvmVariable for U64Variable {
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
        U64::from_big_endian(bytes)
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Zero<F, D> for U64Variable {
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self {
        Self(U32NVariable::zero(builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> One<F, D> for U64Variable {
    fn one(builder: &mut CircuitBuilder<F, D>) -> Self {
        Self(U32NVariable::one(builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Mul<F, D> for U64Variable {
    type Output = Self;

    fn mul(self, rhs: U64Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Self(U32NVariable::mul(self.0, rhs.0, builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Add<F, D> for U64Variable {
    type Output = Self;

    fn add(self, rhs: U64Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Self(U32NVariable::add(self.0, rhs.0, builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Sub<F, D> for U64Variable {
    type Output = Self;

    fn sub(self, rhs: U64Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Self(U32NVariable::sub(self.0, rhs.0, builder))
    }
}

#[cfg(test)]
mod tests {
    use ethers::types::U64;
    use rand::Rng;

    use crate::frontend::uint::uint64::U64Variable;
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    #[test]
    fn test_u64_evm() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut var_bytes = vec![];
        for i in 0..8 {
            let byte = ByteVariable::constant(&mut builder, i as u8);
            var_bytes.push(byte);
        }
        let decoded = U64Variable::decode(&mut builder, &var_bytes);
        let encoded = decoded.encode(&mut builder);
        let redecoded = U64Variable::decode(&mut builder, &encoded[0..8]);
        for i in 0..2 {
            builder.assert_is_equal(decoded.0.limbs[i].0, redecoded.0.limbs[i].0);
        }
        for i in 0..8 {
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
    fn test_u64_evm_value() {
        type F = GoldilocksField;

        let val = 0x123456789abcdef0_u64;
        let encoded = U64Variable::encode_value::<F>(U64([val]));
        let decoded = U64Variable::decode_value::<F>(&encoded);
        assert_eq!(encoded[0], 0x12);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x56);
        assert_eq!(encoded[3], 0x78);
        assert_eq!(encoded[4], 0x9a);
        assert_eq!(encoded[5], 0xbc);
        assert_eq!(encoded[6], 0xde);
        assert_eq!(encoded[7], 0xf0);
        assert_eq!(decoded, U64([0x123456789abcdef0]));
    }

    #[test]
    fn test_u64_add() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u64 = rng.gen();
        let operand_b: u64 = rng.gen();
        // Perform addition without overflow panic
        let expected_result = operand_a.wrapping_add(operand_b);

        let a = U64Variable::constant(&mut builder, U64([operand_a]));
        let b = U64Variable::constant(&mut builder, U64([operand_b]));
        let result = builder.add(a, b);
        let expected_result_var = U64Variable::constant(&mut builder, U64([expected_result]));

        for i in 0..2 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u64_sub() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u64 = rng.gen();
        let operand_b: u64 = rng.gen();
        let expected_result = operand_a.wrapping_sub(operand_b);

        let a = U64Variable::constant(&mut builder, U64([operand_a]));
        let b = U64Variable::constant(&mut builder, U64([operand_b]));
        let result = builder.sub(a, b);
        let expected_result_var = U64Variable::constant(&mut builder, U64([expected_result]));

        for i in 0..2 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u64_mul() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut rng = rand::thread_rng();
        let operand_a: u64 = rng.gen();
        let operand_b: u64 = rng.gen();
        let expected_result = operand_a.wrapping_mul(operand_b);

        let a = U64Variable::constant(&mut builder, U64([operand_a]));
        let b = U64Variable::constant(&mut builder, U64([operand_b]));
        let result = builder.mul(a, b);
        let expected_result_var = U64Variable::constant(&mut builder, U64([expected_result]));

        for i in 0..2 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
