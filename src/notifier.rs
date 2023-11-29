use std::fmt::Write;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use frankenstein::{AsyncApi, AsyncTelegramApi, ParseMode, SendMessageParams};

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
        let mut text = "Bot startup done. IPs at startup:\n".to_owned();
        if let Some(ip) = v4 {
            _ = writeln!(&mut text, "IPv4: <code>{ip}</code>");
        }
        if let Some(ip) = v6 {
            _ = writeln!(&mut text, "IPv6: <code>{ip}</code>");
        }
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
        let downtime = format!("IPv4 was down for {}.\n", format_downtime(down_duration));
        print!("{downtime}");

        let mut text = downtime;

        if old != Some(new) {
            println!("IPv4 old: {old:?}");
            println!("IPv4 new:      {new:?}");
            if let Some(ip) = &old {
                _ = write!(&mut text, "<code>{ip}</code>");
            } else {
                text += "None";
            }
            text += " \u{2192} "; // →
            _ = write!(&mut text, "<code>{new}</code>");
        }

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
        let downtime = format!("IPv6 was down for {}.\n", format_downtime(down_duration));
        print!("{downtime}");

        let mut text = downtime;

        if old != Some(new) {
            println!("IPv6 old: {old:?}");
            println!("IPv6 new:      {new:?}");
            if let Some(ip) = &old {
                _ = write!(&mut text, "<code>{ip}</code>");
            } else {
                text += "None";
            }
            text += "\n\u{2193}\n"; // ↓
            _ = write!(&mut text, "<code>{new}</code>");
        }

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
