use core::fmt::Debug;
use core::marker::PhantomData;

use plonky2::iop::generator::{SimpleGenerator, WitnessGeneratorRef};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use super::general::{HintGenerator, HintSerializer, HintSimpleGenerator};
use crate::backend::circuit::{PlonkParameters, Serializer};
use crate::frontend::vars::{OutputVariableStream, ValueStream, VariableStream};
use crate::prelude::CircuitBuilder;

#[derive(Debug, Clone)]
pub struct HintFn<L: PlonkParameters<D>, const D: usize>(
    pub fn(&mut ValueStream<L, D>, &mut ValueStream<L, D>),
);

impl<L: PlonkParameters<D>, const D: usize> HintGenerator<L, D> for HintFn<L, D> {
    type Serializer = PureHintSerializer<L, D>;

    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        self.0(input_stream, output_stream)
    }

    fn serializer(&self) -> Self::Serializer {
        PureHintSerializer::new(self.0)
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn hint_fn(
        &mut self,
        input_stream: VariableStream,
        hint_fn: fn(&mut ValueStream<L, D>, &mut ValueStream<L, D>),
    ) -> OutputVariableStream<L, D> {
        self.hint_generator(input_stream, HintFn(hint_fn))
    }
}

#[derive(Debug, Clone)]
pub struct PureHintSerializer<L: PlonkParameters<D>, const D: usize> {
    pub hint: fn(&mut ValueStream<L, D>, &mut ValueStream<L, D>),
    _marker: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> PureHintSerializer<L, D> {
    pub fn new(hint: fn(&mut ValueStream<L, D>, &mut ValueStream<L, D>)) -> Self {
        Self {
            hint,
            _marker: PhantomData,
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize>
    Serializer<L::Field, WitnessGeneratorRef<L::Field, D>, D> for PureHintSerializer<L, D>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<WitnessGeneratorRef<L::Field, D>> {
        let input_stream = VariableStream::deserialize_from_reader(buf)?;
        let output_stream = VariableStream::deserialize_from_reader(buf)?;

        let hint = HintFn(self.hint);
        let hint_generator =
            HintSimpleGenerator::<L, HintFn<L, D>>::new(input_stream, output_stream, hint);

        Ok(WitnessGeneratorRef::new(hint_generator.adapter()))
    }

    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &WitnessGeneratorRef<L::Field, D>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        object.0.serialize(buf, common_data)
    }
}

impl<L: PlonkParameters<D>, const D: usize> HintSerializer<L, D> for PureHintSerializer<L, D> {
    fn id(&self) -> String {
        format!("--pure fn hint: {:?}", self.hint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    fn plus_one<L: PlonkParameters<D>, const D: usize>(
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let byte: u8 = input_stream.read_value::<ByteVariable>();

        output_stream.write_value::<ByteVariable>(byte + 1)
    }

    #[test]
    fn test_function_hint() {
        let mut builder = DefaultBuilder::new();

        let a = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a);

        let output_stream = builder.hint_fn(input_stream, plus_one);
        let b = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(b);

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
        assert_eq!(byte_plus_one, 6u8);

        // Test the serialization
        let gate_serializer = GateRegistry::new();
        let mut generator_serializer = WitnessGeneratorRegistry::new();
        generator_serializer.register_hint_function(plus_one);
        circuit.test_serializers(&gate_serializer, &generator_serializer);
    }
}
