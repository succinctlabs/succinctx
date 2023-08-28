use std::fmt::Debug;

use ethers::types::U64;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::uint32_n::{U32NVariable, ValueTrait};
use super::AlgebraicVariable;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::ByteVariable;

impl ValueTrait for U64 {
    fn to_limbs<const N: usize>(self) -> [u32; N] {
        let mut bytes = [0u8; 32];
        self.to_little_endian(&mut bytes);
        let mut ret: [u32; N] = [0; N];
        for i in (0..=N).step_by(4) {
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

const NUM_LIMBS: usize = 2;
type U64type = U64;

/// A variable in the circuit representing a u64 value. Under the hood, it is represented as
/// two U32Variable elements.
#[derive(Debug, Clone, Copy)]
pub struct U64Variable(pub U32NVariable<U64, NUM_LIMBS>);

impl CircuitVariable for U64Variable {
    type ValueType<F: RichField> = U64type;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::constant(builder, value))
    }

    fn variables(&self) -> Vec<Variable> {
        U32NVariable::<U64type, NUM_LIMBS>::variables(&self.0)
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::from_variables(
            variables,
        ))
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        U32NVariable::<U64type, NUM_LIMBS>::get(self, witness)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        U32NVariable::<U64type, NUM_LIMBS>::set(self, witness, value)
    }
}

impl EvmVariable for U64Variable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        U32NVariable::<U64type, NUM_LIMBS>::encode(self, builder)
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        U32NVariable::<U64type, NUM_LIMBS>::decode(builder, bytes)
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        U32NVariable::<U64type, NUM_LIMBS>::encode_value(value)
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        U32NVariable::<U64type, NUM_LIMBS>::decode_value(bytes)
    }
}

impl AlgebraicVariable for U64Variable {
    /// Returns the zero value of the variable.
    fn zero<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::zero(builder))
    }

    /// Returns the one value of the variable.
    fn one<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::one(builder))
    }

    // Adds two variables together.
    fn add<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::add(
            self, builder, other,
        ))
    }

    // Subtracts two variables.
    fn sub<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::sub(
            self, builder, other,
        ))
    }

    // Multiplies two variables.
    fn mul<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        other: &Self,
    ) -> Self {
        Self(U32NVariable::<U64type, NUM_LIMBS>::mul(
            self, builder, other,
        ))
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
    use ethers::types::U64;
    use rand::Rng;

    use crate::frontend::uint::uint64::U64Variable;
    use crate::frontend::uint::AlgebraicVariable;
    use crate::frontend::vars::EvmVariable;
    use crate::prelude::*;

    #[test]
    fn test_u64_evm() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let var = U64Variable::constant(&mut builder, U64([0x1234567812345678]));

        let encoded = var.encode(&mut builder);

        let bytes = [0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78];

        for (i, byte) in encoded.iter().enumerate() {
            let expected = ByteVariable::constant(&mut builder, bytes[i]).0;
            byte.0.iter().enumerate().for_each(|(j, &bit)| {
                builder.assert_is_equal(bit.0, expected[j].0);
            });
        }

        let decoded = U64Variable::decode(&mut builder, &encoded[0..4]);
        for i in 0..2 {
            builder.assert_is_equal(decoded.0.limbs[i].0, var.0.limbs[i].0);
        }

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_u32_evm_value() {
        type F = GoldilocksField;

        let val = 0x1234567812345678_u64;
        let encoded = U64Variable::encode_value::<F>(U64([val]));
        let decoded = U64Variable::decode_value::<F>(&encoded);
        assert_eq!(encoded[0], 0x12);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x56);
        assert_eq!(encoded[3], 0x78);
        assert_eq!(encoded[4], 0x12);
        assert_eq!(encoded[5], 0x34);
        assert_eq!(encoded[6], 0x56);
        assert_eq!(encoded[7], 0x78);
        assert_eq!(decoded, U64([0x1234567812345678]));
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
        let result = a.add(&mut builder, &b);
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
    fn test_u32_sub() {
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
        let result = a.sub(&mut builder, &b);
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
    fn test_u32_mul() {
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
        let result = a.mul(&mut builder, &b);
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
