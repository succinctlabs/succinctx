use ethers::types::U256;
use itertools::Itertools;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::{BasicVariable, U32Variable};
use crate::builder::CircuitBuilder;

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
pub struct U256Variable(Vec<U32Variable>);

impl BasicVariable for U256Variable {
    type Value = U256;

    fn init(builder: &mut CircuitBuilder) -> Self {
        Self((0..4).map(|_| U32Variable::init(builder)).collect_vec())
    }

    fn constant(builder: &mut CircuitBuilder, value: Self::Value) -> Self {
        let limbs = to_limbs(value);
        Self(
            (0..4)
                .map(|i| U32Variable::constant(builder, limbs[i]))
                .collect_vec(),
        )
    }

    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> Self::Value {
        to_u256([
            self.0[0].value(witness),
            self.0[1].value(witness),
            self.0[2].value(witness),
            self.0[3].value(witness),
        ])
    }

    fn set(&self, witness: &mut GeneratedValues<GoldilocksField>, value: Self::Value) {
        let limbs = to_limbs(value);
        for i in 0..4 {
            self.0[i].set(witness, limbs[i]);
        }
    }
}

fn to_limbs(value: U256) -> [u32; 4] {
    let mut bytes = [0u8; 32];
    value.to_little_endian(&mut bytes.as_mut());
    [
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
        u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
    ]
}

fn to_u256(limbs: [u32; 4]) -> U256 {
    let mut bytes = [0u8; 32];
    for (i, &limb) in limbs.iter().enumerate() {
        bytes[i * 4..(i + 1) * 4].copy_from_slice(&limb.to_le_bytes());
    }
    U256::from_little_endian(&bytes)
}
