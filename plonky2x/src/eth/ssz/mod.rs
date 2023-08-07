use crate::builder::API;

pub struct SimpleSerializeAPI {
    pub api: API,
}

impl SimpleSerializeAPI {
    pub fn new(api: API) -> Self {
        Self { api }
    }
}
