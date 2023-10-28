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
    Variable(Vec<ByteVariable>, U32Variable, U32Variable),
}

impl SHARequest {
    pub const fn req_type(&self) -> SHARequestType {
        match self {
            SHARequest::Fixed(_) => SHARequestType::Fixed,
            SHARequest::Variable(_, _, _) => SHARequestType::Variable,
        }
    }
}
