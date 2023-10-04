use std::fmt::Debug;

use ethers::types::H160;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{
    ByteVariable, BytesVariable, CircuitVariable, EvmVariable, SSZVariable,
};
use crate::prelude::{Bytes32Variable, Variable};

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub BytesVariable<48>);

impl CircuitVariable for BLSPubkeyVariable {
    type ValueType<F: RichField> = [u8; 48];

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self(BytesVariable::init_unsafe(builder))
    }

    fn nb_elements() -> usize {
        BytesVariable::<48>::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        BytesVariable::<48>::elements(value)
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        BytesVariable::<48>::from_elements(elements)
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.variables()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables_unsafe(variables))
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub BytesVariable<20>);

impl CircuitVariable for AddressVariable {
    type ValueType<F: RichField> = H160;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self(BytesVariable::init_unsafe(builder))
    }

    fn nb_elements() -> usize {
        BytesVariable::<20>::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        BytesVariable::<20>::elements(value.into())
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        H160::from_slice(&BytesVariable::<20>::from_elements(elements))
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.variables()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables_unsafe(variables))
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
    }
}

impl EvmVariable for AddressVariable {
    fn encode<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<ByteVariable> {
        self.0.encode(builder)
    }

    fn decode<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
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

impl SSZVariable for AddressVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        let mut bytes = self.encode(builder);
        bytes.reverse();
        let zero = builder.constant::<ByteVariable>(0);
        bytes.extend([zero; 12]);
        Bytes32Variable(BytesVariable::<32>(bytes.try_into().unwrap()))
    }
}
