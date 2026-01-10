use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::time::Duration;

use tokio::time::sleep;

use crate::http;

pub struct IPs {
    pub v4: anyhow::Result<Ipv4Addr>,
    pub v6: anyhow::Result<Ipv6Addr>,
}

impl IPs {
    pub async fn get() -> Self {
        let v4 = get_addr(&["https://ipv4.edjopato.de", "https://4.ipwho.de/ip"]);
        let v6 = get_addr(&["https://ipv6.edjopato.de", "https://6.ipwho.de/ip"]);

        let (v4, v6) = futures::join!(v4, v6);

        Self { v4, v6 }
    }
}

async fn get_addr<A: FromStr>(ip_urls: &[&str]) -> anyhow::Result<A> {
    let mut last = None;
    for url in ip_urls.iter().cycle().take(3) {
        let addr = single_attempt(url).await;
        if addr.is_ok() {
            return addr;
        }
        last = Some(addr);
        sleep(Duration::from_millis(100)).await;
    }
    last.expect("ip_urls was empty")
}

#[expect(clippy::map_err_ignore)]
async fn single_attempt<A: FromStr>(ip_url: &str) -> anyhow::Result<A> {
    let body = http::get(ip_url).await?;
    let addr = body
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("parsing ip address failed: {body}"))?;
    Ok(addr)
}
