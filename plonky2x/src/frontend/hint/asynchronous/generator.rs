use core::fmt::Debug;

use anyhow::Result;
use log::trace;
use plonky2::iop::generator::{GeneratedValues, WitnessGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::util::serialization::IoError;
use tokio::sync::mpsc::UnboundedSender;

use super::channel::{HintChannel, HintInMessage};
use super::hint::{AnyAsyncHint, AnyHint, AsyncHint};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::hint::HintGenerator;
use crate::frontend::vars::{ValueStream, VariableStream};
use crate::prelude::{CircuitVariable, Variable};
use crate::utils::serde::BufferWrite;

/// The result of running an asynchronous hint.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HintPoll {
    /// The hint is waiting for all input variables to be set.
    InputPending,
    /// The hint is still running.
    Pending,
    /// The hint has finished and all output variables have been set.
    Ready,
}

pub trait AsyncGeneratorData<L: PlonkParameters<D>, const D: usize>: HintGenerator<L, D> {
    fn generator(&self, tx: UnboundedSender<HintInMessage<L, D>>) -> AsyncHintRef<L, D>;
}

pub trait AsyncHintRunner<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Send + Sync
{
    fn watch_list(&self) -> &[Variable];

    fn run(
        &mut self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) -> HintPoll;
}

#[derive(Debug)]
pub struct AsyncHintRef<L, const D: usize>(pub Box<dyn AsyncHintRunner<L, D>>);

impl<L: PlonkParameters<D>, const D: usize> AsyncHintRef<L, D> {
    pub fn new<H: AsyncHintRunner<L, D>>(hint: H) -> Self {
        Self(Box::new(hint))
    }

    pub(crate) fn id(hint_id: String) -> String {
        format!("--async hint: {:?}", hint_id).to_string()
    }
}

#[derive(Debug)]
pub struct AsyncHintDataRef<L: PlonkParameters<D>, const D: usize>(
    pub(crate) Box<dyn AsyncGeneratorData<L, D>>,
);

impl<L: PlonkParameters<D>, const D: usize> AsyncHintDataRef<L, D> {
    pub(crate) fn new<H: AsyncHint<L, D>>(generator_data: AsyncHintData<L, H, D>) -> Self {
        Self(Box::new(generator_data))
    }
}

/// The wintess generator for asynchronous hints.
#[derive(Debug)]
pub(crate) struct AsyncHintGenerator<L: PlonkParameters<D>, H, const D: usize> {
    pub(crate) hint: H,
    pub(crate) tx: UnboundedSender<HintInMessage<L, D>>,
    pub(crate) channel: HintChannel<L, D>,
    pub(crate) input_stream: VariableStream,
    pub(crate) output_stream: VariableStream,
    pub(crate) state: HintPoll,
}

/// A dummy witness generator containing the hint data and input/output streams.
///
/// This struct is used to register the hint in the dependency graph and to create
/// an `AsyncHintGenerator` during witness generation.
#[derive(Debug, Clone)]
pub(crate) struct AsyncHintData<L, H, const D: usize> {
    pub(crate) hint: H,
    pub(crate) input_stream: VariableStream,
    pub(crate) output_stream: VariableStream,
    _marker: std::marker::PhantomData<L>,
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> HintGenerator<L, D>
    for AsyncHintData<L, H, D>
{
    fn output_stream_mut(&mut self) -> &mut VariableStream {
        &mut self.output_stream
    }
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> AsyncGeneratorData<L, D>
    for AsyncHintData<L, H, D>
{
    fn generator(&self, tx: UnboundedSender<HintInMessage<L, D>>) -> AsyncHintRef<L, D> {
        AsyncHintRef::new(self.generator(tx))
    }
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> AsyncHintData<L, H, D> {
    pub fn new(hint: H, input_stream: VariableStream, output_stream: VariableStream) -> Self {
        Self {
            hint,
            input_stream,
            output_stream,
            _marker: std::marker::PhantomData,
        }
    }

    fn generator(&self, tx: UnboundedSender<HintInMessage<L, D>>) -> AsyncHintGenerator<L, H, D> {
        AsyncHintGenerator::new(
            self.input_stream.clone(),
            self.output_stream.clone(),
            self.hint.clone(),
            tx,
            HintChannel::new(),
        )
    }
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> AsyncHintGenerator<L, H, D> {
    pub fn new(
        input_stream: VariableStream,
        output_stream: VariableStream,
        hint: H,
        tx: UnboundedSender<HintInMessage<L, D>>,
        channel: HintChannel<L, D>,
    ) -> Self {
        Self {
            input_stream,
            output_stream,
            hint,
            tx,
            channel,
            state: HintPoll::InputPending,
        }
    }

    /// send the input to the hint handler.
    pub fn send(&self, input_stream: ValueStream<L, D>) -> Result<()> {
        let hint: Box<dyn AnyAsyncHint<L, D>> = Box::new(AnyHint(self.hint.clone()));

        let message = HintInMessage {
            hint,
            tx: self.channel.tx_out.clone(),
            inputs: input_stream,
        };

        self.tx.send(message)?;

        Ok(())
    }
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> AsyncHintRunner<L, D>
    for AsyncHintGenerator<L, H, D>
{
    fn watch_list(&self) -> &[Variable] {
        self.input_stream.real_all()
    }

    fn run(
        &mut self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) -> HintPoll {
        match self.state {
            HintPoll::InputPending => {
                // Check if all input variables are set, otherwise return.
                if !self.watch_list().iter().all(|v| witness.contains(v.0)) {
                    return HintPoll::InputPending;
                }
                // Send the input to the hint.
                trace!("Async Hint {:?} : Sending input to hint", H::id());
                let input_values = self
                    .input_stream
                    .real_all()
                    .iter()
                    .map(|v| v.get(witness))
                    .collect::<Vec<_>>();

                let input_stream = ValueStream::<L, D>::from_values(input_values);

                self.send(input_stream).unwrap();

                // Update and return the state.
                self.state = HintPoll::Pending;
                HintPoll::Pending
            }
            HintPoll::Pending => {
                // Check the hint channel for the output. If not ready, return `HintPoll::Pending`.
                if let Ok(mut output_stream) = self.channel.rx_out.try_recv() {
                    trace!("Async Hint {:?} : recieved output from hint", H::id());
                    let output_values = output_stream.read_all();
                    let output_vars = self.output_stream.real_all();
                    assert_eq!(output_values.len(), output_vars.len());

                    for (var, val) in output_vars.iter().zip(output_values) {
                        var.set(out_buffer, *val)
                    }
                    return HintPoll::Ready;
                }
                HintPoll::Pending
            }
            HintPoll::Ready => HintPoll::Ready,
        }
    }
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> WitnessGenerator<L::Field, D>
    for AsyncHintData<L, H, D>
{
    fn id(&self) -> String {
        AsyncHintRef::<L, D>::id(H::id())
    }

    fn watch_list(&self) -> Vec<Target> {
        self.input_stream.real_all().iter().map(|v| v.0).collect()
    }

    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<L::Field, D>,
    ) -> plonky2::util::serialization::IoResult<()> {
        self.input_stream.serialize_to_writer(dst)?;
        self.output_stream.serialize_to_writer(dst)?;

        let bytes = bincode::serialize(&self.hint).map_err(|_| IoError)?;
        dst.write_bytes(&bytes)
    }

    fn deserialize(
        _src: &mut plonky2::util::serialization::Buffer,
        _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<L::Field, D>,
    ) -> plonky2::util::serialization::IoResult<Self>
    where
        Self: Sized,
    {
        unimplemented!("Hints are not deserializable through the plonky2 crate, only directly through the witness registry")
    }

    fn run(
        &self,
        _witness: &PartitionWitness<L::Field>,
        _out_buffer: &mut GeneratedValues<L::Field>,
    ) -> bool {
        true
    }
}
