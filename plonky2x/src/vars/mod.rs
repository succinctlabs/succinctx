mod boolean;
mod byte;
mod bytes;
mod bytes32;
mod uint256;
mod uint32;
mod variable;

pub use boolean::*;
pub use byte::*;
pub use bytes::*;
pub use bytes32::*;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};
pub use uint256::*;
pub use uint32::*;
pub use variable::*;

use crate::builder::CircuitBuilder;

pub trait CircuitVariable {
    /// The underlying type of the variable if it were not in a circuit.
    type ValueType;

    /// Initializes the variable with no value in the circuit.
    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self;

    /// Initializes the variable with a constant value in the circuit.
    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self;

    /// Returns the underlying targets used by the variable.
    fn targets(&self) -> Vec<Target>;

    /// Gets the value of the variable from the witness.
    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType;

    /// Sets the value of the variable in the witness.
    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType);
}
