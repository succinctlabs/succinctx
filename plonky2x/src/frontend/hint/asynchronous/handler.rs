use anyhow::Result;
use tokio::sync::mpsc::UnboundedReceiver;

use super::channel::HintInMessage;
use crate::prelude::PlonkParameters;


/// A handler for asynchronous hints.
/// 
/// This handler is responsible for receiving hint requests, computing the hint, and sending the
/// result back to the prover. 
#[derive(Debug)]
pub struct HintHandler<L: PlonkParameters<D>, const D: usize> {
    rx: UnboundedReceiver<HintInMessage<L, D>>,
}

impl<L: PlonkParameters<D>, const D: usize> HintHandler<L, D> {
    pub fn new(rx: UnboundedReceiver<HintInMessage<L, D>>) -> Self {
        Self { rx }
    }

    /// Run the handler.
    /// 
    /// The handler will wait for requests and spawns a new task for each request. This method will 
    /// block finish when channel is closed.
    pub async fn run(&mut self) -> Result<()> {
        while let Some(message) = self.rx.recv().await {
            let HintInMessage { hint, tx, inputs } = message;

            tokio::spawn(async move {
                let outputs = hint.hint_fn(inputs).await;
                tx.send(outputs).unwrap();
            });
        }
        Ok(())
    }
}
