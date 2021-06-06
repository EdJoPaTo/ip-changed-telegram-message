use std::time::Duration;

use reqwest::{Client, ClientBuilder};

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

#[derive(Clone)]
pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Self {
        Self {
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .user_agent(USER_AGENT)
                .build()
                .expect("failed to create reqwest client"),
        }
    }

    pub async fn get(&self, url: &str) -> anyhow::Result<String> {
        let body = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(body)
    }
}
