use super::request::SHARequest;

#[derive(Debug, Clone)]
pub struct SHAAccelerator<T> {
    pub sha_requests: Vec<SHARequest>,
    pub sha_responses: Vec<[T; 8]>,
}
