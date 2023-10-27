use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SHARequestType {
    Fixed,
    Variable,
}

/// A SHA request.
#[derive(Debug, Clone)]
pub enum SHARequest {
    /// A message of fixed length.
    Fixed(Vec<ByteVariable>),
    /// A message of variable length, with the actual legnth given by the Variable.
    Variable(Vec<ByteVariable>, Variable),
}

impl SHARequest {
    pub const fn req_type(&self) -> SHARequestType {
        match self {
            SHARequest::Fixed(_) => SHARequestType::Fixed,
            SHARequest::Variable(_, _) => SHARequestType::Variable,
        }
    }

    pub fn input_len(&self) -> usize {
        match self {
            SHARequest::Fixed(msg) => msg.len(),
            SHARequest::Variable(msg, _) => msg.len(),
        }
    }
}
