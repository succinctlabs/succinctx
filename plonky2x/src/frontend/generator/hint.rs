use core::any::TypeId;
use core::fmt::Debug;
use core::marker::PhantomData;

use plonky2::iop::generator::{SimpleGenerator, WitnessGeneratorRef};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoError, IoResult};
use serde::de::DeserializeOwned;

use super::general::{HintGenerator, HintSerializer, HintSimpleGenerator};
use crate::backend::circuit::{PlonkParameters, Serializer};
use crate::frontend::vars::{OutputVariableStream, ValueStream, VariableStream};
use crate::prelude::CircuitBuilder;

pub trait Hint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync + serde::Serialize + DeserializeOwned
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>);
}

#[derive(Debug, Clone)]
pub struct StatefulHint<H>(pub H);

#[derive(Debug, Clone)]
pub struct SateHintSerializer<L, H>(PhantomData<L>, PhantomData<H>);

impl<L, H> SateHintSerializer<L, H> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<L, H> Default for SateHintSerializer<L, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn hint<H: Hint<L, D>>(
        &mut self,
        input_stream: VariableStream,
        hint: H,
    ) -> OutputVariableStream<L, D> {
        self.hint_generator(input_stream, StatefulHint(hint))
    }
}

impl<L: PlonkParameters<D>, H: Hint<L, D>, const D: usize> HintGenerator<L, D> for StatefulHint<H> {
    type Serializer = SateHintSerializer<L, H>;

    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        self.0.hint(input_stream, output_stream)
    }

    fn serializer(&self) -> Self::Serializer {
        Self::Serializer::new()
    }
}

impl<L: PlonkParameters<D>, H: Hint<L, D>, const D: usize>
    Serializer<L::Field, WitnessGeneratorRef<L::Field, D>, D> for SateHintSerializer<L, H>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<WitnessGeneratorRef<L::Field, D>> {
        let input_stream = VariableStream::deserialize_from_reader(buf)?;
        let output_stream = VariableStream::deserialize_from_reader(buf)?;

        let hint: H = bincode::deserialize(buf.bytes()).map_err(|_| IoError)?;
        let hint_generator = HintSimpleGenerator::<L, StatefulHint<H>>::new(
            input_stream,
            output_stream,
            StatefulHint(hint),
        );

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

impl<L: PlonkParameters<D>, H: Hint<L, D>, const D: usize> HintSerializer<L, D>
    for SateHintSerializer<L, H>
{
    fn id(&self) -> String {
        format!("--pure fn hint: {:?}", TypeId::of::<H>())
    }
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::prelude::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct AddSome {
        amount: u8,
    }

    impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for AddSome {
        fn hint(
            &self,
            input_stream: &mut ValueStream<L, D>,
            output_stream: &mut ValueStream<L, D>,
        ) {
            let a = input_stream.read_value::<ByteVariable>();

            output_stream.write_value::<ByteVariable>(a + self.amount)
        }
    }

    #[test]
    fn test_hint() {
        let mut builder = DefaultBuilder::new();

        let a = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a);

        let hint = AddSome { amount: 1 };
        let output_stream = builder.hint(input_stream, hint);
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
        generator_serializer.register_hint::<AddSome>();
        circuit.test_serializers(&gate_serializer, &generator_serializer);
    }
}
