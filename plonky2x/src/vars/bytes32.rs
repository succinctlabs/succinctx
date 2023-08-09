use super::BoolVariable;

#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub [BoolVariable; 256]);
