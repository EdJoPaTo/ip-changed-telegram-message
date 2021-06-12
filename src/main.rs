use std::time::{Duration, Instant};

use http::Http;
use ips::IPs;
use tokio::time::sleep;

mod http;
mod ips;
mod notifier;

const SLEEP_TIME: Duration = Duration::from_secs(20);

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

    let mut error_since: Option<Instant> = None;

    loop {
        sleep(SLEEP_TIME).await;
        let begin_check = Instant::now();

        match IPs::get(&http).await {
            Ok(now) => {
                let ip_changed = now != current;
                let network_down_duration = error_since.map(|o| o.elapsed());
                let network_was_down = network_down_duration.is_some();

                if ip_changed || network_was_down {
                    if let Err(err) = notifier
                        .notify_change(&current, &now, network_down_duration)
                        .await
                    {
                        eprintln!("notify change failed {}", err);
                    }
                    current = now;
                }

                error_since = None;
            }
            Err(err) => {
                eprintln!("Temporary offline {}", err);
                error_since = Some(begin_check);
            }
        }
    }
}
