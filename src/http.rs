use std::time::Duration;

use tokio::time::sleep;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

async fn get_once(url: &str) -> reqwest::Result<String> {
    reqwest::Client::new()
        .get(url)
        .timeout(Duration::from_secs(5))
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
}

pub async fn get(url: &str) -> reqwest::Result<String> {
    let mut i: usize = 1;
    loop {
        let body = get_once(url).await;
        if i >= 3 || body.is_ok() {
            return body;
        }
        i += 1;
        sleep(Duration::from_millis(1500)).await;
    }
}
