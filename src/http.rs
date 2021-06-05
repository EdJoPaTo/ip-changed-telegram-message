use std::time::Duration;

use ureq::{Agent, AgentBuilder};

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

#[derive(Clone)]
pub struct Http {
    agent: Agent,
}

impl Http {
    pub fn new() -> Self {
        let agent = AgentBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent(USER_AGENT)
            .build();
        Self { agent }
    }

    pub fn get(&self, url: &str) -> anyhow::Result<String> {
        let body = self.agent.get(url).call()?.into_string()?;
        Ok(body)
    }
}
