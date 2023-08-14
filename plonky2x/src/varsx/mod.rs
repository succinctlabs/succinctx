mod boolean;
mod byte;
mod bytes;
mod uint256;
mod uint32;
mod variable;

pub use boolean::*;
pub use byte::*;
pub use bytes::*;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::PartitionWitness;
pub use uint32::*;
pub use variable::*;

use crate::builder::CircuitBuilder;

pub trait CircuitVariable {
    /// The underlying type of the variable if it were not in a circuit.
    type Value;

    /// Initializes the variable with no value in the circuit.
    fn init(builder: &mut CircuitBuilder) -> Self;

    /// Initializes the variable with a constant value in the circuit.
    fn constant(builder: &mut CircuitBuilder, value: Self::Value) -> Self;

    /// Gets the value of the variable from the witness.
    fn value<'a>(&self, witness: &PartitionWitness<'a, GoldilocksField>) -> Self::Value;

    /// Sets the value of the variable in the witness.
    fn set(&self, witness: &mut GeneratedValues<GoldilocksField>, value: Self::Value);
}

impl CircuitBuilder {
    /// Initializes a variable with no value in the circuit.
    pub fn init<V: CircuitVariable>(&mut self) -> V {
        V::init(self)
    }

    /// Initializes a variable with a constant value in the circuit.
    pub fn constant<V: CircuitVariable>(&mut self, value: V::Value) -> V {
        V::constant(self, value)
    }
}
