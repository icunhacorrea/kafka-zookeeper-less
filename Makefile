clean:
	docker system prune

up:
	docker-compose up --build

up-daemon:
	docker-compose up --build -d

stop:
	docker-compose stop 
