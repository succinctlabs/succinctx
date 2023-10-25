use core::time::Duration;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReqwestClient;

impl ReqwestClient {
    pub fn new() -> Self {
        ReqwestClient {}
    }

    pub async fn fetch_async(&self, endpoint: &str) -> Result<reqwest::Response> {
        const MAX_RETRIES: u32 = 3;
        const INITIAL_RETRY_DELAY: u64 = 5;

        let client = reqwest::Client::new();
        let mut retries = 0;
        let mut retry_delay = INITIAL_RETRY_DELAY;

        loop {
            debug!("fetching {}: retries={}", endpoint, retries);
            let response = client
                .get(endpoint)
                .timeout(core::time::Duration::from_secs(60))
                .send()
                .await;

            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        return Ok(res);
                    } else if res.status().is_server_error() {
                        debug!("Server error: {:?}", res.status());
                        if retries >= MAX_RETRIES {
                            return Err(anyhow!("Maximum retries exceeded"));
                        }
                    } else {
                        return Ok(res);
                    }
                }
                Err(err) => {
                    debug!("Connection error {:?}", err);
                    if retries >= MAX_RETRIES {
                        return Err(anyhow!("Maximum retries exceeded"));
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(retry_delay)).await;
            retry_delay *= 2;
            retries += 1;
        }
    }

    pub fn fetch(&self, endpoint: &str) -> Result<Response> {
        const MAX_RETRIES: u32 = 7;
        const INITIAL_RETRY_DELAY: u64 = 5;

        let client = reqwest::blocking::Client::new();
        let mut retries = 0;
        let mut retry_delay = INITIAL_RETRY_DELAY;

        loop {
            let response = client
                .get(endpoint)
                .timeout(core::time::Duration::from_secs(90))
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
