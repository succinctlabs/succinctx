use crate::builder::BuilderAPI;

pub struct SimpleSerializeAPI {
    pub api: BuilderAPI,
}

impl SimpleSerializeAPI {
    pub fn new(api: BuilderAPI) -> Self {
        Self { api }
    }
}
