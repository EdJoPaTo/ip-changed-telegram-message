use std::time::{Duration, Instant};

use ips::IPs;
use teloxide::types::ChatId;
use tokio::time::sleep;

mod http;
mod ips;
mod notifier;

const SLEEP_TIME: Duration = Duration::from_secs(20);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    let target_chat = std::env::var("TARGET_CHAT")
        .expect("TARGET_CHAT is not set")
        .parse::<i64>()
        .expect("TARGET_CHAT is not a valid 64-bit integer");
    let target_chat = ChatId(target_chat);
    let notifier = notifier::Notifier::new(&bot_token, target_chat);

    let mut current = IPs::get().await.expect("failed to get current IPs");

    notifier
        .notify_startup(&current)
        .await
        .expect("notify startup failed");

    let mut error_since: Option<Instant> = None;

    loop {
        sleep(SLEEP_TIME).await;
        let begin_check = Instant::now();

        match IPs::get().await {
            Ok(now) => {
                let ip_changed = now != current;
                let network_was_down = error_since.is_some();
                if ip_changed || network_was_down {
                    let network_down_duration = error_since.map(|o| o.elapsed());
                    if let Err(err) = notifier
                        .notify_change(&current, &now, network_down_duration)
                        .await
                    {
                        eprintln!("notify change failed {}", err);
                    } else {
                        current = now;
                        error_since = None;
                    }
                }
            }
            Err(err) => {
                if error_since.is_none() {
                    eprintln!("Temporary offline\n{}", err);
                    error_since = Some(begin_check);
                } else {
                    eprintln!("Still offline\n{}", err);
                }
            }
        }
    }
}
