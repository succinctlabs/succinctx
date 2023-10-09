use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use super::hint::AnyAsyncHint;
use crate::frontend::vars::ValueStream;
use crate::prelude::PlonkParameters;

/// A channel for sending and receiving output data from the hint handler.
#[derive(Debug)]
pub struct HintChannel<L: PlonkParameters<D>, const D: usize> {
    pub(crate) rx_out: UnboundedReceiver<ValueStream<L, D>>,
    pub(crate) tx_out: UnboundedSender<ValueStream<L, D>>,
}

/// A message sent to the hint handler.
#[derive(Debug)]
pub struct HintInMessage<L: PlonkParameters<D>, const D: usize> {
    pub(crate) hint: Box<dyn AnyAsyncHint<L, D>>,
    pub(crate) tx: UnboundedSender<ValueStream<L, D>>,
    pub(crate) inputs: ValueStream<L, D>,
}

impl<L: PlonkParameters<D>, const D: usize> HintChannel<L, D> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx_out, rx_out) = unbounded_channel();
        Self { tx_out, rx_out }
    }
}
