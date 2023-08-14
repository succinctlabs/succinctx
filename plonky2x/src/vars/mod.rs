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
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::PartitionWitness;
pub use uint32::*;
pub use variable::*;

use crate::builder::{CircuitBuilder, ExtendableField};

pub trait CircuitVariable<F: ExtendableField> {
    /// The underlying type of the variable if it were not in a circuit.
    type ValueType;

    /// Initializes the variable with no value in the circuit.
    fn init(builder: &mut CircuitBuilder<F>) -> Self;

    /// Initializes the variable with a constant value in the circuit.
    fn constant(builder: &mut CircuitBuilder<F>, value: Self::ValueType) -> Self;

    /// Gets the value of the variable from the witness.
    fn value<'a>(&self, witness: &PartitionWitness<'a, F>) -> Self::ValueType;

    /// Sets the value of the variable in the witness.
    fn set(&self, witness: &mut GeneratedValues<F>, value: Self::ValueType);
}
