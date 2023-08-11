use super::{BoolVariable, BytesVariable};

#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);

// TODO this should be a macro
impl From<Bytes32Variable> for BytesVariable<32> {
    fn from(value: Bytes32Variable) -> Self {
        let mut result: [[BoolVariable; 8]; 32] = Default::default();

        for (i, item) in value.0.iter().enumerate() {
            result[i / 8][i % 8] = *item;
        }

        BytesVariable::<32>(result)
    }
}