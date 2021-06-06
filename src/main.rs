use std::time::Duration;

use http::Http;
use ips::IPs;
use tokio::time::sleep;

mod http;
mod ips;
mod notifier;

#[tokio::main]
async fn main() {
    let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    let target_chat = std::env::var("TARGET_CHAT")
        .expect("TARGET_CHAT is not set")
        .parse::<i64>()
        .expect("TARGET_CHAT is not a valid 64-bit integer");
    let notifier = notifier::Notifier::new(&bot_token, target_chat);

    let http = Http::new();
    let mut current = IPs::get(&http).await.expect("failed to get current IPs");

    notifier
        .notify_startup(&current)
        .await
        .expect("notify startup failed");

    loop {
        sleep(Duration::from_secs(20)).await;

        match IPs::get(&http).await {
            Ok(now) => {
                if now != current {
                    if let Err(err) = notifier.notify_change(&current, &now).await {
                        eprintln!("notify change failed {}", err);
                    }
                    current = now;
                }
            }
            Err(err) => eprintln!("Temporary offline {}", err),
        }
    }
}
