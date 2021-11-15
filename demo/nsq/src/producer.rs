use std::thread::sleep;
use std::time::Duration;
use tokio_nsq::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    producer().await;
    Ok(())
}

async fn producer() {
    let topic = NSQTopic::new("names").unwrap();

    let mut producer = NSQProducerConfig::new("127.0.0.1:4150").build();
    let x = producer.consume().await.unwrap();
    println!("{:?}", x);
    // Wait until a connection is initialized
    // assert!(producer.consume().await.unwrap(), NSQEvent::Healthy());
    // Publish a single message
    loop{
        producer.publish(&topic, "你好世界".as_bytes().to_owned()).unwrap();
        println!("{:?}", producer.consume().await.unwrap());
        sleep(Duration::new(5,0));
    }

    // Wait until the message is acknowledged by NSQ
    // assert!(producer.consume().await.unwrap(), NSQEvent::Ok());
}

