use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use frankenstein::{AsyncApi, AsyncTelegramApi, ParseMode, SendMessageParams};

fn code_inline<S: ToString>(s: &S) -> String {
    format!("<code>{}</code>", s.to_string())
}

pub struct Notifier {
    bot: AsyncApi,
    target_chat: i64,
}

impl Notifier {
    pub fn new(bot_token: &str, target_chat: i64) -> Self {
        let bot = AsyncApi::new(bot_token);
        Self { bot, target_chat }
    }

    pub async fn notify_startup(
        &self,
        v4: Option<Ipv4Addr>,
        v6: Option<Ipv6Addr>,
    ) -> Result<(), frankenstein::Error> {
        println!("IPv4: {v4:?}");
        println!("IPv6: {v6:?}");
        let mut lines = Vec::new();
        if let Some(ip) = v4 {
            lines.push(format!("IPv4: {}", code_inline(&ip)));
        }
        if let Some(ip) = v6 {
            lines.push(format!("IPv6: {}", code_inline(&ip)));
        }
        let text = lines.join("\n");
        let text = format!("Bot startup done. IPs at startup:\n{text}");
        self.bot
            .send_message(
                &SendMessageParams::builder()
                    .chat_id(self.target_chat)
                    .text(text)
                    .disable_notification(true)
                    .disable_web_page_preview(true)
                    .parse_mode(ParseMode::Html)
                    .build(),
            )
            .await?;
        Ok(())
    }

    pub async fn notify_change_v4(
        &self,
        old: Option<Ipv4Addr>,
        new: Ipv4Addr,
        down_duration: Duration,
    ) -> Result<(), frankenstein::Error> {
        let mut lines = Vec::new();

        let downtime = format!("IPv4 was down for {}.", format_downtime(down_duration));
        println!("{downtime}");
        lines.push(downtime);

        if old != Some(new) {
            println!("IPv4 old: {old:?}");
            println!("IPv4 new:      {new:?}");
            lines.push("<b>IPv4</b>".to_string());
            let mut line = String::new();
            if let Some(ip) = &old {
                line += &code_inline(&ip);
            } else {
                line += "None";
            }
            line += " \u{2192} "; // →
            line += &code_inline(&new);
            lines.push(line);
        }

        let text = lines.join("\n");
        self.bot
            .send_message(
                &SendMessageParams::builder()
                    .chat_id(self.target_chat)
                    .text(text)
                    .disable_web_page_preview(true)
                    .parse_mode(ParseMode::Html)
                    .build(),
            )
            .await?;
        Ok(())
    }

    pub async fn notify_change_v6(
        &self,
        old: Option<Ipv6Addr>,
        new: Ipv6Addr,
        down_duration: Duration,
    ) -> Result<(), frankenstein::Error> {
        let mut lines = Vec::new();

        let downtime = format!("IPv6 was down for {}.", format_downtime(down_duration));
        println!("{downtime}");
        lines.push(downtime);

        if old != Some(new) {
            println!("IPv6 old: {old:?}");
            println!("IPv6 new:      {new:?}");
            lines.push("<b>IPv6</b>".to_string());
            if let Some(ip) = &old {
                lines.push(code_inline(&ip));
            } else {
                lines.push("None".to_string());
            }
            lines.push("\u{2193}".to_string()); // ↓
            lines.push(code_inline(&new));
        }

        let text = lines.join("\n");
        self.bot
            .send_message(
                &SendMessageParams::builder()
                    .chat_id(self.target_chat)
                    .text(text)
                    .disable_web_page_preview(true)
                    .parse_mode(ParseMode::Html)
                    .build(),
            )
            .await?;
        Ok(())
    }
}

fn format_downtime(down_duration: Duration) -> String {
    let secs = down_duration.as_secs();
    if secs <= 99 {
        return format!("less than {secs} seconds");
    }

    let minutes = down_duration.as_secs_f32() / 60.0;
    if minutes <= 99.0 {
        return format!("{minutes:.1} minutes");
    }

    let hours = minutes / 60.0;
    format!("{hours:.1} hours")
}
