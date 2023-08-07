use crate::builder::API;

struct SimpleSerializeAPI {
    api: API,
}

impl SimpleSerializeAPI {
    fn new(api: API) -> Self {
        Self { api }
    }
}
