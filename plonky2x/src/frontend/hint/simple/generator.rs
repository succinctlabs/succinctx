use core::marker::PhantomData;

use plonky2::iop::generator::{GeneratedValues, WitnessGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoError, IoResult};

use super::hint::Hint;
use crate::frontend::hint::HintGenerator;
use crate::frontend::vars::{ValueStream, VariableStream};
use crate::prelude::{CircuitVariable, PlonkParameters};
use crate::utils::serde::BufferWrite;

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

impl<L: PlonkParameters<D>, const D: usize, H: Hint<L, D>> HintGenerator<L, D>
    for HintSimpleGenerator<L, H>
{
    fn output_stream_mut(&mut self) -> &mut VariableStream {
        &mut self.output_stream
    }
}

impl<L: PlonkParameters<D>, const D: usize, H: Hint<L, D>> WitnessGenerator<L::Field, D>
    for HintSimpleGenerator<L, H>
{
    fn id(&self) -> String {
        H::id()
    }

    fn watch_list(&self) -> Vec<Target> {
        self.input_stream.real_all().iter().map(|v| v.0).collect()
    }

    fn run(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) -> bool {
        if !witness.contains_all(&self.watch_list()) {
            return false;
        }
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
        assert_eq!(
            output_values.len(),
            output_vars.len(),
            "Hint output stream length does not match output variables length"
        );

        for (var, val) in output_vars.iter().zip(output_values) {
            var.set(out_buffer, *val)
        }
        true
    }

    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        self.input_stream.serialize_to_writer(dst)?;
        self.output_stream.serialize_to_writer(dst)?;

        let bytes = bincode::serialize(&self.hint).map_err(|_| IoError)?;
        dst.write_bytes(&bytes)
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
