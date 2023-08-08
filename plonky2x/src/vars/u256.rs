use super::BoolVariable;

#[derive(Debug, Clone, Copy)]
pub struct U256Variable(pub [BoolVariable; 256]);
