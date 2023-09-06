use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::{CircuitVariable, Variable};

#[derive(Debug, Clone)]
pub struct VariableBuffer<'a> {
    variables: &'a [Variable],
    position: usize,
}

impl<'a> VariableBuffer<'a> {
    pub fn new(variables: &'a [Variable]) -> Self {
        Self {
            variables,
            position: 0,
        }
    }

    pub fn read<V: CircuitVariable>(&mut self) -> V {
        let variables = self.read_exact(V::nb_elements());
        V::from_variables(variables)
    }

    pub fn read_exact(&mut self, len: usize) -> &[Variable] {
        if (self.position + len) > self.variables.len() {
            panic!("Not enough variables in buffer");
        }
        let out_slice = self.variables[self.position..self.position + len].as_ref();
        self.position += len;

        out_slice
    }

    pub fn read_all(&mut self) -> &[Variable] {
        let length = self.variables.len() - self.position;
        self.read_exact(length)
    }
}

#[derive(Debug, Clone)]
pub struct ElementBuffer<'a, F, const D: usize> {
    elements: &'a [F],
    position: usize,
}

impl<'a, F: RichField + Extendable<D>, const D: usize> ElementBuffer<'a, F, D> {
    pub fn new(elements: &'a [F]) -> Self {
        Self {
            elements,
            position: 0,
        }
    }

    pub fn read<V: CircuitVariable>(&mut self) -> <V as CircuitVariable>::ValueType<F> {
        let elements = self.read_exact(V::nb_elements());
        V::from_elements(elements)
    }

    pub fn read_exact(&mut self, len: usize) -> &[F] {
        if (self.position + len) > self.elements.len() {
            panic!("Not enough elements in buffer");
        }
        let out_slice = self.elements[self.position..self.position + len].as_ref();
        self.position += len;

        out_slice
    }
}
