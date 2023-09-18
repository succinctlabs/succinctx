use core::fmt::Debug;
use core::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use plonky2::iop::generator::{GeneratedValues, WitnessGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use tokio::sync::mpsc::UnboundedSender;

use super::channel::{HintChannel, HintInMessage};
use super::hint::{AnyAsyncHint, AnyHint, AsyncHint};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::{ValueStream, VariableStream};
use crate::prelude::CircuitVariable;

#[derive(Debug)]
pub struct AsyncHintGenerator<L: PlonkParameters<D>, H, const D: usize> {
    pub(crate) hint: H,
    pub(crate) tx: UnboundedSender<HintInMessage<L, D>>,
    pub(crate) channel: HintChannel<L, D>,
    pub(crate) input_stream: VariableStream,
    pub(crate) output_stream: VariableStream,
    pub(crate) waiting: AtomicBool,
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
            waiting: AtomicBool::new(false),
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

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> WitnessGenerator<L::Field, D>
    for AsyncHintGenerator<L, H, D>
{
    fn id(&self) -> String {
        H::id()
    }

    fn watch_list(&self) -> Vec<Target> {
        self.input_stream.real_all().iter().map(|v| v.0).collect()
    }

    fn serialize(
        &self,
        _dst: &mut Vec<u8>,
        _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<L::Field, D>,
    ) -> plonky2::util::serialization::IoResult<()> {
        unimplemented!("AsyncHintGenerator::serialize")
    }

    fn deserialize(
        _src: &mut plonky2::util::serialization::Buffer,
        _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<L::Field, D>,
    ) -> plonky2::util::serialization::IoResult<Self>
    where
        Self: Sized,
    {
        unimplemented!("AsyncHintGenerator::deserialize")
    }

    fn run(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) -> bool {
        // check if all the inputs has been set.
        if !witness.contains_all(&self.watch_list()) {
            return false;
        }

        // check if the hint is already waiting for output.
        let waiting = self.waiting.load(Ordering::Relaxed);

        // If the hint is waiting, try to receive the output.
        if waiting {
            let mut rx_out = self.channel.rx_out.lock().unwrap();
            if let Ok(mut output_stream) = rx_out.try_recv() {
                let output_values = output_stream.read_all();
                let output_vars = self.output_stream.real_all();
                assert_eq!(output_values.len(), output_vars.len());

                for (var, val) in output_vars.iter().zip(output_values) {
                    var.set(out_buffer, *val)
                }
                assert_eq!(output_values.len(), output_vars.len());

                for (var, val) in output_vars.iter().zip(output_values) {
                    var.set(out_buffer, *val)
                }
                return true;
            }
            false
        }
        // if the hint is not waiting, send the input and update the waiting flag.
        else {
            let input_values = self
                .input_stream
                .real_all()
                .iter()
                .map(|v| v.get(witness))
                .collect::<Vec<_>>();

            let input_stream = ValueStream::<L, D>::from_values(input_values);

            self.send(input_stream).unwrap();

            // update the waiting flag to `true`.
            self.waiting.store(true, Ordering::Relaxed);

            false
        }
    }
}
