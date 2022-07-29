use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Request, Requester},
    types::{ChatId, ParseMode},
    utils::html,
    Bot,
};

pub struct Notifier {
    bot: Bot,
    target_chat: ChatId,
}

impl Notifier {
    pub fn new<C>(bot_token: &str, target_chat: C) -> Self
    where
        C: Into<ChatId>,
    {
        pretty_env_logger::init();
        let bot = Bot::new(bot_token);
        let target_chat = target_chat.into();
        Self { bot, target_chat }
    }

    pub async fn notify_startup(
        &self,
        v4: Option<Ipv4Addr>,
        v6: Option<Ipv6Addr>,
    ) -> anyhow::Result<()> {
        println!("IPv4: {:?}", v4);
        println!("IPv6: {:?}", v6);
        let mut lines = Vec::new();
        if let Some(ip) = v4 {
            lines.push(format!("IPv4: {}", html::code_inline(&ip.to_string())));
        }
        if let Some(ip) = v6 {
            lines.push(format!("IPv6: {}", html::code_inline(&ip.to_string())));
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

    pub async fn notify_change_v4(
        &self,
        old: Option<Ipv4Addr>,
        new: Ipv4Addr,
        down_duration: Duration,
    ) -> anyhow::Result<()> {
        let mut lines = Vec::new();

        let downtime = format!(
            "IPv4 was down for less than {} seconds = {:.1} minutes.",
            down_duration.as_secs(),
            down_duration.as_secs_f32() / 60.0,
        );
        println!("{}", downtime);
        lines.push(downtime);

        if old != Some(new) {
            println!("IPv4 old: {:?}", old);
            println!("IPv4 new:      {:?}", new);
            lines.push(html::bold("IPv4"));
            let mut line = String::new();
            if let Some(ip) = &old {
                line += &html::code_inline(&ip.to_string());
            } else {
                line += "None";
            }
            line += " \u{2192} "; // →
            line += &html::code_inline(&new.to_string());
            lines.push(line);
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

    pub async fn notify_change_v6(
        &self,
        old: Option<Ipv6Addr>,
        new: Ipv6Addr,
        down_duration: Duration,
    ) -> anyhow::Result<()> {
        let mut lines = Vec::new();

        let downtime = format!(
            "IPv6 was down for less than {} seconds = {:.1} minutes.",
            down_duration.as_secs(),
            down_duration.as_secs_f32() / 60.0,
        );
        println!("{}", downtime);
        lines.push(downtime);

        if old != Some(new) {
            println!("IPv6 old: {:?}", old);
            println!("IPv6 new:      {:?}", new);
            lines.push(html::bold("IPv6"));
            if let Some(ip) = &old {
                lines.push(html::code_inline(&ip.to_string()));
            } else {
                lines.push("None".to_string());
            }
            lines.push("\u{2193}".to_string()); // ↓
            lines.push(html::code_inline(&new.to_string()));
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
