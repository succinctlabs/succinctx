use std::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{CircuitVariable, EvmVariable, Variable};
use crate::prelude::{BoolVariable, ByteVariable};

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
