use crate::http;

#[derive(PartialEq)]
pub struct IPs {
    pub v4: Option<String>,
    pub v6: Option<String>,
}

impl IPs {
    pub async fn get() -> anyhow::Result<Self> {
        let v4 = http::get("https://ipv4.edjopato.de")
            .await
            .map(|body| body.trim().to_string());
        let v6 = http::get("https://ipv6.edjopato.de")
            .await
            .map(|body| body.trim().to_string());

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
