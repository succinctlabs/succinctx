use crate::builder::API;

pub struct RecursiveLengthPrefixAPI {
    pub api: API,
}

impl RecursiveLengthPrefixAPI {
    pub fn new(api: API) -> Self {
        Self { api }
    }
}
