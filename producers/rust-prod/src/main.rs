use std::time::Duration;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main() {
    println!("Hello, world!");

    env_logger::init();

    let broker = "0.0.0.0:9093";
    let topic = "teste-topic";

    let data = "hello Kafka".as_bytes();

    if let Err(e) = produce_message(data, topic,vec![broker.to_owned()]) {
        println!("Failed to produce a message. Err: {}", e);
    }

}

fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>
    ) -> Result<(), KafkaError> {

    println!("Sending messagens to a Kafka broker.");
    println!("Topic: {}   Brokers: {:?}", topic, brokers);

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    for n in 0..100 {
        println!("Sending message with Key: {} Value: {:?}", n, String::from_utf8_lossy(data));
        producer.send(&Record {
            topic,
            partition: -1,
            key: n.to_string(),
            value: data
        })?;
    }

    Ok(())
}
