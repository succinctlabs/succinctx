use std::fmt::Debug;

use ethers::types::H160;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{BytesVariable, CircuitVariable, FieldSerializable};

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub BytesVariable<48>);

impl CircuitVariable for BLSPubkeyVariable {
    type ValueType<F: RichField> = [u8; 48];

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
        self.0.get(witness)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub BytesVariable<20>);

impl CircuitVariable for AddressVariable {
    type ValueType<F: RichField> = H160;

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
        H160::from_slice(&self.0.get(witness))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(
            witness,
            value.as_bytes().try_into().expect("wrong slice length"),
        )
    }
}

impl<F: RichField> FieldSerializable<F> for H160 {
    fn nb_elements() -> usize {
        160
    }

    fn elements(&self) -> Vec<F> {
        self.as_bytes()
            .into_iter()
            .flat_map(|x| x.elements())
            .collect()
    }

    fn from_elements(elements: &[F]) -> Self {
        let mut bytes = [0u8; 20];
        for i in 0..20 {
            bytes[i] = u8::from_elements(&elements[i * 8..(i + 1) * 8]);
        }
        H160::from_slice(&bytes)
    }
}
