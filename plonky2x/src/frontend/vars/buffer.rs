use super::{CircuitVariable, Variable};
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
        self.position += len;
        self.variables[self.position..self.position + len].as_ref()
    }
}
