use std::fmt::Debug;

use ethers::types::U256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::uint32_n::{U32NVariable, ValueTrait};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::*;

impl ValueTrait for U256 {
    fn to_limbs<const N: usize>(self) -> [u32; N] {
        let mut bytes = [0u8; 32];
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

const NUM_LIMBS: usize = 8;

/// A variable in the circuit representing a u64 value. Under the hood, it is represented as
/// two U32Variable elements.
#[derive(Debug, Clone, Copy)]
pub struct U256Variable(pub U32NVariable<U256, NUM_LIMBS>);

impl CircuitVariable for U256Variable {
    type ValueType<F: RichField> = U256;

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

impl EvmVariable for U256Variable {
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
        U256::from_big_endian(bytes)
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Zero<F, D> for U256Variable {
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self {
        Self(U32NVariable::zero(builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> One<F, D> for U256Variable {
    fn one(builder: &mut CircuitBuilder<F, D>) -> Self {
        Self(U32NVariable::one(builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Mul<F, D> for U256Variable {
    type Output = Self;

    fn mul(self, rhs: U256Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Self(U32NVariable::mul(self.0, rhs.0, builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Add<F, D> for U256Variable {
    type Output = Self;

    fn add(self, rhs: U256Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Self(U32NVariable::add(self.0, rhs.0, builder))
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Sub<F, D> for U256Variable {
    type Output = Self;

    fn sub(self, rhs: U256Variable, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        Self(U32NVariable::sub(self.0, rhs.0, builder))
    }
}
#[cfg(test)]
mod tests {
    use ethers::types::U256;
    use rand::rngs::OsRng;
    use rand::Rng;

    use crate::frontend::uint::uint256::U256Variable;
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    #[test]
    fn test_u256_evm() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let mut var_bytes = vec![];
        for i in 0..32 {
            let byte = ByteVariable::constant(&mut builder, i as u8);
            var_bytes.push(byte);
        }
        let decoded = U256Variable::decode(&mut builder, &var_bytes);
        let encoded = decoded.encode(&mut builder);
        let redecoded = U256Variable::decode(&mut builder, &encoded[0..32]);
        for i in 0..8 {
            builder.assert_is_equal(decoded.0.limbs[i].0, redecoded.0.limbs[i].0);
        }
        for i in 0..32 {
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
    fn test_u256_evm_value() {
        type F = GoldilocksField;

        let val = 0x123456789abcdef0_u64;
        let encoded = U256Variable::encode_value::<F>(U256([val, val, val, val]));
        let decoded = U256Variable::decode_value::<F>(&encoded);
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
        assert_eq!(encoded[16], 0x12);
        assert_eq!(encoded[17], 0x34);
        assert_eq!(encoded[18], 0x56);
        assert_eq!(encoded[19], 0x78);
        assert_eq!(encoded[20], 0x9a);
        assert_eq!(encoded[21], 0xbc);
        assert_eq!(encoded[22], 0xde);
        assert_eq!(encoded[23], 0xf0);
        assert_eq!(encoded[24], 0x12);
        assert_eq!(encoded[25], 0x34);
        assert_eq!(encoded[26], 0x56);
        assert_eq!(encoded[27], 0x78);
        assert_eq!(encoded[28], 0x9a);
        assert_eq!(encoded[29], 0xbc);
        assert_eq!(encoded[30], 0xde);
        assert_eq!(encoded[31], 0xf0);

        assert_eq!(
            decoded,
            U256([
                0x123456789abcdef0,
                0x123456789abcdef0,
                0x123456789abcdef0,
                0x123456789abcdef0
            ])
        );
    }

    #[test]
    fn test_u256_add() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut rng = OsRng;

        // Get the limbs for the a_operand
        let a_operand_0 = rng.gen();
        let a_operand_1 = rng.gen();
        let a_operand_2 = rng.gen();
        let a_operand_3 = rng.gen();
        let a = U256([a_operand_0, a_operand_1, a_operand_2, a_operand_3]);

        // Get the limbs for the b_operand
        let b_operand_0 = rng.gen();
        let b_operand_1 = rng.gen();
        let b_operand_2 = rng.gen();
        let b_operand_3 = rng.gen();
        let b = U256([b_operand_0, b_operand_1, b_operand_2, b_operand_3]);

        let (expected_value, _) = a.overflowing_add(b);

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = U256Variable::constant(&mut builder, a);
        let b = U256Variable::constant(&mut builder, b);
        let result = builder.add(a, b);
        let expected_result_var = U256Variable::constant(&mut builder, expected_value);

        for i in 0..8 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u256_sub() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut rng = OsRng;

        // Get the limbs for the a_operand
        let a_operand_0 = rng.gen();
        let a_operand_1 = rng.gen();
        let a_operand_2 = rng.gen();
        let a_operand_3 = rng.gen();
        let a = U256([a_operand_0, a_operand_1, a_operand_2, a_operand_3]);

        // Get the limbs for the b_operand
        let b_operand_0 = rng.gen();
        let b_operand_1 = rng.gen();
        let b_operand_2 = rng.gen();
        let b_operand_3 = rng.gen();
        let b = U256([b_operand_0, b_operand_1, b_operand_2, b_operand_3]);

        let (expected_value, _) = a.overflowing_sub(b);

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = U256Variable::constant(&mut builder, a);
        let b = U256Variable::constant(&mut builder, b);
        let result = builder.sub(a, b);
        let expected_result_var = U256Variable::constant(&mut builder, expected_value);

        for i in 0..8 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u256_mul() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut rng = OsRng;

        // Get the limbs for the a_operand
        let a_operand_0 = rng.gen();
        let a_operand_1 = rng.gen();
        let a_operand_2 = rng.gen();
        let a_operand_3 = rng.gen();
        let a = U256([a_operand_0, a_operand_1, a_operand_2, a_operand_3]);

        // Get the limbs for the b_operand
        let b_operand_0 = rng.gen();
        let b_operand_1 = rng.gen();
        let b_operand_2 = rng.gen();
        let b_operand_3 = rng.gen();
        let b = U256([b_operand_0, b_operand_1, b_operand_2, b_operand_3]);

        let (expected_value, _) = a.overflowing_mul(b);

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = U256Variable::constant(&mut builder, a);
        let b = U256Variable::constant(&mut builder, b);
        let result = builder.mul(a, b);
        let expected_result_var = U256Variable::constant(&mut builder, expected_value);

        for i in 0..8 {
            builder.assert_is_equal(result.0.limbs[i].0, expected_result_var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
