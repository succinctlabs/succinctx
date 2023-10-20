use super::generator::{AsyncHintData, AsyncHintDataRef};
use super::hint::AsyncHint;
use crate::frontend::vars::{OutputVariableStream, VariableStream};
use crate::prelude::{CircuitBuilder, PlonkParameters};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Get the outputs of an asynchronous hint to the circuit.
    pub fn async_hint<H: AsyncHint<L, D>>(
        &mut self,
        input_stream: VariableStream,
        hint: H,
    ) -> OutputVariableStream<L, D> {
        let output_stream = VariableStream::new();
        let hint_data = AsyncHintData::new(hint, input_stream, output_stream.clone());
        let hint_id = self.hints.len();
        self.hints.push(Box::new(hint_data.clone()));

        self.async_hints.push(AsyncHintDataRef::new(hint_data));
        self.async_hints_indices.push(hint_id);

        OutputVariableStream::new(hint_id)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use tokio::time::{sleep, Duration};

    use super::*;
    use crate::backend::circuit::CircuitBuild;
    use crate::frontend::vars::ValueStream;
    use crate::prelude::{
        ByteVariable, DefaultBuilder, DefaultParameters, GateRegistry, HintRegistry,
    };
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
            if time == 20 {
                panic!("Test panic, immediate failure");
            }
            sleep(Duration::from_secs(time.into())).await;
            if time == 10 {
                panic!("Test panic, delayed failure");
            }
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

        let gate_serializer = GateRegistry::<DefaultParameters, 2>::new();
        let mut hint_serializer = HintRegistry::new();
        hint_serializer.register_async_hint::<TestAsyncGenerator>();

        let circuit_bytes = circuit
            .serialize(&gate_serializer, &hint_serializer)
            .unwrap();
        let circuit = CircuitBuild::<DefaultParameters, 2>::deserialize(
            &circuit_bytes,
            &gate_serializer,
            &hint_serializer,
        )
        .unwrap();

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

    #[test]
    #[should_panic]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_failure_async_hint() {
        setup_logger();
        let mut builder = DefaultBuilder::new();

        let time = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&time);

        let hint = TestAsyncGenerator {};
        let output_stream = builder.async_hint(input_stream.clone(), hint.clone());
        let back_time = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(back_time);

        // running a few in parallel to make sure the tests takes roughly the duratio of one sleep.
        let mut fail_input_stream = VariableStream::new();
        let fail_time = builder.constant::<ByteVariable>(10u8);
        fail_input_stream.write(&fail_time);
        let output_stream = builder.async_hint(fail_input_stream, hint.clone());
        let _ = output_stream.read::<ByteVariable>(&mut builder);
        let output_stream = builder.async_hint(input_stream.clone(), hint.clone());
        let _ = output_stream.read::<ByteVariable>(&mut builder);
        let output_stream = builder.async_hint(input_stream, hint);
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
        assert_eq!(byte_plus_one, 5u8);
    }
}
