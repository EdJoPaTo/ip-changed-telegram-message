use std::time::Duration;

use reqwest::header::{HeaderValue, USER_AGENT};

const USER_AGENT_VALUE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub async fn get(url: &str) -> reqwest::Result<String> {
    reqwest::Client::new()
        .get(url)
        .timeout(Duration::from_secs(5))
        .header(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
}
