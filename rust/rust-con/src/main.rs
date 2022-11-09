use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

fn main() {
    env_logger::init();

    let broker = "localhost:9093".to_owned();
    let topic = "teste-topic".to_owned();
    let group = "my-group".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed to consume messages: {}", e);
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {

    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            println!("No messages.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {

                println!(
                    "{}:{}@{}: {:?} {:?}",
                    ms.topic(),
                    ms.partition(),
                    m.offset,
                    String::from_utf8_lossy(m.key),
                    String::from_utf8_lossy(m.value)
                );
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}
