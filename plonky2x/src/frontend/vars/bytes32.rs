use std::fmt::Debug;

use ethers::types::H256;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{ByteVariable, CircuitVariable, EvmVariable, U256Variable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::BytesVariable;

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

impl CircuitVariable for Bytes32Variable {
    type ValueType<F: RichField> = H256;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self(BytesVariable::constant(
            builder,
            value.as_bytes().try_into().unwrap(),
        ))
    }

    fn variables(&self) -> Vec<super::Variable> {
        self.0.variables()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables(variables))
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
