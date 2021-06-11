use std::time::Duration;

use reqwest::{Client, ClientBuilder};
use tokio::time::sleep;

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
                .timeout(Duration::from_secs(2))
                .user_agent(USER_AGENT)
                .build()
                .expect("failed to create reqwest client"),
        }
    }

    async fn get_once(&self, url: &str) -> reqwest::Result<String> {
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    pub async fn get(&self, url: &str) -> reqwest::Result<String> {
        let mut i: usize = 1;
        loop {
            let body = self.get_once(url).await;
            if i >= 2 || body.is_ok() {
                return body;
            }
            i += 1;
            sleep(Duration::from_millis(800)).await;
        }
    }
}
