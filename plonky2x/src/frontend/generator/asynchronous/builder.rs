use super::generator::{AsyncGeneratorRef, AsyncHintData};
use super::hint::AsyncHint;
use crate::frontend::vars::{OutputVariableStream, VariableStream};
use crate::prelude::{CircuitBuilder, PlonkParameters};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn async_hint<H: AsyncHint<L, D>>(
        &mut self,
        input_stream: VariableStream,
        hint: H,
    ) -> OutputVariableStream<L, D> {
        let output_stream = VariableStream::new();
        let generator = AsyncHintData::new(hint, input_stream, output_stream.clone());
        let hint_id = self.hints.len();
        self.hints.push(Box::new(generator.clone()));

        self.async_generators
            .push(AsyncGeneratorRef::new(generator));
        self.async_generators_indices.push(hint_id);

        OutputVariableStream::new(hint_id)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use tokio::time::{sleep, Duration};

    use super::*;
    use crate::frontend::vars::ValueStream;
    use crate::prelude::{ByteVariable, DefaultBuilder};
    use crate::utils::setup_logger;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TestAsyncGenerator;

    #[async_trait]
    impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for TestAsyncGenerator {
        async fn hint(
            &self,
            input_stream: &mut ValueStream<L, D>,
            output_stream: &mut ValueStream<L, D>,
        ) {
            let time = input_stream.read_value::<ByteVariable>();
            sleep(Duration::from_millis(time.into())).await;
            output_stream.write_value::<ByteVariable>(time);
        }
    }

    #[test]
    fn test_async_hint() {
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let time = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&time);

        let hint = TestAsyncGenerator {};
        let output_stream = builder.async_hint(input_stream, hint);
        let back_time = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(back_time);

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
        assert_eq!(byte_plus_one, 5u8);
    }
}
