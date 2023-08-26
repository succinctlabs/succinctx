use std::fmt::Debug;

use array_macro::array;
use ethers::types::U256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{CircuitVariable, EvmVariable, U32Variable};
use crate::prelude::{ByteVariable, Variable};

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy)]
pub struct U256Variable(pub [U32Variable; 8]);

impl CircuitVariable for U256Variable {
    type ValueType<F: RichField> = U256;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => U32Variable::init(builder); 8])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        let limbs = to_limbs(value);
        Self(array![i => U32Variable::constant(builder, limbs[i]); 8])
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.iter().map(|x| x.0).collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 8);
        Self(array![i => U32Variable(variables[i]); 8])
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        to_u256([
            self.0[0].get(witness),
            self.0[1].get(witness),
            self.0[2].get(witness),
            self.0[3].get(witness),
            self.0[4].get(witness),
            self.0[5].get(witness),
            self.0[6].get(witness),
            self.0[7].get(witness),
        ])
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        let limbs = to_limbs(value);
        for i in 0..8 {
            self.0[i].set(witness, limbs[i]);
        }
    }
}

fn to_limbs(value: U256) -> [u32; 8] {
    let mut bytes = [0u8; 32];
    value.to_little_endian(&mut bytes);
    [
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
        u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
        u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
        u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
        u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
    ]
}

fn to_u256(limbs: [u32; 8]) -> U256 {
    let mut bytes = [0u8; 32];
    for (i, &limb) in limbs.iter().enumerate() {
        bytes[i * 4..(i + 1) * 4].copy_from_slice(&limb.to_le_bytes());
    }
    U256::from_little_endian(&bytes)
}

impl EvmVariable for U256Variable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        self.0
            .iter()
            .flat_map(|x| x.encode(builder))
            .collect::<Vec<_>>()
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        assert_eq!(bytes.len(), 32);
        let mut limbs = [U32Variable::init(builder); 8];
        for i in 0..8 {
            limbs[i] = U32Variable::decode(builder, &bytes[i * 4..(i + 1) * 4]);
        }
        Self(limbs)
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        bytes.to_vec()
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        U256::from_big_endian(bytes)
    }
}

#[cfg(test)]
mod tests {
    use ethers::types::U256;

    use super::U32Variable;
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
            builder.assert_is_equal(decoded.0[i].0, redecoded.0[i].0);
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

        let mut u8bytes = [0_u8; 32];
        for i in 0..32 {
            u8bytes[i] = rand::random::<u8>();
        }
        let decoded = U256Variable::decode_value::<F>(&u8bytes);
        let encoded = U256Variable::encode_value::<F>(decoded);

        let realu256 = U256::from_big_endian(&u8bytes);

        assert_eq!(realu256, decoded);
        for i in 0..8 {
            assert_eq!(encoded[i * 4..(i + 1) * 4], u8bytes[i * 4..(i + 1) * 4]);
        }
    }
}
