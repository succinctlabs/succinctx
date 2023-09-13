use core::fmt::Debug;
use core::marker::PhantomData;

use plonky2::iop::generator::{GeneratedValues, SimpleGenerator, WitnessGeneratorRef};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::backend::circuit::{PlonkParameters, Serializer};
use crate::frontend::vars::{OutputVariableStream, ValueStream, VariableStream};
use crate::prelude::{CircuitBuilder, CircuitVariable};

pub trait HintGenerator<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync
{
    type Serializer: HintSerializer<L, D>;
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>);

    fn serializer(&self) -> Self::Serializer;
}

pub trait HintSerializer<L: PlonkParameters<D>, const D: usize>:
    Serializer<L::Field, WitnessGeneratorRef<L::Field, D>, D>
{
    fn id(&self) -> String;
}

pub trait HintRef<L: PlonkParameters<D>, const D: usize> {
    fn output_stream(&mut self) -> &mut VariableStream;
    fn register(&self, builder: &mut CircuitBuilder<L, D>);
}

#[derive(Debug, Clone)]
pub struct HintSimpleGenerator<L, H> {
    pub(crate) input_stream: VariableStream,
    pub(crate) output_stream: VariableStream,
    pub(crate) hint: H,
    _marker: PhantomData<L>,
}

impl<L, H> HintSimpleGenerator<L, H> {
    pub fn new(input_stream: VariableStream, output_stream: VariableStream, hint: H) -> Self {
        Self {
            input_stream,
            output_stream,
            hint,
            _marker: PhantomData,
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize, H: HintGenerator<L, D>> HintRef<L, D>
    for HintSimpleGenerator<L, H>
{
    fn output_stream(&mut self) -> &mut VariableStream {
        &mut self.output_stream
    }

    fn register(&self, builder: &mut CircuitBuilder<L, D>) {
        builder.add_simple_generator(self.clone())
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn hint_generator<H: HintGenerator<L, D>>(
        &mut self,
        input_stream: VariableStream,
        hint: H,
    ) -> OutputVariableStream<L, D> {
        let output_stream = VariableStream::new();

        let hint = HintSimpleGenerator::<L, H> {
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

impl<L: PlonkParameters<D>, const D: usize, H: HintGenerator<L, D>> SimpleGenerator<L::Field, D>
    for HintSimpleGenerator<L, H>
{
    fn id(&self) -> String {
        self.hint.serializer().id()
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
        unimplemented!("Hints are not deserializable through the plonky2 crate, only directly through the witness registry")
    }
}
