use super::request::BLAKE2BRequest;
use crate::prelude::U64Variable;

#[derive(Debug, Clone)]
pub struct BLAKE2BAccelerator {
    pub blake2b_requests: Vec<BLAKE2BRequest>,
    pub blake2b_responses: Vec<[U64Variable; 4]>,
}
