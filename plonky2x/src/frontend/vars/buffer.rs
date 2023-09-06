use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::util::serialization::{IoResult, Read, Write};

use super::{CircuitVariable, Variable};
use crate::prelude::CircuitBuilder;

#[derive(Debug, Clone)]
pub struct ValueStream<F, const D: usize>(Stream<F>);

#[derive(Debug, Clone)]
pub struct VariableStream(Stream<Variable>);

#[derive(Debug, Clone)]
pub struct Stream<T> {
    data: Vec<T>,
    position: usize,
}

impl<T> Stream<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, position: 0 }
    }

    pub fn read_exact(&mut self, len: usize) -> &[T] {
        if (self.position + len) > self.data.len() {
            panic!("Not enough elements in Stream");
        }
        let out_slice = self.data[self.position..self.position + len].as_ref();
        self.position += len;

        out_slice
    }

    /// Read all remaining elements
    pub fn read_all(&self) -> &[T] {
        let length = self.data.len() - self.position;
        &self.data[self.position..self.position + length]
    }

    /// Drain the stream and return the underlying data (including data already read)
    pub fn drain(self) -> Vec<T> {
        self.data
    }

    pub fn write_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice);
    }
}

impl Stream<Variable> {
    pub fn read<V: CircuitVariable>(&mut self) -> V {
        let variables = self.read_exact(V::nb_elements());
        V::from_variables(variables)
    }
}

impl VariableStream {
    pub fn new() -> Self {
        Self(Stream::new(Vec::new()))
    }

    pub fn from_variables(variables: Vec<Variable>) -> Self {
        Self(Stream::new(variables))
    }

    pub fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        size: usize,
    ) -> Self {
        let variables = (0..size)
            .map(|_| builder.init::<Variable>())
            .collect::<Vec<_>>();
        Self(Stream::new(variables))
    }

    pub fn read_exact<F: RichField + Extendable<D>, const D: usize>(
        &mut self,
        builder: &mut CircuitBuilder<F, D>,
        len: usize,
    ) -> &[Variable] {
        let variables = (0..len)
            .map(|_| builder.init::<Variable>())
            .collect::<Vec<_>>();
        self.0.write_slice(&variables);

        self.0.read_exact(len)
    }

    pub fn read<F: RichField + Extendable<D>, const D: usize, V: CircuitVariable>(
        &mut self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> V {
        let variables = self.read_exact(builder, V::nb_elements());
        V::from_variables(variables)
    }

    pub(crate) fn all_variables(&self) -> &[Variable] {
        self.0.read_all()
    }

    pub fn write<F: RichField + Extendable<D>, const D: usize, V: CircuitVariable>(
        &mut self,
        _builder: &mut CircuitBuilder<F, D>,
        value: &V,
    ) {
        self.0.write_slice(&value.variables());
    }

    /// Derialize the stream from a buffer compatible with `Plonky2` serialization
    pub fn deserialize_from_reader(reader: &mut impl Read) -> IoResult<Self> {
        let variables = reader
            .read_target_vec()?
            .into_iter()
            .map(Variable)
            .collect::<Vec<_>>();
        Ok(VariableStream::from_variables(variables))
    }

    /// Serialize the stream to a buffer compatible with `Plonky2` serialization
    pub fn serialize_to_writer(&self, writer: &mut impl Write) -> IoResult<()> {
        let targets = self.0.read_all().iter().map(|v| v.0).collect::<Vec<_>>();
        writer.write_target_vec(&targets)
    }
}

impl Default for VariableStream {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> ValueStream<F, D> {
    pub fn new() -> Self {
        Self(Stream::new(Vec::new()))
    }

    pub fn from_values(values: Vec<F>) -> Self {
        Self(Stream::new(values))
    }

    pub fn read_value<V: CircuitVariable>(&mut self) -> V::ValueType<F> {
        let elements = self.0.read_exact(V::nb_elements());
        V::from_elements(elements)
    }

    pub(crate) fn read_all(&mut self) -> &[F] {
        self.0.read_all()
    }

    pub fn write_value<V: CircuitVariable>(&mut self, value: V::ValueType<F>) {
        self.0.write_slice(&V::elements(value));
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn init_stream(&mut self, size: usize) -> VariableStream {
        VariableStream::init(self, size)
    }

    pub fn read<V: CircuitVariable>(&mut self, stream: &mut VariableStream) -> V {
        stream.read::<F, D, V>(self)
    }

    pub fn write<V: CircuitVariable>(&mut self, stream: &mut VariableStream, value: &V) {
        stream.write::<F, D, V>(self, value)
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Default for ValueStream<F, D> {
    fn default() -> Self {
        Self::new()
    }
}
