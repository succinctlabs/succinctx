use anyhow::Result;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinSet;

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
    /// The handler will wait for requests and spawns a new task for each request. Awaiting this
    /// this method will return `Ok(())` when all tasks have finished, or `Err` if any task fails.
    pub async fn run(&mut self) -> Result<()> {
        // Initialize a join set to spawn tasks for each hint request.
        let mut set = JoinSet::new();

        // Wait for requests and spawn a new task for each request.
        while let Some(message) = self.rx.recv().await {
            let HintInMessage { hint, tx, inputs } = message;

            set.spawn(async move {
                let outputs = hint.hint_fn(inputs).await;
                tx.send(outputs)
            });
        }

        // Wait for all tasks to finish, and return on the first error if there is one.
        while let Some(result) = set.join_next().await {
            result??;
        }
        Ok(())
    }
}
