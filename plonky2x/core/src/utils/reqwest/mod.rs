use core::time::Duration;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::blocking::Response;

#[derive(Debug, Clone, Default)]
pub struct ReqwestClient {
    pub client: reqwest::blocking::Client,
    pub client_async: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        ReqwestClient {
            client: reqwest::blocking::Client::new(),
            client_async: reqwest::Client::new(),
        }
    }

    pub fn from_clients(client: reqwest::blocking::Client, client_async: reqwest::Client) -> Self {
        ReqwestClient {
            client,
            client_async,
        }
    }

    pub async fn fetch_async(&self, endpoint: &str) -> Result<reqwest::Response> {
        const MAX_RETRIES: u32 = 2;
        const INITIAL_RETRY_DELAY: u64 = 5;

        let mut retries = 0;
        let mut retry_delay = INITIAL_RETRY_DELAY;

        loop {
            debug!("fetching {}: retries={}", endpoint, retries);
            let response = self
                .client_async
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
        const MAX_RETRIES: u32 = 2;
        const INITIAL_RETRY_DELAY: u64 = 5;

        let mut retries = 0;
        let mut retry_delay = INITIAL_RETRY_DELAY;

        loop {
            let response = self
                .client
                .get(endpoint)
                .timeout(core::time::Duration::from_secs(60))
                .send();

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

            std::thread::sleep(Duration::from_secs(retry_delay));
            retry_delay *= 2;
            retries += 1;
        }
    }
}
