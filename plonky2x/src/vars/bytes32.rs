use super::{BoolVariable, BytesVariable};

#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);
