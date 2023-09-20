use core::marker::PhantomData;

use plonky2::iop::generator::WitnessGeneratorRef;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoError, IoResult};

use super::generator::{AsyncHintData, AsyncHintRef};
use super::hint::AsyncHint;
use crate::backend::circuit::Serializer;
use crate::frontend::vars::VariableStream;
use crate::prelude::PlonkParameters;
use crate::utils::serde::BufferRead;

#[derive(Debug, Clone)]
pub struct AsyncHintSerializer<L, H>(PhantomData<L>, PhantomData<H>);

impl<L, H> AsyncHintSerializer<L, H> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<L, H> Default for AsyncHintSerializer<L, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize>
    Serializer<L::Field, WitnessGeneratorRef<L::Field, D>, D> for AsyncHintSerializer<L, H>
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
        let hint_data = AsyncHintData::<L, H, D>::new(hint, input_stream, output_stream);

        Ok(WitnessGeneratorRef::new(hint_data))
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

impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize>
    Serializer<L::Field, AsyncHintRef<L, D>, D> for AsyncHintSerializer<L, H>
{
    fn read(
        &self,
        buf: &mut Buffer,
        _common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<AsyncHintRef<L, D>> {
        let input_stream = VariableStream::deserialize_from_reader(buf)?;
        let output_stream = VariableStream::deserialize_from_reader(buf)?;

        let bytes = buf.read_bytes()?;
        let hint: H = bincode::deserialize(&bytes).map_err(|_| IoError)?;
        let hint_data = AsyncHintData::<L, H, D>::new(hint, input_stream, output_stream);

        Ok(AsyncHintRef::new(hint_data))
    }

    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &AsyncHintRef<L, D>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        object.0.serialize(buf, common_data)
    }
}
