use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HashRequestType {
    Fixed,
    Variable,
}

/// A request for a SHA computation.
#[derive(Debug, Clone)]
pub enum HashRequest {
    /// A message of fixed length.
    Fixed(Vec<ByteVariable>),
    /// A message of variable length, represented by a tuple `(total_message, lengh, last_chunk)`.
    Variable(Vec<ByteVariable>, U32Variable, U32Variable),
}

impl HashRequest {
    /// Returns the type of the request.
    pub const fn req_type(&self) -> HashRequestType {
        match self {
            HashRequest::Fixed(_) => HashRequestType::Fixed,
            HashRequest::Variable(_, _, _) => HashRequestType::Variable,
        }
    }
}
