use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BLAKE2BRequestType {
    Fixed,
    Variable,
}

/// A request for a BLAKE2B computation.
#[derive(Debug, Clone)]
pub enum BLAKE2BRequest {
    /// A message of fixed length.
    Fixed(Vec<ByteVariable>),
    /// A message of variable length, represented by a tuple `(total_message, lengh, last_chunk)`.
    Variable(Vec<ByteVariable>, U64Variable, U64Variable),
}

impl BLAKE2BRequest {
    /// Returns the type of the request.
    pub const fn req_type(&self) -> BLAKE2BRequestType {
        match self {
            BLAKE2BRequest::Fixed(_) => BLAKE2BRequestType::Fixed,
            BLAKE2BRequest::Variable(_, _, _) => BLAKE2BRequestType::Variable,
        }
    }
}
