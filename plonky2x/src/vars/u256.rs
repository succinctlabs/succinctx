use super::BoolVariable;
// use crate::impl_variable_methods;

#[derive(Debug, Clone, Copy)]
pub struct U256Variable(pub [BoolVariable; 256]);
