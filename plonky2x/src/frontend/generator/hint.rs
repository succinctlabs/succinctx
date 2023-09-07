use core::any::TypeId;
use core::fmt::Debug;
use core::marker::PhantomData;

use plonky2::iop::generator::{GeneratedValues, SimpleGenerator, WitnessGeneratorRef};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::backend::circuit::serialization::Serializer;
use crate::backend::config::PlonkParameters;
use crate::frontend::vars::{OutputVariableStream, ValueStream, VariableStream};
use crate::prelude::{CircuitBuilder, CircuitVariable};

pub trait Hint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>);
}

impl<L: PlonkParameters<D>, const D: usize, F> Hint<L, D> for F
where
    F: Fn(&mut ValueStream<L, D>, &mut ValueStream<L, D>) + 'static + Debug + Clone + Send + Sync,
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        self(input_stream, output_stream)
    }
}

#[derive(Debug, Clone)]
pub struct HintFn<L: PlonkParameters<D>, const D: usize>(
    pub fn(&mut ValueStream<L, D>, &mut ValueStream<L, D>),
);

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for HintFn<L, D> {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        (self.0)(input_stream, output_stream)
    }
}

pub trait HintRef<L: PlonkParameters<D>, const D: usize> {
    fn output_stream(&mut self) -> &mut VariableStream;
    fn register(&self, builder: &mut CircuitBuilder<L, D>);
}

#[derive(Debug, Clone)]
pub struct HintGenerator<L, H> {
    pub(crate) input_stream: VariableStream,
    pub(crate) output_stream: VariableStream,
    hint: H,
    _marker: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize, H: Hint<L, D>> HintRef<L, D> for HintGenerator<L, H> {
    fn output_stream(&mut self) -> &mut VariableStream {
        &mut self.output_stream
    }

    fn register(&self, builder: &mut CircuitBuilder<L, D>) {
        builder.add_simple_generator(self.clone())
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn hint<H: Hint<L, D>>(
        &mut self,
        input_stream: VariableStream,
        hint: H,
    ) -> OutputVariableStream<L, D> {
        let output_stream = VariableStream::new();

        let hint = HintGenerator::<L, H> {
            input_stream,
            output_stream,
            hint,
            _marker: PhantomData,
        };
        let hint_id = self.hints.len();
        self.hints.push(Box::new(hint));

        OutputVariableStream::new(hint_id)
    }
}

impl<L: PlonkParameters<D>, const D: usize, H: Hint<L, D>> SimpleGenerator<L::Field, D>
    for HintGenerator<L, H>
{
    fn id(&self) -> String {
        let hint_serializer = HintSerializer::<L, H>::new(self.hint.clone());
        hint_serializer.id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.input_stream.real_all().iter().map(|v| v.0).collect()
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let input_values = self
            .input_stream
            .real_all()
            .iter()
            .map(|v| v.get(witness))
            .collect::<Vec<_>>();
        let mut input_stream = ValueStream::from_values(input_values);
        let mut output_stream = ValueStream::new();

        self.hint.hint(&mut input_stream, &mut output_stream);

        let output_values = output_stream.read_all();
        let output_vars = self.output_stream.real_all();
        assert_eq!(output_values.len(), output_vars.len());

        for (var, val) in output_vars.iter().zip(output_values) {
            var.set(out_buffer, *val)
        }
    }

    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        self.input_stream.serialize_to_writer(dst)?;
        self.output_stream.serialize_to_writer(dst)
    }

    fn deserialize(
        _src: &mut Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self>
    where
        Self: Sized,
    {
        unimplemented!("Hint functions are not deserializable through the plonky2 crate, only directly through the witness registry")
    }
}

#[derive(Debug)]
pub struct HintSerializer<L, H> {
    pub hint: H,
    _marker: PhantomData<L>,
}

impl<L: 'static, H: 'static> HintSerializer<L, H> {
    pub fn new(hint: H) -> Self {
        Self {
            hint,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> String {
        format!(
            "--Hint, name:{:?}, id: {:?}",
            core::any::type_name::<H>(),
            TypeId::of::<H>()
        )
        .to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize, H: Hint<L, D>>
    Serializer<L::Field, WitnessGeneratorRef<L::Field, D>, D> for HintSerializer<L, H>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<WitnessGeneratorRef<L::Field, D>> {
        let input_stream = VariableStream::deserialize_from_reader(buf)?;
        let output_stream = VariableStream::deserialize_from_reader(buf)?;

        let hint = HintGenerator::<L, H> {
            input_stream,
            output_stream,
            hint: self.hint.clone(),
            _marker: PhantomData,
        };

        Ok(WitnessGeneratorRef::new(hint.adapter()))
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
    fn test_hint_serialization() {
        let mut builder = CircuitBuilderX::new();

        let a = builder.read::<ByteVariable>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a);

        let output_stream = builder.hint(input_stream, HintFn(plus_one));
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
        generator_serializer.register_hint(HintFn(plus_one));
        circuit.test_serializers(&gate_serializer, &generator_serializer);
    }
}
