FROM openjdk:11.0.16-jre-buster

ENV KAFKA_VERSION=3.3.1
ENV SCALA_VERSION=2.13

WORKDIR /root/

COPY ./files/start-kafka.sh /usr/bin/

RUN wget -P /root/ \
	https://downloads.apache.org/kafka/$KAFKA_VERSION/kafka_$SCALA_VERSION-$KAFKA_VERSION.tgz  \
	&& tar -C /opt/ /opt/kafka_$SCALA_VERSION-$KAFKA_VERSION.tgz \
	&& mv /opt/kafka_$SCALA_VERSION-$KAFKA_VERSION /opt/kafka \
	&& chmod +x /usr/bin/start-kafka.sh

CMD ["start-kafka.sh"]

