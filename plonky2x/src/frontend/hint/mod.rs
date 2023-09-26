use plonky2::iop::generator::WitnessGenerator;

use super::vars::VariableStream;
use crate::prelude::PlonkParameters;

pub mod asynchronous;
pub mod simple;
pub mod synchronous;

pub trait HintGenerator<L: PlonkParameters<D>, const D: usize>:
    WitnessGenerator<L::Field, D>
{
    /// returns a mutable reference to the output stream.
    fn output_stream_mut(&mut self) -> &mut VariableStream;
}
