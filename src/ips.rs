use crate::http::Http;

#[derive(PartialEq)]
pub struct IPs {
    pub v4: Option<String>,
    pub v6: Option<String>,
}

impl IPs {
    pub async fn get(http: &Http) -> anyhow::Result<Self> {
        let v4 = http
            .get("https://ipv4.edjopato.de")
            .await
            .map(|body| body.trim().to_string());
        let v6 = http
            .get("https://ipv6.edjopato.de")
            .await
            .map(|body| body.trim().to_string());

        if v4.is_err() && v6.is_err() {
            Err(anyhow::anyhow!("{:?} {:?}", v4, v6))
        } else {
            Ok(Self {
                v4: v4.ok(),
                v6: v6.ok(),
            })
        }
    }
}
