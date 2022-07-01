use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use crate::http;

#[derive(PartialEq, Eq)]
pub struct IPs {
    pub v4: Option<Ipv4Addr>,
    pub v6: Option<Ipv6Addr>,
}

impl IPs {
    pub async fn get() -> anyhow::Result<Self> {
        let v4 = get_addr("https://ipv4.edjopato.de").await;
        let v6 = get_addr("https://ipv6.edjopato.de").await;

        if let Err(v4) = &v4 {
            if let Err(v6) = &v6 {
                return Err(anyhow::anyhow!("IPv4 Err: {}\nIPv6 Err: {}", v4, v6));
            }
        }

        Ok(Self {
            v4: v4.ok(),
            v6: v6.ok(),
        })
    }
}

async fn get_addr<A: FromStr>(ip_url: &str) -> anyhow::Result<A> {
    let body = http::get(ip_url).await?;
    let addr = body
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("parsing ip address failed: {}", body))?;
    Ok(addr)
}
