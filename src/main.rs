use std::{thread::sleep, time::Duration};

use http::Http;
use ips::IPs;

mod http;
mod ips;
mod notifier;

fn main() {
    let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    let target_chat = std::env::var("TARGET_CHAT").expect("TARGET_CHAT is not set");
    let notifier = notifier::Notifier::new(bot_token, target_chat);

    let http = Http::new();
    let mut current = IPs::get(&http).expect("failed to get current IPs");

    notifier
        .notify_startup(&current)
        .expect("notify startup failed");

    loop {
        sleep(Duration::from_secs(20));

        match IPs::get(&http) {
            Ok(now) => {
                if now != current {
                    if let Err(err) = notifier.notify_change(&current, &now) {
                        eprintln!("notify change failed {}", err);
                    }
                    current = now;
                }
            }
            Err(err) => eprintln!("Temporary offline {}", err),
        }
    }
}
