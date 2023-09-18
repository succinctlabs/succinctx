use anyhow::Result;
use tokio::sync::mpsc::UnboundedReceiver;

use super::channel::HintInMessage;
use crate::prelude::PlonkParameters;

#[derive(Debug)]
pub struct HintHandler<L: PlonkParameters<D>, const D: usize> {
    rx: UnboundedReceiver<HintInMessage<L, D>>,
}

impl<L: PlonkParameters<D>, const D: usize> HintHandler<L, D> {

    pub fn new(rx: UnboundedReceiver<HintInMessage<L, D>>) -> Self {
        Self { rx }
    }

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
