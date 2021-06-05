use frankenstein::{Api, ChatIdEnum, SendMessageParams, TelegramApi};

use crate::ips::IPs;

pub struct Notifier {
    api: Api,
    target_chat: String,
}

impl Notifier {
    pub fn new(bot_token: String, target_chat: String) -> Self {
        let api = Api::new(bot_token);
        Self { api, target_chat }
    }

    pub fn notify_startup(&self, ips: &IPs) -> anyhow::Result<()> {
        let mut lines = Vec::new();
        if let Some(ip) = &ips.v4 {
            lines.push(format!("IPv4: {}", ip));
        }
        if let Some(ip) = &ips.v6 {
            lines.push(format!("IPv6: {}", ip));
        }
        let text = lines.join("\n");
        println!("{}", text);

        let text = format!("Bot startup done. IPs at startup:\n```\n{}\n```", text);
        let mut params = SendMessageParams::new(
            ChatIdEnum::StringVariant(self.target_chat.to_string()),
            text,
        );
        params.set_disable_notification(Some(true));
        params.set_disable_web_page_preview(Some(true));
        params.set_parse_mode(Some("Markdown".into()));
        if let Err(err) = self.api.send_message(&params) {
            return Err(anyhow::anyhow!("send_message failed {:?}", err));
        }

        Ok(())
    }

    pub fn notify_change(&self, old: &IPs, new: &IPs) -> anyhow::Result<()> {
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
        println!("Change detected\n{}\n", text);

        let text = format!("IPs changed\n```\n{}\n```", text);
        let mut params = SendMessageParams::new(
            ChatIdEnum::StringVariant(self.target_chat.to_string()),
            text,
        );
        params.set_disable_web_page_preview(Some(true));
        params.set_parse_mode(Some("Markdown".into()));
        if let Err(err) = self.api.send_message(&params) {
            return Err(anyhow::anyhow!("send_message failed {:?}", err));
        }

        Ok(())
    }
}
