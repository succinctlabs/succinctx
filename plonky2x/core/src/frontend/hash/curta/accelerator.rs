use crate::frontend::hash::curta::request::HashRequest;

#[derive(Debug, Clone)]
pub struct HashAccelerator<T, const S: usize> {
    pub hash_requests: Vec<HashRequest>,
    pub hash_responses: Vec<[T; S]>,
}
