use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Request, Requester},
    types::ParseMode,
    utils::html::code_block,
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
        let mut lines = Vec::new();
        if let Some(ip) = &ips.v4 {
            lines.push(format!("IPv4: {}", ip));
        }
        if let Some(ip) = &ips.v6 {
            lines.push(format!("IPv6: {}", ip));
        }
        let text = lines.join("\n");
        println!("{}", text);

        let text = format!("Bot startup done. IPs at startup:\n{}", code_block(&text));
        self.bot
            .send_message(self.target_chat, &text)
            .disable_notification(true)
            .disable_web_page_preview(true)
            .parse_mode(ParseMode::Html)
            .send()
            .await?;
        Ok(())
    }

    pub async fn notify_change(&self, old: &IPs, new: &IPs) -> anyhow::Result<()> {
        let mut lines = Vec::new();
        if let Some(ip) = &old.v4 {
            lines.push(format!("IPv4 old: {}", ip));
        }
        if let Some(ip) = &new.v4 {
            lines.push(format!("IPv4 new: {}", ip));
        }
        if let Some(ip) = &old.v6 {
            lines.push(format!("IPv6 old: {}", ip));
        }
        if let Some(ip) = &new.v6 {
            lines.push(format!("IPv6 new: {}", ip));
        }
        let text = lines.join("\n");
        println!("Change detected\n{}", text);

        let text = format!("IPs changed\n{}", code_block(&text));
        self.bot
            .send_message(self.target_chat, &text)
            .disable_web_page_preview(true)
            .parse_mode(ParseMode::Html)
            .send()
            .await?;
        Ok(())
    }
}
