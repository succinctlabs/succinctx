use std::fmt::Debug;

use ethers::types::H256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{ByteSerializable, ByteVariable, CircuitVariable, EvmVariable, FieldSerializable};
use crate::builder::CircuitBuilder;
use crate::vars::BytesVariable;

/// A variable in the circuit representing a byte32 value.
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub BytesVariable<32>);

impl CircuitVariable for Bytes32Variable {
    type ValueType<F: RichField> = H256;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn targets(&self) -> Vec<Target> {
        self.0.targets()
    }

    fn from_targets(targets: &[Target]) -> Self {
        Self(BytesVariable::from_targets(targets))
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let bytes = self.0.get(witness);
        H256::from_slice(&bytes)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
    }
}

impl EvmVariable for Bytes32Variable {
    type ValueType<F: RichField> = H256;

    fn bytes<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        self.0.bytes(builder)
    }

    fn from_bytes<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[super::ByteVariable],
    ) -> Self {
        Self(BytesVariable::from_bytes(builder, bytes))
    }
}

impl<F: RichField> FieldSerializable<F> for H256 {
    fn nb_elements() -> usize {
        256
    }

    fn elements(&self) -> Vec<F> {
        self.as_bytes()
            .into_iter()
            .flat_map(|x| x.elements())
            .collect()
    }

    fn from_elements(elements: &[F]) -> Self {
        assert_eq!(elements.len(), 256);
        let mut acc = [0u8; 32];
        for i in 0..32 {
            acc[i] = u8::from_elements(&elements[i * 8..(i + 1) * 8])
        }
        H256::from(acc)
    }
}

impl<F: RichField> ByteSerializable<F> for H256 {
    fn nb_bytes() -> usize {
        32
    }

    fn bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 32);
        let mut acc = [0u8; 32];
        acc.copy_from_slice(bytes);
        H256::from(acc)
    }
}
