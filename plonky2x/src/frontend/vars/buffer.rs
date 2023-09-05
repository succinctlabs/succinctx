use super::Variable;
struct VariableBuffer {
    variables: Vec<Variable>,
    position: usize,
}

impl VariableBuffer {
    pub fn new(variables: Vec<Variable>) -> Self {
        Self {
            variables,
            position: 0,
        }
    }

    pub fn read(&mut self, len: usize) -> &[Variable] {
        if (self.position + len) > self.variables.len() {
            panic!("Not enough variables in buffer");
        }
        self.position += len;
        self.variables[self.position..self.position + len].as_ref()
    }
}
