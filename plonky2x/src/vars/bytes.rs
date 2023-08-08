use super::Variable;

#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [Variable;N]);