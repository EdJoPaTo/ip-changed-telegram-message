use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use crate::http;

pub struct IPs {
    pub v4: anyhow::Result<Ipv4Addr>,
    pub v6: anyhow::Result<Ipv6Addr>,
}

impl IPs {
    pub async fn get() -> Self {
        let v4 = get_addr("https://ipv4.edjopato.de");
        let v6 = get_addr("https://ipv6.edjopato.de");

        let (v4, v6) = futures::join!(v4, v6);

        Self { v4, v6 }
    }
}

#[expect(clippy::map_err_ignore)]
async fn get_addr<A: FromStr>(ip_url: &str) -> anyhow::Result<A> {
    let body = http::get(ip_url).await?;
    let addr = body
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("parsing ip address failed: {body}"))?;
    Ok(addr)
}
