use core::marker::PhantomData;

use plonky2::iop::generator::{SimpleGenerator, WitnessGeneratorRef};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoError, IoResult};

use super::generator::HintSimpleGenerator;
use super::hint::Hint;
use crate::backend::circuit::Serializer;
use crate::frontend::vars::VariableStream;
use crate::prelude::PlonkParameters;
use crate::utils::serde::BufferRead;

#[derive(Debug, Clone)]
pub struct SimpleHintSerializer<L, H>(PhantomData<L>, PhantomData<H>);

impl<L, H> SimpleHintSerializer<L, H> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<L, H> Default for SimpleHintSerializer<L, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: PlonkParameters<D>, H: Hint<L, D>, const D: usize>
    Serializer<L::Field, WitnessGeneratorRef<L::Field, D>, D> for SimpleHintSerializer<L, H>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<WitnessGeneratorRef<L::Field, D>> {
        let input_stream = VariableStream::deserialize_from_reader(buf)?;
        let output_stream = VariableStream::deserialize_from_reader(buf)?;

        let bytes = buf.read_bytes()?;
        let hint: H = bincode::deserialize(&bytes).map_err(|_| IoError)?;
        let hint_generator = HintSimpleGenerator::<L, H>::new(input_stream, output_stream, hint);

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

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::backend::circuit::CircuitBuild;
    use crate::frontend::vars::ValueStream;
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
    fn test_hint_serialization() {
        let mut builder = DefaultBuilder::new();

        let a = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a);

        let hint = AddSome { amount: 2 };
        let output_stream = builder.hint(input_stream, hint);
        let b = output_stream.read::<ByteVariable>(&mut builder);
        builder.write(b);

        let circuit = builder.build();

        // Write to the circuit input.
        let mut input = circuit.input();
        input.write::<ByteVariable>(5u8);

        // Test the serialization
        let gate_serializer = GateRegistry::new();
        let mut generator_serializer = WitnessGeneratorRegistry::new();
        generator_serializer.register_hint::<AddSome>();
        circuit.test_serializers(&gate_serializer, &generator_serializer);

        // serialize, deserialize, and then generate a proof.
        let bytes = circuit
            .serialize(&gate_serializer, &generator_serializer)
            .unwrap();

        let circuit =
            CircuitBuild::deserialize(&bytes, &gate_serializer, &generator_serializer).unwrap();

        // generate a proof with the deserialized circuit.
        let (proof, mut output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let byte_plus_one = output.read::<ByteVariable>();
        assert_eq!(byte_plus_one, 7u8);
    }
}
