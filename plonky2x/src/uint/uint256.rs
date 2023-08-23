use array_macro::array;
use ethers::types::U256;
use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{CircuitVariable, U32Variable};

/// A variable in the circuit representing a u32 value. Under the hood, it is represented as
/// a single field element.
#[derive(Debug, Clone, Copy, Default)]
pub struct U256Variable(pub [U32Variable; 4]);

impl CircuitVariable for U256Variable {
    type ValueType<F> = U256;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => U32Variable::init(builder); 4])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        let limbs = to_limbs(value);
        Self(array![i => U32Variable::constant(builder, limbs[i]); 4])
    }

    fn targets(&self) -> Vec<Target> {
        self.0.iter().flat_map(|v| v.targets()).collect_vec()
    }

<<<<<<< HEAD:plonky2x/src/vars/uint256.rs
    fn from_targets(targets: &[Target]) -> Self {
        assert_eq!(targets.len(), 4);
        Self(array![i => U32Variable::from_targets(&[targets[i]]); 4])
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
=======
    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
>>>>>>> main:plonky2x/src/uint/uint256.rs
        to_u256([
            self.0[0].value(witness),
            self.0[1].value(witness),
            self.0[2].value(witness),
            self.0[3].value(witness),
        ])
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        let limbs = to_limbs(value);
        for i in 0..4 {
            self.0[i].set(witness, limbs[i]);
        }
    }
}

fn to_limbs(value: U256) -> [u32; 4] {
    let mut bytes = [0u8; 32];
    value.to_little_endian(&mut bytes);
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
