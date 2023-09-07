use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::CircuitVariable;

pub mod uint128;
pub mod uint256;
pub mod uint32;
pub mod uint64;

mod uint32_n;

/// A variable in the circuit representing an algebraic value.
///
/// It has a zero value, a one value, and can be added, subtracted, and multiplied.
pub trait AlgebraicVariable: CircuitVariable {
    /// Returns the zero value of the variable.
    fn zero<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self;

    /// Returns the one value of the variable.
    fn one<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self;

    // Adds two variables together.
    fn add<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        other: &Self,
    ) -> Self;

    // Subtracts two variables.
    fn sub<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        other: &Self,
    ) -> Self;

    // Multiplies two variables.
    fn mul<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        other: &Self,
    ) -> Self;

    // Negates a variable.
    fn neg<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self;
}

pub trait UintVariable: AlgebraicVariable {}
