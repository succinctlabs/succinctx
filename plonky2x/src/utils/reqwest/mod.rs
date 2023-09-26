use core::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReqwestClient;

impl ReqwestClient {
    pub fn new() -> Self {
        ReqwestClient {}
    }

    pub fn fetch(&self, endpoint: &str) -> Result<Response> {
        const MAX_RETRIES: u32 = 5;
        const INITIAL_RETRY_DELAY: u64 = 5;

        let client = reqwest::blocking::Client::new();
        let mut retries = 0;
        let mut retry_delay = INITIAL_RETRY_DELAY;

        loop {
            let response = client
                .get(endpoint)
                .timeout(core::time::Duration::from_secs(300))
                .send();

            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        return Ok(res);
                    } else if res.status().is_server_error() {
                        if retries >= MAX_RETRIES {
                            return Err(anyhow!("Maximum retries exceeded"));
                        }
                    } else {
                        return Ok(res);
                    }
                }
                Err(_) => {
                    if retries >= MAX_RETRIES {
                        return Err(anyhow!("Maximum retries exceeded"));
                    }
                }
            }

            std::thread::sleep(Duration::from_secs(retry_delay));
            retry_delay *= 2;
            retries += 1;
        }
    }
}
