use core::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator, WitnessGeneratorRef};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::backend::circuit::serialization::Serializer;
use crate::frontend::vars::{ValueStream, VariableStream};
use crate::prelude::{CircuitBuilder, CircuitVariable};

#[derive(Debug, Clone)]
pub struct Hint<F, const D: usize> {
    input_stream: VariableStream,
    output_stream: VariableStream,
    hint_fn: fn(&mut ValueStream<F, D>, &mut ValueStream<F, D>),
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn hint(
        &mut self,
        input_stream: &VariableStream,
        hint_fn: fn(&mut ValueStream<F, D>, &mut ValueStream<F, D>),
    ) -> &mut VariableStream {
        let num_inputs = input_stream.all_variables().len();
        let input_stream = VariableStream::init(self, num_inputs);

        let output_stream = VariableStream::new();

        let hint = Hint::<F, D> {
            input_stream,
            output_stream,
            hint_fn,
        };
        self.hints.push(hint);

        &mut self.hints.last_mut().unwrap().output_stream
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D> for Hint<F, D> {
    fn id(&self) -> String {
        let hint_serializer = HintSerializer::<F, D>::new(self.hint_fn);
        hint_serializer.id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.input_stream
            .all_variables()
            .iter()
            .map(|v| v.0)
            .collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let input_values = self
            .input_stream
            .all_variables()
            .iter()
            .map(|v| v.get(witness))
            .collect::<Vec<_>>();
        let mut input_stream = ValueStream::from_values(input_values);
        let mut output_stream = ValueStream::new();

        (self.hint_fn)(&mut input_stream, &mut output_stream);

        let output_values = output_stream.read_all();
        let output_vars = self.output_stream.all_variables();
        assert_eq!(output_values.len(), output_vars.len());

        for (var, val) in output_vars.iter().zip(output_values) {
            var.set(out_buffer, *val)
        }
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        self.input_stream.serialize_to_writer(dst)?;
        self.output_stream.serialize_to_writer(dst)
    }

    fn deserialize(_src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self>
    where
        Self: Sized,
    {
        unimplemented!("Hint functions are not deserializable through the plonky2 crate, only directly through the witness registry")
    }
}

#[derive(Debug)]
pub struct HintSerializer<F, const D: usize> {
    pub hint_fn: fn(&mut ValueStream<F, D>, &mut ValueStream<F, D>),
}

impl<F, const D: usize> HintSerializer<F, D> {
    pub fn new(hint_fn: fn(&mut ValueStream<F, D>, &mut ValueStream<F, D>)) -> Self {
        Self { hint_fn }
    }

    pub fn id(&self) -> String {
        format!("--Hint, fn :{:?}", self.hint_fn).to_string()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Serializer<F, WitnessGeneratorRef<F, D>, D>
    for HintSerializer<F, D>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<WitnessGeneratorRef<F, D>> {
        let input_stream = VariableStream::deserialize_from_reader(buf)?;
        let output_stream = VariableStream::deserialize_from_reader(buf)?;

        let hint = Hint::<F, D> {
            input_stream,
            output_stream,
            hint_fn: self.hint_fn,
        };

        Ok(WitnessGeneratorRef::new(hint.adapter()))
    }

    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &WitnessGeneratorRef<F, D>,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()> {
        object.0.serialize(buf, common_data)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::backend::circuit::serialization::{GateRegistry, WitnessGeneratorRegistry};
    // use crate::prelude::{ByteVariable, *};

    // fn plus_one<F: RichField + Extendable<D>, const D: usize>(
    //     input_stream: &mut ValueStream<F, D>,
    //     output_stream: &mut ValueStream<F, D>,
    // ) {
    //     let byte: u8 = input_stream.read_value::<ByteVariable>();

    //     output_stream.write_value::<ByteVariable>(byte + 1)
    // }

    // #[test]
    // fn test_hint_serialization() {
    //     let mut builder = CircuitBuilderX::new();

    //     let a = builder.read_input::<ByteVariable>();

    //     let mut input_stream = VariableStream::new();
    //     builder.write(&mut input_stream, &a);

    //     let mut output_stream = builder.hint(&input_stream, plus_one);
    //     let b = builder.read::<ByteVariable>(&mut output_stream);
    //     builder.write_output(b);

    //     let circuit = builder.build::<PoseidonGoldilocksConfig>();

    //     // Write to the circuit input.
    //     let mut input = circuit.input();
    //     input.write::<ByteVariable>(5u8);

    //     // Generate a proof.
    //     let (proof, output) = circuit.prove(&input);

    //     // Verify proof.
    //     circuit.verify(&proof, &input, &output);

    //     // Read output.
    //     let byte_plus_one = output.read::<ByteVariable>();
    //     assert_eq!(byte_plus_one, 6u8);

    //     // Test the serialization
    //     let gate_serializer = GateRegistry::new();
    //     let mut generator_serializer = WitnessGeneratorRegistry::new::<PoseidonGoldilocksConfig>();
    //     generator_serializer.register_hint(plus_one);
    //     circuit.test_serializers(&gate_serializer, &generator_serializer);
    // }
}
