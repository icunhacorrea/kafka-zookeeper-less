#!/bin/bash

docker exec kafka-zookeeper-less-kafka1-1 \
	/opt/kafka/bin/kafka-topics.sh \
	--create \
	--replication-factor 1 \
	--partitions 1 \
	--topic teste-topic \
	--bootstrap-server 0.0.0.0:9092

