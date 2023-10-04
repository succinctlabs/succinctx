use core::str::Bytes;
use std::fmt::Debug;

use ethers::types::H256;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{
    ByteVariable, BytesVariable, CircuitVariable, EvmVariable, SSZVariable, U256Variable, Variable,
};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

/// A variable in the circuit representing a byte32 value.
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub BytesVariable<32>);

impl Bytes32Variable {
    pub fn as_bytes(&self) -> [ByteVariable; 32] {
        self.0 .0
    }

    pub fn as_u256<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> U256Variable {
        U256Variable::decode(builder, &self.0 .0)
    }
}

impl From<[ByteVariable; 32]> for Bytes32Variable {
    fn from(bytes: [ByteVariable; 32]) -> Self {
        Self(BytesVariable(bytes))
    }
}

impl From<&[ByteVariable]> for Bytes32Variable {
    fn from(bytes: &[ByteVariable]) -> Self {
        let bytes_fixed: [ByteVariable; 32] = bytes.try_into().unwrap();
        Self(BytesVariable(bytes_fixed))
    }
}

impl CircuitVariable for Bytes32Variable {
    type ValueType<F: RichField> = H256;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self(BytesVariable::init_unsafe(builder))
    }

    fn variables(&self) -> Vec<super::Variable> {
        self.0.variables()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables_unsafe(variables))
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder)
    }

    fn nb_elements() -> usize {
        BytesVariable::<32>::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        BytesVariable::<32>::elements(value.as_bytes().try_into().unwrap())
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        H256::from_slice(&BytesVariable::<32>::from_elements(elements))
    }
}

impl EvmVariable for Bytes32Variable {
    fn encode<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<ByteVariable> {
        self.0.encode(builder)
    }

    fn decode<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        bytes: &[super::ByteVariable],
    ) -> Self {
        Self(BytesVariable::decode(builder, bytes))
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        value.as_bytes().to_vec()
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        H256::from_slice(bytes)
    }
}

impl SSZVariable for Bytes32Variable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        let bytes = self.encode(builder);
        Bytes32Variable(BytesVariable::<32>(bytes.try_into().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use ethers::types::U256;
    use plonky2::iop::witness::PartialWitness;

    use super::Bytes32Variable;
    use crate::frontend::uint::uint256::U256Variable;
    use crate::prelude::{CircuitVariable, DefaultBuilder};
    use crate::utils::bytes32;

    #[test]
    fn test_bytes32_as_u256() {
        let mut builder = DefaultBuilder::new();

        let b32 = Bytes32Variable::constant(
            &mut builder,
            bytes32!("0xf0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b"),
        );

        let u256 = b32.as_u256(&mut builder);
        let expected = U256Variable::constant(
            &mut builder,
            U256::from_dec_str(
                "108959270400061671294053818573968651411470832267186275529291850190552309358907",
            )
            .unwrap(),
        );
        builder.assert_is_equal(u256, expected);

        let circuit = builder.build();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
