mod boolean;
mod byte;
mod bytes;
mod bytes32;

mod variable;

use std::fmt::Debug;

pub use boolean::*;
pub use byte::*;
pub use bytes::*;
pub use bytes32::*;
use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};
pub use variable::*;

pub use super::uint::uint256::*;
pub use super::uint::uint32::*;
use crate::builder::CircuitBuilder;

/// A trait enforcing that the a type can be serialized to a list of field elements. This trait
/// is necessary for translating values between the circuit and the outside context.
pub trait FieldSerializable<F: RichField> {
    /// Returns the number of field elements that this value needs to be represented in the circuit.
    fn nb_elements() -> usize;

    /// Serializes the value to a list of field elements.
    fn elements(&self) -> Vec<F>;

    /// Deserializes the value from a list of field elements.
    fn from_elements(elements: &[F]) -> Self;
}

pub trait CircuitVariable: Debug + Clone + Sized + Sync + Send + 'static {
    /// The underlying type of the variable if it were not in a circuit.
    type ValueType<F: RichField>: FieldSerializable<F>;

    /// Initializes the variable with no value in the circuit.
    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self;

    /// Initializes the variable with a constant value in the circuit.
    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        let variable = Self::init(builder);
        let targets = variable.targets();
        let elements = value.elements();
        assert_eq!(targets.len(), elements.len());
        for i in 0..targets.len() {
            let constant = builder.api.constant(elements[i]);
            builder.api.connect(targets[i], constant);
        }
        variable
    }

    /// Serializes the circuit variable to variables.
    fn variables(&self) -> Vec<Variable> {
        self.targets().into_iter().map(Variable).collect()
    }

    /// Deserializes the circuit variable from variables.
    fn from_variables(variables: &[Variable]) -> Self {
        Self::from_targets(&variables.iter().map(|v| v.0).collect_vec())
    }

    /// Serializes the circuit variable to targets.
    fn targets(&self) -> Vec<Target>;

    /// Deserializes a variable from a list of targets.
    fn from_targets(targets: &[Target]) -> Self;

    /// Gets the value of the variable from the witness.
    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F>;

    /// Sets the value of the variable in the witness.
    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>);
}

/// A trait enforcing that the a type can be serialized to a list of byte variables. This trait
/// is necessary for translating values between the circuit and often abi-encoded values.
pub trait ByteSerializable<F: RichField> {
    fn nb_bytes() -> usize;

    fn bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait EvmVariable: CircuitVariable {
    type ValueType<F: RichField>: ByteSerializable<F>;

    /// Serializes the variable to a vector of byte variables with len `nb_bytes()`.
    fn bytes<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable>;

    /// Deserializes the variable to a vector of byte variables with len `nb_bytes()`.
    fn from_bytes<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[ByteVariable],
    ) -> Self;
}
