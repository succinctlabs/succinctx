use std::sync::Mutex;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use super::hint::AnyAsyncHint;
use crate::frontend::vars::ValueStream;
use crate::prelude::PlonkParameters;


#[derive(Debug)]
pub struct HintChannel<L: PlonkParameters<D>, const D: usize> {
    pub(crate) rx_out: Mutex<UnboundedReceiver<ValueStream<L, D>>>,
    pub(crate) tx_out: UnboundedSender<ValueStream<L, D>>,
}

#[derive(Debug)]
pub struct HintInMessage<L: PlonkParameters<D>, const D: usize> {
    pub(crate) hint: Box<dyn AnyAsyncHint<L, D>>,
    pub(crate) tx: UnboundedSender<ValueStream<L, D>>,
    pub(crate) inputs: ValueStream<L, D>,
}

#[derive(Clone, Debug)]
pub struct HintOutMessage<L: PlonkParameters<D>, const D: usize> {
    outputs: ValueStream<L, D>,
}

impl<L: PlonkParameters<D>, const D: usize> HintOutMessage<L, D> {
    pub fn new(outputs: ValueStream<L, D>) -> Self {
        Self { outputs }
    }
}

impl<L: PlonkParameters<D>, const D: usize> HintChannel<L, D> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx_out, rx_out) = unbounded_channel();
        Self {
            tx_out,
            rx_out: Mutex::new(rx_out),
        }
    }
}
