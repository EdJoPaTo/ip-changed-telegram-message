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

    let (mut last_known_v4, mut last_known_v6) = {
        let IPs { v4, v6 } = IPs::get().await;
        assert!(
            v4.is_ok() || v6.is_ok(),
            "both IPv4 and IPv6 seem to be down currently"
        );
        (v4.ok(), v6.ok())
    };

    let mut ipv4_error_since: Option<Instant> = None;
    let mut ipv6_error_since: Option<Instant> = None;

    notifier
        .notify_startup(last_known_v4, last_known_v6)
        .await
        .expect("notify startup failed");

    loop {
        sleep(SLEEP_TIME).await;
        let begin_check = Instant::now();

        let IPs { v4, v6 } = IPs::get().await;
        match v4 {
            Ok(v4) => {
                if Some(v4) != last_known_v4 || ipv4_error_since.is_some() {
                    let down_duration = ipv4_error_since.map_or(SLEEP_TIME, |o| o.elapsed());
                    if let Err(err) = notifier
                        .notify_change_v4(last_known_v4, v4, down_duration)
                        .await
                    {
                        eprintln!("notify IPv4 change failed {}", err);
                    } else {
                        last_known_v4 = Some(v4);
                        ipv4_error_since = None;
                    }
                }
            }
            Err(err) => {
                if last_known_v4.is_some() {
                    if ipv4_error_since.is_none() {
                        eprintln!("IPv4 temporary offline\n{}", err);
                        ipv4_error_since = Some(begin_check);
                    } else {
                        eprintln!("IPv4 still offline\n{}", err);
                    }
                }
            }
        }

        match v6 {
            Ok(v6) => {
                if Some(v6) != last_known_v6 || ipv6_error_since.is_some() {
                    let down_duration = ipv6_error_since.map_or(SLEEP_TIME, |o| o.elapsed());
                    if let Err(err) = notifier
                        .notify_change_v6(last_known_v6, v6, down_duration)
                        .await
                    {
                        eprintln!("notify IPv6 change failed {}", err);
                    } else {
                        last_known_v6 = Some(v6);
                        ipv6_error_since = None;
                    }
                }
            }
            Err(err) => {
                if last_known_v6.is_some() {
                    if ipv6_error_since.is_none() {
                        eprintln!("IPv6 temporary offline\n{}", err);
                        ipv6_error_since = Some(begin_check);
                    } else {
                        eprintln!("IPv6 still offline\n{}", err);
                    }
                }
            }
        }
    }
}
