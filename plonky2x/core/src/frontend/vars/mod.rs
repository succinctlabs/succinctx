mod array;
mod boolean;
mod byte;
mod bytes;
mod bytes32;
mod collections;

mod stream;
mod variable;
use std::fmt::Debug;

pub use array::*;
pub use boolean::*;
pub use byte::*;
pub use bytes::*;
pub use bytes32::*;
use itertools::Itertools;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};
pub use stream::*;
pub use variable::*;

pub use super::uint::uint256::*;
pub use super::uint::uint32::*;
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

pub trait CircuitVariable: Debug + Clone + Sized + Sync + Send + 'static {
    /// The underlying type of the variable if it were not in a circuit.
    type ValueType<F: RichField>: Debug + Clone;

    /// Initializes the variable with no value in the circuit and checks that the variable is valid
    /// (i.e., range checks).
    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        let variable = Self::init_unsafe(builder);
        variable.assert_is_valid(builder);
        variable
    }

    /// Initialies the variable with no value and does not check that the variable is valid.
    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self;

    /// Initializes the variable with a constant value in the circuit.
    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        let field_elements = Self::elements::<L::Field>(value);
        let variables = field_elements
            .into_iter()
            .map(|element| builder.constant::<Variable>(element))
            .collect_vec();
        // Because this is a constant, we do not need to add constraints to ensure validity
        // as it is assumed that the value is valid.
        Self::from_variables_unsafe(&variables)
    }

    /// Serializes the circuit variable to variables.
    fn variables(&self) -> Vec<Variable>;

    /// Deserializes the circuit variable from variables and checks that the variable is valid.
    fn from_variables<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        variables: &[Variable],
    ) -> Self {
        let variable = Self::from_variables_unsafe(variables);
        variable.assert_is_valid(builder);
        variable
    }

    /// Deserializes the circuit variable from variables and does not check that the variable is
    /// valid.
    fn from_variables_unsafe(variables: &[Variable]) -> Self;

    /// Asserts that the variable is valid (i.e., range checks).
    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    );

    /// Gets the value of the variable from the witness.
    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let target_values = self
            .targets()
            .into_iter()
            .map(|t| witness.get_target(t))
            .collect::<Vec<F>>();
        Self::from_elements::<F>(&target_values)
    }

    /// Sets the value of the variable in the witness.
    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        let elements = Self::elements::<F>(value);
        let targets = self.targets();
        assert_eq!(elements.len(), targets.len());
        for (element, target) in elements.into_iter().zip(targets.into_iter()) {
            witness.set_target(target, element);
        }
    }

    /// Serializes the circuit variable to targets.
    fn targets(&self) -> Vec<Target> {
        self.variables().into_iter().map(|v| v.0).collect()
    }

    /// Deserializes a variable from a list of targets.  It does not do any validity checks (e.g.
    /// range checks).
    fn from_targets(targets: &[Target]) -> Self {
        Self::from_variables_unsafe(&targets.iter().map(|t| Variable(*t)).collect_vec())
    }

    /// The number of field elements it takes to represent this variable.
    fn nb_elements() -> usize;

    /// Serializes the value type to a list of field elements.
    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F>;

    /// Deserializes a list of field elements to the value type.
    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F>;
}

pub trait EvmVariable: CircuitVariable {
    /// The number of bytes it takes to represent this variable.
    fn nb_bytes<L: PlonkParameters<D>, const D: usize>() -> usize {
        let mut builder = CircuitBuilder::<L, D>::new();
        let variable = builder.init::<Self>();
        variable.encode(&mut builder).len()
    }

    /// The number of bits it takes to represent this variable.
    fn nb_bits<L: PlonkParameters<D>, const D: usize>() -> usize {
        Self::nb_bytes::<L, D>() * 8
    }

    /// Serializes the variable to a vector of byte variables with len `nb_bytes()`. This
    /// implementation should match the implementation of `abi.encodePacked(...)`.
    fn encode<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<ByteVariable>;

    /// Deserializes the variable to a vector of byte variables with len `nb_bytes()`. This
    /// implementation should match the implementation of `abi.decodePacked(...)`.
    fn decode<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        bytes: &[ByteVariable],
    ) -> Self;

    /// Serializes a value to bytes. This implementation should match the implementation of
    /// `abi.encodePacked(...)`.
    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8>;

    /// Deserializes a value from bytes. This implementation should match the implementation of
    /// `abi.decodePacked(...)`.
    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F>;

    /// Serializes the variable to little endian bits.
    fn to_le_bits<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<BoolVariable> {
        let bytes = self.encode(builder);
        let mut bytes = bytes.into_iter().flat_map(|b| b.as_be_bits()).collect_vec();
        bytes.reverse();
        bytes
    }

    /// Serializes the variable to big endian bits.
    fn to_be_bits<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Vec<BoolVariable> {
        let mut bits = self.to_le_bits(builder);
        bits.reverse();
        bits
    }
}

pub trait SSZVariable: CircuitVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_derive_struct() {
        #[derive(Debug, Clone, CircuitVariable)]
        #[value_name(MyPoint)]
        struct Point<V: CircuitVariable, U, const N: usize> {
            pub x: ArrayVariable<V, N>,
            y: U,
            z: (Variable, Variable),
        }

        type TestPoint = Point<Variable, ByteVariable, 1>;

        let mut builder = CircuitBuilder::<DefaultParameters, 2>::new();

        let point = builder.read::<TestPoint>();

        let constant_point = builder.constant::<TestPoint>(MyPoint {
            x: vec![GoldilocksField::ONE],
            y: 1u8,
            z: (GoldilocksField::ZERO, GoldilocksField::ONE),
        });

        builder.assert_is_equal(point.clone(), constant_point.clone());

        let variables = point.variables();
        let point_back = TestPoint::from_variables_unsafe(&variables);
        assert_eq!(point.variables(), point_back.variables());

        builder.write::<TestPoint>(constant_point);

        let circuit = builder.build();
        let mut input = circuit.input();
        input.write::<TestPoint>(MyPoint {
            x: vec![GoldilocksField::ONE],
            y: 1u8,
            z: (GoldilocksField::ZERO, GoldilocksField::ONE),
        });
    }

    #[test]
    fn test_value_derive_struct() {
        #[derive(Debug, Clone, CircuitVariable)]
        #[value_name(MyPoint)]
        #[value_derive(PartialEq, Eq)]
        struct Point {
            x: ArrayVariable<Variable, 2>,
            y: Variable,
        }

        let mut builder = CircuitBuilder::<DefaultParameters, 2>::new();

        type TestPoint = Point;

        let point = builder.read::<TestPoint>();

        let constant_point = builder.constant::<TestPoint>(MyPoint {
            x: vec![GoldilocksField::ONE, GoldilocksField::ZERO],
            y: GoldilocksField::ZERO,
        });

        builder.assert_is_equal(point.clone(), constant_point.clone());

        let variables = point.variables();
        let point_back = TestPoint::from_variables_unsafe(&variables);
        assert_eq!(point.variables(), point_back.variables());

        builder.write::<TestPoint>(constant_point);

        let circuit = builder.build();
        let mut input = circuit.input();
        input.write::<TestPoint>(MyPoint {
            x: vec![GoldilocksField::ONE, GoldilocksField::ZERO],
            y: GoldilocksField::ZERO,
        });
    }
}
