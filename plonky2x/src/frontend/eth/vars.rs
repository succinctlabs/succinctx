use std::fmt::Debug;

use ethers::types::H160;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{ByteVariable, BytesVariable, CircuitVariable, EvmVariable};
use crate::prelude::Variable;

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub BytesVariable<48>);

impl CircuitVariable for BLSPubkeyVariable {
    type ValueType<F: RichField> = [u8; 48];

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(BytesVariable::constant(builder, value))
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.variables()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables(variables))
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

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(BytesVariable::constant(
            builder,
            value.as_bytes().try_into().expect("wrong slice length"),
        ))
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.variables()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables(variables))
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

impl EvmVariable for AddressVariable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        self.0.encode(builder)
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self {
        Self(BytesVariable::decode(builder, bytes))
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        value.as_bytes().to_vec()
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        H160::from_slice(bytes)
    }
}
