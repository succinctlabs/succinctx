use super::vars::VariableStream;
use crate::prelude::{CircuitBuilder, PlonkParameters};

pub mod asynchronous;
pub mod simple;
pub mod synchronous;

pub trait HintRef<L: PlonkParameters<D>, const D: usize> {
    /// returns a mutable reference to the output stream.
    fn output_stream_mut(&mut self) -> &mut VariableStream;

    /// adds the hint type to the circuit builder.
    fn register(&self, builder: &mut CircuitBuilder<L, D>);
}
