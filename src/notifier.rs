use std::fmt::Write;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use frankenstein::{AsyncApi, AsyncTelegramApi, LinkPreviewOptions, ParseMode, SendMessageParams};

const LINK_PREVIEW_OPTIONS: LinkPreviewOptions = LinkPreviewOptions {
    is_disabled: Some(true),
    url: None,
    prefer_small_media: None,
    prefer_large_media: None,
    show_above_text: None,
};

pub struct Notifier {
    bot_token: String,
    target_chat: i64,
}

impl Notifier {
    pub const fn new(bot_token: String, target_chat: i64) -> Self {
        Self {
            bot_token,
            target_chat,
        }
    }

    pub async fn notify_startup(
        &self,
        v4: Option<Ipv4Addr>,
        v6: Option<Ipv6Addr>,
    ) -> Result<(), frankenstein::Error> {
        const STARTUP_PREFIX: &str = concat!(
            "<a href=\"",
            env!("CARGO_PKG_REPOSITORY"),
            "\">",
            env!("CARGO_PKG_NAME"),
            " v",
            env!("CARGO_PKG_VERSION"),
            "</a> startup done. IPs at startup:\n",
        );

        println!("IPv4: {v4:?}");
        println!("IPv6: {v6:?}");
        let mut text = STARTUP_PREFIX.to_owned();
        if let Some(ip) = v4 {
            _ = writeln!(&mut text, "IPv4: <code>{ip}</code>");
        }
        if let Some(ip) = v6 {
            _ = writeln!(&mut text, "IPv6: <code>{ip}</code>");
        }
        AsyncApi::new(&self.bot_token)
            .send_message(
                &SendMessageParams::builder()
                    .disable_notification(true)
                    .link_preview_options(LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::Html)
                    .chat_id(self.target_chat)
                    .text(text)
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

        AsyncApi::new(&self.bot_token)
            .send_message(
                &SendMessageParams::builder()
                    .link_preview_options(LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::Html)
                    .chat_id(self.target_chat)
                    .text(text)
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

        AsyncApi::new(&self.bot_token)
            .send_message(
                &SendMessageParams::builder()
                    .link_preview_options(LINK_PREVIEW_OPTIONS)
                    .parse_mode(ParseMode::Html)
                    .chat_id(self.target_chat)
                    .text(text)
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
