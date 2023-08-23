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
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};
pub use variable::*;

pub use super::uint::uint256::*;
pub use super::uint::uint32::*;
use crate::builder::CircuitBuilder;

pub trait CircuitVariable: Debug + Clone + Sized + Send + Sync {
    /// The underlying type of the variable if it were not in a circuit.
    type ValueType<F>;

    /// Initializes the variable with no value in the circuit.
    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self;

    /// Initializes the variable with a constant value in the circuit.
    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self;

    /// Returns the underlying targets used by the variable.
    fn targets(&self) -> Vec<Target>;

    /// Deserializes a variable from a list of targets.
    fn from_targets(targets: &[Target]) -> Self;

    /// Gets the value of the variable from the witness.
    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F>;

    /// Sets the value of the variable in the witness.
    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>);
}
