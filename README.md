# kafka-zookeeper-less
Test of a Kafka Zookeeper-less using docker and implementation of consumer and producer in rust language.

# Run brokers kafka
You must have docker and docker-compose installed in your environment. After that, execute the 
following command:

```bash
    make up

```

Or run broker in daemon mode:

```bash
    make up-daemon

```

# Create Topic 

Use bash scripts "bash" directory for create a topic inside Kafka broker.

```bash
    bash/create-topic.sh <topic name>

```

# Produce and consume message
Inside "rust" directory has two sub-directories with a rust producer (rust-prod) and consumer (rust-con). 

Change directory for each of them to execute a produce or a consume.

To produce:

```rust
    cd rust/rust-prod
    cargo run

```
To consume:

```rust
    cd rust/rust-con
    cargo run

```

