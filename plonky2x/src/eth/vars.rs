use ethers::types::H160;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::vars::{BytesVariable, CircuitVariable};

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub BytesVariable<48>);

impl CircuitVariable for BLSPubkeyVariable {
    type ValueType = Vec<u8>;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        Self(BytesVariable::constant(builder, value))
    }

    fn targets(&self) -> Vec<Target> {
        self.0.targets()
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        self.0.value(witness)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        self.0.set(witness, value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub BytesVariable<20>);

impl CircuitVariable for AddressVariable {
    type ValueType = H160;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        Self(BytesVariable::constant(builder, value.as_bytes().to_vec()))
    }

    fn targets(&self) -> Vec<Target> {
        self.0.targets()
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        H160::from_slice(&self.0.value(witness))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        self.0.set(witness, value.as_bytes().to_vec())
    }
}
