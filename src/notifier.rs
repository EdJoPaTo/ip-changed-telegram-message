use std::time::Duration;

use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Request, Requester},
    types::ParseMode,
    utils::html,
    Bot,
};

use crate::ips::IPs;

pub struct Notifier {
    bot: Bot,
    target_chat: i64,
}

impl Notifier {
    pub fn new(bot_token: &str, target_chat: i64) -> Self {
        teloxide::enable_logging!();
        let bot = Bot::new(bot_token);
        Self { bot, target_chat }
    }

    pub async fn notify_startup(&self, ips: &IPs) -> anyhow::Result<()> {
        println!("IPv4: {:?}", ips.v4);
        println!("IPv6: {:?}", ips.v6);
        let mut lines = Vec::new();
        if let Some(ip) = &ips.v4 {
            lines.push(format!("IPv4: {}", html::code_inline(ip)));
        }
        if let Some(ip) = &ips.v6 {
            lines.push(format!("IPv6: {}", html::code_inline(ip)));
        }
        let text = lines.join("\n");
        let text = format!("Bot startup done. IPs at startup:\n{}", text);
        self.bot
            .send_message(self.target_chat, &text)
            .disable_notification(true)
            .disable_web_page_preview(true)
            .parse_mode(ParseMode::Html)
            .send()
            .await?;
        Ok(())
    }

    pub async fn notify_change(
        &self,
        old: &IPs,
        new: &IPs,
        down_duration: Option<Duration>,
    ) -> anyhow::Result<()> {
        let mut lines = Vec::new();

        if let Some(down_duration) = down_duration {
            let downtime = format!(
                "Network was down for up to {} seconds = {:.1} minutes.",
                down_duration.as_secs(),
                down_duration.as_secs_f32() / 60.0,
            );
            println!("{}", downtime);
            lines.push(downtime);
        }

        if old.v4 != new.v4 {
            println!("IPv4 old: {:?}", old.v4);
            println!("IPv4 new: {:?}", new.v4);
            lines.push(html::bold("IPv4"));
            let mut line = String::new();
            if let Some(ip) = &old.v4 {
                line += &html::code_inline(&ip);
            } else {
                line += "None";
            }
            line += " \u{2192} "; // →
            if let Some(ip) = &new.v4 {
                line += &html::code_inline(&ip);
            } else {
                line += "None";
            }
            lines.push(line);
        }
        if old.v6 != new.v6 {
            println!("IPv6 old: {:?}", old.v6);
            println!("IPv6 new: {:?}", new.v6);
            lines.push(html::bold("IPv6"));
            if let Some(ip) = &old.v6 {
                lines.push(html::code_inline(&ip));
            } else {
                lines.push("None".to_string())
            }
            lines.push("\u{2193}".to_string()); // ↓
            if let Some(ip) = &new.v6 {
                lines.push(html::code_inline(&ip));
            } else {
                lines.push("None".to_string())
            }
        }

        let text = lines.join("\n");
        self.bot
            .send_message(self.target_chat, &text)
            .disable_web_page_preview(true)
            .parse_mode(ParseMode::Html)
            .send()
            .await?;
        Ok(())
    }
}
