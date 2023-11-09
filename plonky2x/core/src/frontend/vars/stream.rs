use core::marker::PhantomData;

use plonky2::iop::target::Target;
use plonky2::util::serialization::{IoResult, Read, Write};
use serde::{Deserialize, Serialize};

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::prelude::CircuitBuilder;
use crate::utils::stream::Stream;

/// A stream of field elements.
///
/// This struct is used as a buffer for `CircuitVariable`s values.
#[derive(Debug, Clone)]
pub struct ValueStream<L: PlonkParameters<D>, const D: usize>(Stream<L::Field>);

/// A stream of variables.
///
/// This struct is used as a buffer for `CircuitVariable`s.
#[derive(Debug, Clone)]
pub struct VariableStream(Stream<Variable>);

/// A stream  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputVariableStream<L: PlonkParameters<D>, const D: usize> {
    hint_id: usize,
    _marker: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> OutputVariableStream<L, D> {
    pub(crate) fn new(hint_id: usize) -> Self {
        Self {
            hint_id,
            _marker: PhantomData,
        }
    }

    pub fn read_exact(&self, builder: &mut CircuitBuilder<L, D>, len: usize) -> Vec<Variable> {
        let variables = (0..len)
            .map(|_| builder.init_unsafe::<Variable>())
            .collect::<Vec<_>>();
        let stream = &mut builder
            .hints
            .get_mut(self.hint_id)
            .expect("Hint not found")
            .output_stream_mut();
        stream.0.write_slice(&variables);

        variables
    }

    /// Read a single variable from the stream.
    ///
    /// The output value is asserted to be a valid circuit variable.
    pub fn read<V: CircuitVariable>(&self, builder: &mut CircuitBuilder<L, D>) -> V {
        let variables = self.read_exact(builder, V::nb_elements());

        V::from_variables(builder, &variables)
    }

    /// Read a circuit variable from the output stream without doing any validity checks.
    pub fn read_unsafe<V: CircuitVariable>(&self, builder: &mut CircuitBuilder<L, D>) -> V {
        let variables = self.read_exact(builder, V::nb_elements());

        V::from_variables_unsafe(&variables)
    }

    pub fn read_vec<V: CircuitVariable>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        len: usize,
    ) -> Vec<V> {
        (0..len).map(|_| self.read::<V>(builder)).collect()
    }
}

impl VariableStream {
    pub fn new() -> Self {
        Self(Stream::new(Vec::new()))
    }

    pub fn from_variables(variables: Vec<Variable>) -> Self {
        Self(Stream::new(variables))
    }

    pub fn from_targets(targets: Vec<Target>) -> Self {
        Self(Stream::new(
            targets.into_iter().map(Variable).collect::<Vec<_>>(),
        ))
    }

    pub fn init<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        size: usize,
    ) -> Self {
        let variables = (0..size)
            .map(|_| builder.init::<Variable>())
            .collect::<Vec<_>>();
        Self(Stream::new(variables))
    }

    pub fn real_all(&self) -> &[Variable] {
        self.0.read_all()
    }

    pub fn read_exact(&mut self, len: usize) -> &[Variable] {
        self.0.read_exact(len)
    }

    pub fn read<V: CircuitVariable>(&mut self) -> V {
        let variables = self.0.read_exact(V::nb_elements());
        // Reads from stream don't do any validity checks on the circuit variable.  It is the
        // stream reader's responsibility to do so.
        V::from_variables_unsafe(variables)
    }

    pub fn read_vec<V: CircuitVariable>(&mut self, len: usize) -> Vec<V> {
        (0..len).map(|_| self.read::<V>()).collect()
    }

    pub fn write<V: CircuitVariable>(&mut self, value: &V) {
        self.0.write_slice(&value.variables());
    }

    pub fn write_slice<V: CircuitVariable>(&mut self, values: &[V]) {
        values.iter().for_each(|v| self.write(v));
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

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    pub fn new() -> Self {
        Self(Stream::new(Vec::new()))
    }

    pub fn from_values(values: Vec<L::Field>) -> Self {
        Self(Stream::new(values))
    }

    pub fn read_value<V: CircuitVariable>(&mut self) -> V::ValueType<L::Field> {
        let elements = self.0.read_exact(V::nb_elements());
        V::from_elements::<L::Field>(elements)
    }

    pub fn read_exact(&mut self, len: usize) -> &[L::Field] {
        self.0.read_exact(len)
    }

    pub fn read_vec<V: CircuitVariable>(&mut self, len: usize) -> Vec<V::ValueType<L::Field>> {
        (0..len).map(|_| self.read_value::<V>()).collect()
    }

    pub fn write_slice(&mut self, values: &[L::Field]) {
        self.0.write_slice(values);
    }

    pub fn read_all(&mut self) -> &[L::Field] {
        self.0.read_all()
    }

    pub fn write_value<V: CircuitVariable>(&mut self, value: V::ValueType<L::Field>) {
        self.0.write_slice(&V::elements::<L::Field>(value));
    }
}

impl<L: PlonkParameters<D>, const D: usize> Default for ValueStream<L, D> {
    fn default() -> Self {
        Self::new()
    }
}
