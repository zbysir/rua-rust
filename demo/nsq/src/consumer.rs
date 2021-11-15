use std::process::exit;
use tokio::sync::{mpsc};
use tokio_nsq::*;

use signal_hook::{iterator::Signals};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    listen().await;
    Ok(())
}

async fn listen() {
    let topic = NSQTopic::new("names").unwrap();
    let channel = NSQChannel::new("first").unwrap();

    let mut addresses = std::collections::HashSet::new();
    addresses.insert("http://127.0.0.1:4161".to_string());

    // 不知道为什么 使用 nsqlookupd 卡死，所以使用直连 nsqd 的方案
    // 直连 nsqd
    let daemons = vec!("127.0.0.1:4150".to_string());
    let mut consumer = NSQConsumerConfig::new(topic, channel)
        .set_max_in_flight(1)
        .set_sources(
            // NSQConsumerConfigSources::Lookup(
            //     NSQConsumerLookupConfig::new().set_addresses(addresses)
            // )
            NSQConsumerConfigSources::Daemons(
                daemons
            )
        )
        .build();

    // handle term signal
    let (mut tx, mut rx) = mpsc::channel::<u32>(1);
    let mut signals = Signals::new(signal_hook::consts::TERM_SIGNALS).unwrap();
    tokio::spawn(async move {
        // https://docs.rs/signal-hook/0.3.10/signal_hook/
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);

            drop(tx);
            return;
        }
    });

    loop {
        tokio::select! {
            message = consumer.consume_filtered() =>{
                let x = message.unwrap();
                let message_body_str = std::str::from_utf8(&x.body).unwrap();
                println!("message body = {}", message_body_str);

                x.finish();
            }
            _ = rx.recv() => {
               exit(1)
            }
        }
    }
}

