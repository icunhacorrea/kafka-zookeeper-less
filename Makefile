build:
	docker build -t icorrea:kafka-zookeeper-less docker/

clean:
	docker system prune

up:
	docker-compose up
