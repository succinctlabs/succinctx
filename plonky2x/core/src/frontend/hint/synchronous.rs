use core::fmt::Debug;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use starkyx::maybe_rayon::rayon;
use tokio::sync::oneshot;

use super::asynchronous::hint::AsyncHint;
use super::simple::hint::Hint;
use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::ValueStream;

/// An adapter to perform a hint asynchronously.
///
/// This is useful for hints that are expensive to compute, and can be computed without blocking
/// other parts of witness generation that don't depend on the hint's output.
///
/// This can be used when building a circuit as follows:
/// ```ignore
/// let output_stream = builder.async_hint(input_stream, Async(hint));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Async<H>(pub H);

#[async_trait]
impl<L: PlonkParameters<D>, H: Hint<L, D>, const D: usize> AsyncHint<L, D> for Async<H> {
    async fn hint(
        &self,
        _input_stream: &mut ValueStream<L, D>,
        _output_stream: &mut ValueStream<L, D>,
    ) {
        unimplemented!("Sync hints implement owned function hint_fn instead of hint")
    }

    /// A version of the hint function that owns the input stream and returns the output stream.
    ///
    /// Only one of `hint` or `hint_fn` needs to be implemented. By default, `hint_fn` calls `hint`.
    async fn hint_fn(&self, input_stream: ValueStream<L, D>) -> ValueStream<L, D> {
        let (tx, rx) = oneshot::channel();
        let hint = self.0.clone();
        rayon::spawn(move || {
            let mut input_stream = input_stream;
            let mut output_stream = ValueStream::new();
            hint.hint(&mut input_stream, &mut output_stream);
            tx.send(output_stream).unwrap();
        });

        rx.await.unwrap()
    }
}

#[cfg(test)]
mod tests {

    use core::time::Duration;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::prelude::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct AddSomeSleep {
        amount: u8,
        sleep: Duration,
    }

    impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for AddSomeSleep {
        fn hint(
            &self,
            input_stream: &mut ValueStream<L, D>,
            output_stream: &mut ValueStream<L, D>,
        ) {
            let a = input_stream.read_value::<ByteVariable>();
            std::thread::sleep(self.sleep);
            output_stream.write_value::<ByteVariable>(a + self.amount)
        }
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sync_hint() {
        let mut builder = DefaultBuilder::new();

        let a = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a);

        let hint = AddSomeSleep {
            amount: 2,
            sleep: Duration::from_secs(1),
        };
        let output_stream = builder.async_hint(input_stream.clone(), Async(hint.clone()));
        let b = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(b);

        // runing a few in parallel to make sure the tests takes roughly the duratio of one sleep.
        let output_stream = builder.async_hint(input_stream.clone(), Async(hint.clone()));
        let _ = output_stream.read::<ByteVariable>(&mut builder);
        let output_stream = builder.async_hint(input_stream.clone(), Async(hint.clone()));
        let _ = output_stream.read::<ByteVariable>(&mut builder);
        let output_stream = builder.async_hint(input_stream.clone(), Async(hint));
        let _ = output_stream.read::<ByteVariable>(&mut builder);

        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<ByteVariable>(5u8);

        // Generate a proof.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let byte_plus_one = output.read::<ByteVariable>();
        assert_eq!(byte_plus_one, 7u8);
    }
}
