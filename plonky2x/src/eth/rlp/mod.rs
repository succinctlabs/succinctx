use crate::builder::API;

struct RecursiveLengthPrefixAPI {
    api: API,
}

impl RecursiveLengthPrefixAPI {
    fn new(api: API) -> Self {
        Self { api }
    }
}
