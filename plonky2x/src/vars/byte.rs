use super::Variable;

#[derive(Debug, Clone, Copy)]
pub struct ByteVariable(pub Variable);

impl From<Variable> for ByteVariable {
    fn from(item: Variable) -> Self {
        Self(item)
    }
}
