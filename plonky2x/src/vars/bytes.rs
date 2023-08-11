use super::{BoolVariable, Bytes32Variable};

#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [[BoolVariable; 8]; N]);

impl<const N: usize> BytesVariable<N> {
    pub fn new() -> BytesVariable<N> {
        BytesVariable([[BoolVariable::default(); 8]; N])
    }
}

impl From<Bytes32Variable> for BytesVariable<32> {
    fn from(value: Bytes32Variable) -> Self {
        let mut result: [[BoolVariable; 8]; 32] = Default::default();
        for (i, item) in value.0.iter().enumerate() {
            result[i / 8][i % 8] = *item;
        }
        BytesVariable::<32>(result)
    }
}
