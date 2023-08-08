use crate::builder::BuilderAPI;

pub struct RecursiveLengthPrefixAPI {
    pub api: BuilderAPI,
}

impl RecursiveLengthPrefixAPI {
    pub fn new(api: BuilderAPI) -> Self {
        Self { api }
    }
}
